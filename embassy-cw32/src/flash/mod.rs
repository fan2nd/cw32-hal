//! Blocking internal NOR flash for an explicitly reserved data partition.
//!
//! Establishing the partition is unsafe: the FLASH peripheral token cannot
//! prove that the linker, Rust references, DMA, or exception handlers do not use
//! its contents. See [`Flash::new_blocking`] and `docs/flash.md`.
//!
//! Operations mask normal interrupts and may stall all flash instruction fetch.
//! There is no operation timeout: once launched, an operation must complete
//! before the driver can restore read mode, locks, cache, and interrupts.
//! A batch that blocks GTIM1's IRQ for 65.536 ms can lose time-driver wraps;
//! callers must meet that service budget and any motor/control deadlines.

use core::ops::Range;
use core::sync::atomic::{compiler_fence, Ordering};

use embedded_storage::nor_flash::{ErrorType, NorFlashError, NorFlashErrorKind, ReadNorFlash};

use crate::{pac, peripherals::FLASH, rcc::PeripheralClock, Peri};

include!(concat!(env!("OUT_DIR"), "/_generated_flash.rs"));
/// Minimum read granularity, in bytes.
pub const READ_SIZE: usize = 1;
/// Minimum program granularity, in bytes. Each byte must be erased first.
pub const WRITE_SIZE: usize = 1;
/// Erase page size, in bytes.
pub const ERASE_SIZE: usize = 512;

/// Flash operation error. A write/erase error may leave a partial result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Range is reversed, empty at construction, overflowing, or out of bounds.
    OutOfBounds,
    /// Partition or erase endpoints are not 512-byte aligned.
    Unaligned,
    /// Range includes security controls or a configured secure-library region.
    Protected,
    /// An earlier operation is still active. No new operation was launched.
    Busy,
    /// Controller was not in read mode, or configuration readback failed.
    Configuration,
    /// At least one destination byte was not 0xff before programming.
    NotErased,
    /// Hardware rejected programming because the destination was not erased.
    Program,
    /// Hardware rejected a write or erase to a locked page group.
    Locked,
    /// Hardware rejected a write or erase to the currently executing page.
    ProgramCounter,
    /// Hardware rejected an access to the secure-library region (L012 only).
    SecureLibrary,
    /// Hardware detected cache/prefetch enabled during a mutation (L012 only).
    CacheEnabled,
    /// Completed operation did not read back the requested contents.
    Verify,
}

impl NorFlashError for Error {
    fn kind(&self) -> NorFlashErrorKind {
        match self {
            Self::OutOfBounds => NorFlashErrorKind::OutOfBounds,
            Self::Unaligned => NorFlashErrorKind::NotAligned,
            _ => NorFlashErrorKind::Other,
        }
    }
}

/// Exclusive controller owner with a fixed, non-executable data partition.
///
/// All method and read-storage offsets are relative to [`Self::base_address`].
/// Dropping or forgetting this value does not restore its previous flash data.
pub struct Flash<'d> {
    _peripheral: Peri<'d, FLASH>,
    clock: crate::rcc::ClockGuard,
    start: u32,
    end: u32,
}

impl<'d> Flash<'d> {
    /// Acquire a nonempty, erase-aligned absolute address range in main flash.
    ///
    /// L012 always excludes the last erase page, containing security control
    /// words, and every page in the configured secure-library region. F030 has
    /// no such control words in main flash. OTP/boot ROM/option registers are
    /// outside the accepted main-flash address range on both devices.
    /// No flash-controller reset or erase is performed by this constructor.
    ///
    /// # Safety
    ///
    /// The caller must establish and maintain all of the following:
    ///
    /// - Every byte of `region` is reserved outside Rust allocations and the
    ///   firmware's code, vector table, constants, literal pools, unwind/return
    ///   paths, and any other live data. No Rust reference, including a static
    ///   reference or DMA buffer, may refer to it for the driver's lifetime.
    ///   This must be enforced in the final linker layout, including load images
    ///   copied to RAM at startup. The reservation must remain valid if this
    ///   driver is forgotten; erasing data does not make it valid code on drop.
    /// - No other code accesses the partition or changes FLASH configuration,
    ///   clocks, remapping, or protection while this driver exists. This includes
    ///   raw addresses, other drivers, interrupt handlers, and other bus masters.
    /// - During a write/erase, all DMA and other non-CPU bus masters must avoid
    ///   **all** flash reads, including reads outside this partition. The driver
    ///   masks ordinary interrupts, but cannot stop DMA or mask NMI/HardFault.
    ///   Any unmasked exception path must obey the same exclusion; if it must
    ///   run while flash is busy, its vectors, code, literals and data must all
    ///   be in SRAM and must not access flash or this controller.
    /// - The selected critical-section implementation excludes every normal
    ///   interrupt/task that could interfere. Supply voltage, clock settings,
    ///   watchdog behavior and the application's interrupt latency constraints
    ///   must permit a complete erase/program with flash instruction stalls.
    ///
    /// These are whole-system obligations, not properties proved by `Peri`.
    /// A RAM function annotation alone does not establish them. This API does
    /// not authorize replacing running firmware or its vectors.
    pub unsafe fn new_blocking(
        peripheral: Peri<'d, FLASH>,
        region: Range<u32>,
    ) -> Result<Self, Error> {
        if region.start >= region.end || region.end > FLASH_SIZE as u32 {
            return Err(Error::OutOfBounds);
        }
        if region.start % ERASE_SIZE as u32 != 0 || region.end % ERASE_SIZE as u32 != 0 {
            return Err(Error::Unaligned);
        }
        let mut clock = FLASH::acquire_no_reset();
        let result = critical_section::with(|_| {
            if busy() {
                // Do not gate a controller inherited in an active state.
                clock.pin();
                return Err(Error::Busy);
            }
            if pac::FLASH.cr1().read().mode() != 0 {
                return Err(Error::Configuration);
            }
            #[cfg(flash_l012)]
            {
                if pac::FLASH.cr2().read().cacheinvalid() {
                    return Err(Error::Configuration);
                }
                // RM 7.6.4: 0xfff0..0xffff contains security activation words.
                // Reserve its entire erase page, even when SDK is disabled.
                let sdk = pac::FLASH.sdkcfr().read();
                let limit = if sdk.start() <= sdk.end() {
                    u32::from(sdk.start()) * ERASE_SIZE as u32
                } else {
                    FLASH_SIZE as u32 - ERASE_SIZE as u32
                };
                if region.end > limit.min(FLASH_SIZE as u32 - ERASE_SIZE as u32) {
                    return Err(Error::Protected);
                }
            }
            Ok(())
        });
        result?;
        Ok(Self {
            _peripheral: peripheral,
            clock,
            start: region.start,
            end: region.end,
        })
    }

    /// Absolute start address of the reserved partition.
    pub fn base_address(&self) -> u32 {
        self.start
    }

    /// Partition size in bytes, not the size of the entire flash device.
    pub fn capacity(&self) -> usize {
        (self.end - self.start) as usize
    }

    fn address(&self, offset: u32, len: usize) -> Result<u32, Error> {
        let len = u32::try_from(len).map_err(|_| Error::OutOfBounds)?;
        let end = offset.checked_add(len).ok_or(Error::OutOfBounds)?;
        if end > self.end - self.start {
            return Err(Error::OutOfBounds);
        }
        Ok(self.start + offset)
    }

    fn idle(&mut self) -> Result<(), Error> {
        if busy() {
            // Under the constructor contract this cannot be our operation:
            // every launched operation is awaited before releasing &mut self.
            self.clock.pin();
            return Err(Error::Busy);
        }
        if pac::FLASH.cr1().read().mode() != 0 {
            return Err(Error::Configuration);
        }
        #[cfg(flash_l012)]
        if pac::FLASH.cr2().read().cacheinvalid() {
            return Err(Error::Configuration);
        }
        Ok(())
    }

    /// Read bytes at a partition-relative offset. No alignment is required.
    /// An empty read at exactly `capacity()` succeeds without hardware access.
    pub fn blocking_read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Error> {
        let address = self.address(offset, bytes.len())?;
        if bytes.is_empty() {
            return Ok(());
        }
        self.idle()?;
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = read_byte(address + i as u32);
        }
        Ok(())
    }

    /// Program erased bytes at a partition-relative offset.
    ///
    /// No alignment is required. Every destination byte must be 0xff; a whole
    /// request precheck rejects non-erased bytes before programming any byte.
    /// A hardware error may still leave a partially programmed request.
    /// Completed bytes are read back with cache/prefetch disabled. Multiwrite
    /// programming (even only further 1-to-0 transitions) is not supported.
    /// Normal interrupts remain masked for the entire nonempty request.
    pub fn blocking_write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Error> {
        let address = self.address(offset, bytes.len())?;
        if bytes.is_empty() {
            return Ok(());
        }
        critical_section::with(|_| {
            self.idle()?;
            let state = Operation::begin()?;
            let result = (|| {
                for i in 0..bytes.len() {
                    if read_byte(address + i as u32) != 0xff {
                        return Err(Error::NotErased);
                    }
                }
                set_mode(1);
                if pac::FLASH.cr1().read().mode() != 1 {
                    return Err(Error::Configuration);
                }
                for (i, &byte) in bytes.iter().enumerate() {
                    let current = address + i as u32;
                    state.unlock(current)?;
                    // Byte programming makes every offset naturally aligned.
                    // The unsafe constructor establishes memory ownership.
                    unsafe { (current as *mut u8).write_volatile(byte) };
                    wait_complete();
                    check_errors()?;
                    if read_byte(current) != byte {
                        return Err(Error::Verify);
                    }
                }
                Ok(())
            })();
            state.finish().and(result)
        })
    }

    /// Erase `[from, to)` using partition-relative, 512-byte-aligned offsets.
    ///
    /// Reversed/out-of-bounds/unaligned ranges cause no hardware writes.
    /// An aligned empty range succeeds, including at `capacity()`.
    /// Every completed page is checked for 0xff with cache/prefetch disabled.
    /// Normal interrupts remain masked for the entire nonempty request.
    pub fn blocking_erase(&mut self, from: u32, to: u32) -> Result<(), Error> {
        let len = to.checked_sub(from).ok_or(Error::OutOfBounds)?;
        let address = self.address(from, len as usize)?;
        if from % ERASE_SIZE as u32 != 0 || to % ERASE_SIZE as u32 != 0 {
            return Err(Error::Unaligned);
        }
        if len == 0 {
            return Ok(());
        }
        critical_section::with(|_| {
            self.idle()?;
            let state = Operation::begin()?;
            let result = (|| {
                set_mode(2);
                if pac::FLASH.cr1().read().mode() != 2 {
                    return Err(Error::Configuration);
                }
                for page in (address..address + len).step_by(ERASE_SIZE) {
                    state.unlock(page)?;
                    // Any byte write within the page triggers page erase.
                    unsafe { (page as *mut u8).write_volatile(0) };
                    wait_complete();
                    check_errors()?;
                    for i in 0..ERASE_SIZE as u32 {
                        if read_byte(page + i) != 0xff {
                            return Err(Error::Verify);
                        }
                    }
                }
                Ok(())
            })();
            state.finish().and(result)
        })
    }
}

fn busy() -> bool {
    #[cfg(flash_l012)]
    return pac::FLASH.isr().read().busy();
    #[cfg(flash_f030)]
    return pac::FLASH.cr1().read().busy();
}

fn wait_complete() {
    // No documented abort exists. Returning early would release exclusion and
    // possibly the clock while the controller is still writing. Flash-resident
    // code can stall before even reaching this loop (RM L012 7.8 / F030 7.7).
    while busy() {
        core::hint::spin_loop();
    }
    compiler_fence(Ordering::SeqCst);
}

fn read_byte(address: u32) -> u8 {
    // Reserved hardware memory, outside Rust allocations; unlike a slice,
    // volatile access may represent hardware address zero. No reference is made.
    unsafe { (address as *const u8).read_volatile() }
}

fn set_mode(mode: u8) {
    let mut cr1 = pac::FLASH.cr1().read();
    cr1.set_key(0x5a5a);
    cr1.set_mode(mode);
    pac::FLASH.cr1().write_value(cr1);
}

fn check_errors() -> Result<(), Error> {
    let status = pac::FLASH.isr().read();
    if status.prog() {
        return Err(Error::Program);
    }
    if status.pagelock() {
        return Err(Error::Locked);
    }
    if status.pc() {
        return Err(Error::ProgramCounter);
    }
    #[cfg(flash_l012)]
    {
        if status.sdkerr() {
            return Err(Error::SecureLibrary);
        }
        if status.cacheon() {
            return Err(Error::CacheEnabled);
        }
    }
    Ok(())
}

fn clear_errors() {
    let mut clear = pac::FLASH.icr().read();
    clear.set_prog(false);
    clear.set_pagelock(false);
    clear.set_pc(false);
    #[cfg(flash_l012)]
    {
        clear.set_cacheon(false);
        clear.set_sdkerr(false);
    }
    // Zero-to-clear implemented fields; preserve reserved bits read back.
    pac::FLASH.icr().write_value(clear);
}

// One scoped hardware operation. Only constructed inside a critical section
// after idle() proves no outstanding write. Every launch is synchronously
// awaited; none of the code between launch and completion can panic or return.
struct Operation {
    cr2: pac::flash::regs::Cr2,
    locks: pac::flash::regs::Pagelock,
    ier: pac::flash::regs::Ier,
}

impl Operation {
    fn begin() -> Result<Self, Error> {
        let state = Self {
            cr2: pac::FLASH.cr2().read(),
            locks: pac::FLASH.pagelock().read(),
            ier: pac::FLASH.ier().read(),
        };
        compiler_fence(Ordering::SeqCst);
        let mut ier = state.ier;
        ier.set_prog(false);
        ier.set_pagelock(false);
        ier.set_pc(false);
        #[cfg(flash_l012)]
        {
            ier.set_cacheon(false);
            ier.set_sdkerr(false);
        }
        pac::FLASH.ier().write_value(ier);
        let mut cr2 = state.cr2;
        cr2.set_key(0x5a5a);
        cr2.set_cache(false);
        cr2.set_fetch(false);
        #[cfg(flash_l012)]
        {
            // Explicit software pulse, not a self-clearing command.
            cr2.set_cacheinvalid(true);
            pac::FLASH.cr2().write_value(cr2);
            cr2.set_cacheinvalid(false);
        }
        pac::FLASH.cr2().write_value(cr2);
        let actual = pac::FLASH.cr2().read();
        let configured = !actual.cache() && !actual.fetch() && actual.wait() == cr2.wait();
        #[cfg(flash_l012)]
        let configured = configured && !actual.cacheinvalid();
        if !configured {
            state.finish()?;
            return Err(Error::Configuration);
        }
        clear_errors();
        if let Err(error) = check_errors() {
            state.finish()?;
            return Err(error);
        }
        Ok(state)
    }

    fn unlock(&self, address: u32) -> Result<(), Error> {
        let group = (address as usize) / (8 * ERASE_SIZE);
        let mut locks = self.locks;
        locks.set_key(0x5a5a);
        locks.set_lock(group, true);
        pac::FLASH.pagelock().write_value(locks);
        if !pac::FLASH.pagelock().read().lock(group) {
            return Err(Error::Locked);
        }
        Ok(())
    }

    fn finish(self) -> Result<(), Error> {
        // All launch sites waited until idle, including hardware-error paths.
        // Restore only mode, preserving F030 STANDBY and read-only SECURITY.
        set_mode(0);
        let mut locks = self.locks;
        locks.set_key(0x5a5a);
        pac::FLASH.pagelock().write_value(locks);
        let mut cr2 = self.cr2;
        cr2.set_key(0x5a5a);
        #[cfg(flash_l012)]
        {
            cr2.set_cacheinvalid(true);
            pac::FLASH.cr2().write_value(cr2);
            cr2.set_cacheinvalid(false);
        }
        pac::FLASH.cr2().write_value(cr2);
        // Captured errors belong to this polled operation. Clear them before
        // restoring any previously enabled FLASH interrupt sources; do not
        // clear the NVIC line shared with the independent RAM controller.
        clear_errors();
        pac::FLASH.ier().write_value(self.ier);
        compiler_fence(Ordering::SeqCst);
        let cr2_mask = if cfg!(flash_l012) { 0x3f } else { 0x1f };
        if pac::FLASH.cr1().read().mode() != 0
            || pac::FLASH.pagelock().read().bits() & 0xffff != self.locks.bits() & 0xffff
            || pac::FLASH.cr2().read().bits() & cr2_mask != self.cr2.bits() & cr2_mask
        {
            return Err(Error::Configuration);
        }
        Ok(())
    }
}

impl ErrorType for Flash<'_> {
    type Error = Error;
}

impl ReadNorFlash for Flash<'_> {
    const READ_SIZE: usize = READ_SIZE;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Error> {
        self.blocking_read(offset, bytes)
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }
}
