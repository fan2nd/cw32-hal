//! Blocking NOR storage in an exclusively owned, linker-reserved FLASH region.
//!
//! Only reviewed exact parts from all thirteen CW32 families
//! are qualified. Generic profiles lack memory metadata. Package-neutral
//! compatibility aliases are not qualified for this storage API, even when
//! their memory sizes are known. Both reject region construction. A
//! FLASH singleton is not ownership of the array: [`ReservedRegion`] requires
//! an unsafe, explicit code/data exclusion contract. Offsets are relative to
//! that reserved region, not the whole device. No array references are created.
//!
//! Program/erase stalls instruction fetch from FLASH, including interrupts and
//! a FLASH-resident executor. There is no completion interrupt or documented
//! abort. Once triggered, this driver waits without a timeout until BUSY clears;
//! a stuck controller can block forever. Typical datasheet times are not upper
//! bounds. Validate watchdog/interrupt latency and electrical conditions on the
//! actual board. Operations are not transactional or power-fail safe.
//!
//! Every programmed byte must first be 0xff. There is deliberately no
//! `NorFlash` or `MultiwriteNorFlash`, automatic erase, chip erase, option/security access,
//! interrupt binding, DMA, or async API. The configuration clock remains enabled
//! and WAIT/STANDBY are preserved; neither construction nor Drop resets FLASH.
//!
//! Only `ReadNorFlash` is implemented. `NorFlash::write` promises that power loss
//! leaves the rest of the page unchanged; the audited CW32 sources do not prove
//! that property. Explicit blocking write/erase methods make no such promise.

pub use crate::mode::Blocking;
use crate::{Peri, peripherals::FLASH};
use core::{marker::PhantomData, ops::Range};
use embedded_storage::nor_flash::{ErrorType, NorFlashError, NorFlashErrorKind, ReadNorFlash};

use crate::pac::{self, flash::vals::Mode};
use crate::rcc::{Readback, SealedRccPeripheral};

/// Minimum read granularity in bytes.
pub const READ_SIZE: usize = 1;
/// Minimum program granularity in bytes; all addressed bytes must be erased.
pub const WRITE_SIZE: usize = 1;
/// Page erase granularity in bytes.
pub const ERASE_SIZE: usize = 512;
/// Main-array size qualified for this storage API, from exact-part metadata.
/// `None` for generic profiles without memory metadata and package-neutral
/// compatibility aliases not qualified for this API despite known memory sizes.
pub const FLASH_SIZE: Option<u32> = crate::FLASH_STORAGE_SIZE;

/// FLASH errors. Write/erase errors may follow partially completed operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// The selected profile has no capacity qualified for this storage API.
    /// Generic profiles lack memory metadata; package-neutral aliases are not
    /// qualified for this API even when their memory sizes are known.
    UnknownCapacity,
    /// Arithmetic overflow, reversed range, or range outside the reserved area.
    OutOfBounds,
    /// Region or erase endpoints are not 512-byte aligned.
    NotAligned,
    /// A reservation must contain at least one page.
    EmptyRegion,
    /// Board bounds violate this family's qualified electrical conditions.
    InvalidConditions,
    /// FLASH is held in reset; this driver never changes the reset register.
    HeldInReset,
    /// The FLASH configuration clock failed to read back as enabled.
    ClockNotEnabled,
    /// Existing WAIT cannot support the caller's actual worst-case HCLK bound.
    InvalidLatency,
    /// A pre-existing operation is still busy; no operation was started here.
    Busy,
    /// FLASH error interrupts are enabled. The combined RAM IRQ is not owned.
    InterruptsEnabled,
    /// Requested bytes include a value other than 0xff; nothing was programmed.
    NotErased,
    /// Reservation overlaps the SLIB descriptor page or an active protected library.
    ReservedProtection,
    /// The required temporary page-group unlock failed to read back.
    Protected,
    /// A keyed operation MODE write failed to read back.
    Configuration,
    /// Idle-state cleanup failed. The controller must be inspected before reuse;
    /// this error takes precedence over the operation error without losing flags.
    Cleanup {
        /// All error flags captured after the operation completed.
        hardware_errors: ErrorFlags,
        /// Whether MODE read back as Read.
        mode_restored: bool,
        /// Whether the saved page-group lock bits read back correctly.
        locks_restored: bool,
        /// Whether the saved FETCH/CACHE state and WAIT read back correctly.
        cache_restored: bool,
    },
    /// Hardware captured one or more FLASH errors (all implemented bits retained).
    Hardware(ErrorFlags),
    /// Completed memory readback disagrees with the requested write/erase.
    Verify,
}
impl NorFlashError for Error {
    fn kind(&self) -> NorFlashErrorKind {
        match self {
            Self::OutOfBounds => NorFlashErrorKind::OutOfBounds,
            Self::NotAligned => NorFlashErrorKind::NotAligned,
            _ => NorFlashErrorKind::Other,
        }
    }
}

/// Snapshot of all implemented FLASH errors; never includes RAM flags.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ErrorFlags(u8);
impl ErrorFlags {
    /// Attempted erase/program of the page containing the executing instruction.
    pub const fn pc(self) -> bool {
        self.0 & 1 != 0
    }
    /// Target page group was write-protected.
    pub const fn page_lock(self) -> bool {
        self.0 & 2 != 0
    }
    /// Programming non-erased bytes or another documented program error.
    pub const fn program(self) -> bool {
        self.0 & 16 != 0
    }
    /// Attempted modification of a secure library (L011/L012 only).
    pub const fn secure_library(self) -> bool {
        self.0 & 4 != 0
    }
    /// Mutation attempted with FETCH/CACHE enabled (L012 only).
    pub const fn cache_enabled(self) -> bool {
        self.0 & 8 != 0
    }
    /// Raw PC[0], PAGELOCK[1], SDKERR[2], CACHEON[3], PROG[4] snapshot.
    /// Unsupported bits are always zero.
    pub const fn bits(self) -> u8 {
        self.0
    }
}

/// Exclusive ownership of a main-FLASH reservation. Not Clone or Copy.
/// No reference to the underlying array is formed, including address zero.
pub struct ReservedRegion<'d> {
    range: Range<u32>,
    _exclusive: PhantomData<&'d mut [u8]>,
}
impl<'d> ReservedRegion<'d> {
    /// Claim a page-aligned half-open main-array address range.
    ///
    /// # Safety
    /// The linker and application must exclusively reserve every byte in this
    /// range for this owner for `'d`. It must not contain code, vectors, literals,
    /// live immutable data, allocator storage, or any object/reference used by
    /// other code. No overlapping storage owner, DMA reader/writer, debugger
    /// write, interrupt, or other FLASH programming agent may access it while
    /// owned. The caller must keep that exclusion even across partial errors.
    /// Merely choosing the last page or possessing `Peri<FLASH>` is insufficient.
    /// L010/L011/L012 always exclude the final descriptor erase page; the driver
    /// additionally rejects any active SDKCFR protected-library overlap.
    pub unsafe fn new(range: Range<u32>) -> Result<Self, Error> {
        validate_region(&range, FLASH_SIZE)?;
        Ok(Self {
            range,
            _exclusive: PhantomData,
        })
    }
    /// Absolute byte address of the first reserved byte.
    pub fn start(&self) -> u32 {
        self.range.start
    }
    /// Reserved length in bytes.
    pub fn len(&self) -> u32 {
        self.range.end - self.range.start
    }
    /// Always false: empty reservations cannot be constructed.
    pub fn is_empty(&self) -> bool {
        false
    }
}

/// A caller-validated, sustained board operating envelope. No default exists.
/// Values are bounds on actual supply/clock, not nominal RCC estimates.
pub struct OperatingConditions {
    max_hclk_hz: u32,
}
impl OperatingConditions {
    /// Validate and assert actual supply and HCLK bounds throughout storage use.
    /// L010 permits 1.62–5.5 V, L011/L012 1.7–5.5 V, and other non-radio
    /// families 1.65–5.5 V. All permit at most 24 MHz below 1.8 V.
    /// At >=1.8 V, L011/L012 permit 96 MHz, F030/A030/L083 64 MHz,
    /// and others 48 MHz. R031 requires 2.2–3.6 V.
    ///
    /// # Safety
    /// The board must actually sustain these bounds, including supply transients
    /// and oscillator tolerance, and satisfy its own datasheet temperature/power
    /// conditions. Clock/voltage changes must retain the envelope while the
    /// driver exists. Validate interrupt and watchdog latency for blocking FLASH
    /// operations; no maximum completion time or power-failure recovery is
    /// promised. The driver cannot measure voltage or prove these conditions.
    pub unsafe fn new(supply_mv: u16, max_hclk_hz: u32) -> Result<Self, Error> {
        let (min, max) = crate::FLASH_SUPPLY_RANGE_MV;
        if !(min..=max).contains(&supply_mv)
            || max_hclk_hz == 0
            || max_hclk_hz > crate::FLASH_MAX_HCLK_HZ
            || (supply_mv < crate::FLASH_LOW_VOLTAGE_THRESHOLD_MV
                && max_hclk_hz > crate::FLASH_LOW_VOLTAGE_MAX_HCLK_HZ)
        {
            return Err(Error::InvalidConditions);
        }
        Ok(Self { max_hclk_hz })
    }
    /// Qualify the declared RCC board envelope using the actual HCLK upper bound.
    /// The source-qualified temperature interval is checked, and the outward
    /// rounded `ClockBounds::maximum` is used instead of nominal HCLK.
    ///
    /// # Safety
    /// The board must sustain `board` and the factory-HSI/clock contract behind
    /// `clocks` for this driver's lifetime, as well as all requirements of
    /// [`Self::new`]. Passing declarations does not measure or establish them.
    pub unsafe fn from_rcc(
        board: crate::rcc::OperatingConditions,
        clocks: crate::rcc::Clocks,
    ) -> Result<Self, Error> {
        let (min, max) = crate::rcc::HSI_BOUND_TEMPERATURE_C;
        if board.min_temperature_c < min
            || board.max_temperature_c > max
            || board.min_temperature_c > board.max_temperature_c
        {
            return Err(Error::InvalidConditions);
        }
        unsafe { Self::new(board.supply_mv, clocks.hclk_bounds().maximum().0) }
    }
}

/// Exclusive blocking driver for one explicitly reserved storage partition.
/// Dropping it never resets FLASH, changes WAIT, or disables its clock.
pub struct Flash<'d, MODE = Blocking> {
    _peripheral: Peri<'d, FLASH>,
    region: ReservedRegion<'d>,
    conditions: OperatingConditions,
    flash: pac::flash::Flash,
    _mode: PhantomData<MODE>,
}
impl<'d> Flash<'d, Blocking> {
    /// Acquire a previously reserved partition and validate controller state.
    /// Returns an error rather than resetting a controller or altering WAIT.
    /// Error IRQs must be disabled; FLASH/RAM IRQ ownership remains elsewhere.
    pub fn new_blocking(
        peripheral: Peri<'d, FLASH>,
        region: ReservedRegion<'d>,
        conditions: OperatingConditions,
    ) -> Result<Self, Error> {
        #[cfg(not(target_arch = "arm"))]
        panic!("CW32 FLASH hardware is only accessible on its ARM target");
        #[cfg(target_arch = "arm")]
        {
            let mut driver = Self {
                _peripheral: peripheral,
                region,
                conditions,
                flash: pac::FLASH,
                _mode: PhantomData,
            };
            driver.initialize()?;
            Ok(driver)
        }
    }

    /// Reserved capacity, not whole-device capacity.
    pub fn capacity(&self) -> usize {
        self.region.len() as usize
    }
    /// Absolute first address of this partition.
    pub fn start(&self) -> u32 {
        self.region.start()
    }
    /// Read bytes at a partition-relative offset, with no FLASH-array reference.
    pub fn blocking_read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Error> {
        let range = checked_range(&self.region.range, offset, bytes.len())?;
        if bytes.is_empty() {
            return Ok(());
        }
        self.check_state()?;
        // Reads are only valid in Read mode, including after an external stale MODE.
        self.set_mode(Mode::Read)?;
        for (address, byte) in range.zip(bytes) {
            *byte = self.read_byte(address);
        }
        Ok(())
    }
    /// Program already-erased bytes. Prechecks the entire request before any
    /// programming. Later hardware failures may leave a programmed prefix.
    pub fn blocking_write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Error> {
        let range = checked_range(&self.region.range, offset, bytes.len())?;
        if bytes.is_empty() {
            return Ok(());
        }
        self.check_state()?;
        self.set_mode(Mode::Read)?;
        for address in range.clone() {
            if self.read_byte(address) != 0xff {
                return Err(Error::NotErased);
            }
        }
        for (address, &byte) in range.zip(bytes) {
            self.operate(address, byte, Mode::Program)?;
            if self.read_byte(address) != byte {
                return Err(Error::Verify);
            }
        }
        Ok(())
    }
    /// Erase page-aligned partition-relative `[from, to)`. Reversed ranges fail.
    /// A later error may leave an erased prefix. No timeout/abort is available.
    pub fn blocking_erase(&mut self, from: u32, to: u32) -> Result<(), Error> {
        let len = to.checked_sub(from).ok_or(Error::OutOfBounds)?;
        let range = checked_range(&self.region.range, from, len as usize)?;
        if !from.is_multiple_of(ERASE_SIZE as u32) || !to.is_multiple_of(ERASE_SIZE as u32) {
            return Err(Error::NotAligned);
        }
        for address in range.step_by(ERASE_SIZE) {
            self.operate(address, 0xff, Mode::PageErase)?;
            for offset in 0..ERASE_SIZE as u32 {
                if self.read_byte(address + offset) != 0xff {
                    return Err(Error::Verify);
                }
            }
        }
        Ok(())
    }
}
impl ErrorType for Flash<'_, Blocking> {
    type Error = Error;
}
impl ReadNorFlash for Flash<'_, Blocking> {
    const READ_SIZE: usize = READ_SIZE;
    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Error> {
        self.blocking_read(offset, bytes)
    }
    fn capacity(&self) -> usize {
        self.capacity()
    }
}

const KEY: u16 = 0x5a5a;

fn validate_region(range: &Range<u32>, capacity: Option<u32>) -> Result<(), Error> {
    let capacity = capacity.ok_or(Error::UnknownCapacity)?;
    if range.start > range.end || range.end > capacity {
        return Err(Error::OutOfBounds);
    }
    #[cfg(any(flash_cw32l010_v1, flash_cw32l011_v1, flash_cw32l012_v1))]
    if range.end > capacity - ERASE_SIZE as u32 {
        return Err(Error::ReservedProtection);
    }
    if range.start == range.end {
        return Err(Error::EmptyRegion);
    }
    if !range.start.is_multiple_of(ERASE_SIZE as u32)
        || !range.end.is_multiple_of(ERASE_SIZE as u32)
    {
        return Err(Error::NotAligned);
    }
    Ok(())
}
fn checked_range(region: &Range<u32>, offset: u32, len: usize) -> Result<Range<u32>, Error> {
    let len = u32::try_from(len).map_err(|_| Error::OutOfBounds)?;
    let end = offset.checked_add(len).ok_or(Error::OutOfBounds)?;
    if end > region.end - region.start {
        return Err(Error::OutOfBounds);
    }
    Ok(region.start.checked_add(offset).ok_or(Error::OutOfBounds)?
        ..region.start.checked_add(end).ok_or(Error::OutOfBounds)?)
}
impl Flash<'_, Blocking> {
    fn busy(&self) -> bool {
        #[cfg(any(flash_cw32l010_v1, flash_cw32l011_v1, flash_cw32l012_v1))]
        {
            self.flash.isr().read().busy()
        }
        #[cfg(not(any(flash_cw32l010_v1, flash_cw32l011_v1, flash_cw32l012_v1)))]
        {
            self.flash.cr1().read().busy()
        }
    }

    fn check_state(&self) -> Result<(), Error> {
        if FLASH::RCC_INFO.reset_asserted() {
            return Err(Error::HeldInReset);
        }
        if !FLASH::RCC_INFO.is_enabled() {
            return Err(Error::ClockNotEnabled);
        }
        if self.busy() {
            return Err(Error::Busy);
        }
        let irq = self.flash.ier().read();
        let enabled = irq.pc() || irq.pagelock() || irq.prog();
        #[cfg(any(flash_cw32l011_v1, flash_cw32l012_v1))]
        let enabled = enabled || irq.sdkerr();
        #[cfg(flash_cw32l012_v1)]
        let enabled = enabled || irq.cacheon();
        if enabled {
            return Err(Error::InterruptsEnabled);
        }
        let cr2 = self.flash.cr2().read();
        let wait = u32::from(cr2.wait());
        if wait > crate::FLASH_MAX_WAIT_STATES
            || self.conditions.max_hclk_hz > (wait + 1) * crate::FLASH_WAIT_STEP_HZ
        {
            return Err(Error::InvalidLatency);
        }
        #[cfg(flash_cw32l012_v1)]
        if cr2.cacheinvalid() {
            return Err(Error::Configuration);
        }
        #[cfg(any(flash_cw32l010_v1, flash_cw32l011_v1, flash_cw32l012_v1))]
        {
            // SDKCFR is read-only. Reset START=127, END=0 has no active interval.
            // Protect any nonempty reported interval, including unexpected values.
            let protected = self.flash.sdkcfr().read();
            let first = u32::from(protected.start()) * ERASE_SIZE as u32;
            let last = (u32::from(protected.end()) + 1) * ERASE_SIZE as u32;
            if first < last && self.region.range.start < last && first < self.region.range.end {
                return Err(Error::ReservedProtection);
            }
        }
        Ok(())
    }

    fn initialize(&mut self) -> Result<(), Error> {
        critical_section::with(|cs| {
            if FLASH::RCC_INFO.reset_asserted() {
                return Err(Error::HeldInReset);
            }
            FLASH::RCC_INFO
                .enable_with_cs_readback(cs, Readback::None)
                .expect("unpolled FLASH gate write cannot fail");
            self.check_state()?;
            self.set_mode(Mode::Read)
        })
    }

    fn set_mode(&self, mode: Mode) -> Result<(), Error> {
        #[cfg(not(any(flash_cw32l010_v1, flash_cw32l011_v1, flash_cw32l012_v1)))]
        let standby = self.flash.cr1().read().standby();
        self.flash.cr1().write(|v| {
            v.set_key(KEY);
            v.set_mode(mode);
            #[cfg(not(any(flash_cw32l010_v1, flash_cw32l011_v1, flash_cw32l012_v1)))]
            v.set_standby(standby);
        });
        if self.flash.cr1().read().mode() != mode {
            return Err(Error::Configuration);
        }
        Ok(())
    }

    fn error_flags(&self) -> ErrorFlags {
        let status = self.flash.isr().read();
        let bits = u8::from(status.pc())
            | (u8::from(status.pagelock()) << 1)
            | (u8::from(status.prog()) << 4);
        #[cfg(any(flash_cw32l011_v1, flash_cw32l012_v1))]
        let bits = bits | (u8::from(status.sdkerr()) << 2);
        #[cfg(flash_cw32l012_v1)]
        let bits = bits | (u8::from(status.cacheon()) << 3);
        ErrorFlags(bits)
    }

    fn clear_errors(&self) {
        // W0C, never RMW: seed the documented low-five-bit reset image to retain
        // reserved [3:2] (or [3]) at one; clear only implemented typed fields.
        let mut clear = pac::flash::regs::Icr(0x1f);
        clear.set_pc(false);
        clear.set_pagelock(false);
        clear.set_prog(false);
        #[cfg(any(flash_cw32l011_v1, flash_cw32l012_v1))]
        clear.set_sdkerr(false);
        #[cfg(flash_cw32l012_v1)]
        clear.set_cacheon(false);
        self.flash.icr().write_value(clear);
    }

    fn locks(&self) -> u64 {
        // Pack typed one-bit fields, including all four L083 register groups.
        // This u64 is a software snapshot, never a raw register image.
        macro_rules! read_group {
            ($register:ident, $($field:ident),+ $(,)?) => {{
                let value = self.flash.$register().read();
                let mut bits = 0u64;
                for (index, unlocked) in [$(value.$field()),+].into_iter().enumerate() {
                    bits |= u64::from(unlocked) << index;
                }
                bits
            }};
        }
        #[cfg(any(flash_cw32f002_v1, flash_cw32f020_v1))]
        {
            read_group!(
                pagelock, lock0, lock1, lock2, lock3, lock4, lock5, lock6, lock7
            )
        }
        #[cfg(flash_cw32f003_v1)]
        {
            read_group!(
                pagelock, lock0, lock1, lock2, lock3, lock4, lock5, lock6, lock7, lock8, lock9
            )
        }
        #[cfg(any(flash_v1, flash_cw32l010_v1, flash_cw32l011_v1, flash_cw32l012_v1))]
        {
            read_group!(
                pagelock, lock0, lock1, lock2, lock3, lock4, lock5, lock6, lock7, lock8, lock9,
                lock10, lock11, lock12, lock13, lock14, lock15
            )
        }
        #[cfg(flash_cw32l031_v1)]
        {
            read_group!(
                pagelock1, lock0, lock1, lock2, lock3, lock4, lock5, lock6, lock7, lock8, lock9,
                lock10, lock11, lock12, lock13, lock14, lock15
            )
        }
        #[cfg(flash_cw32l083_v1)]
        {
            (read_group!(
                pagelock1, lock0, lock1, lock2, lock3, lock4, lock5, lock6, lock7, lock8, lock9,
                lock10, lock11, lock12, lock13, lock14, lock15
            ) << 0)
                | (read_group!(
                    pagelock2, lock16, lock17, lock18, lock19, lock20, lock21, lock22, lock23,
                    lock24, lock25, lock26, lock27, lock28, lock29, lock30, lock31
                ) << 16)
                | (read_group!(
                    pagelock3, lock32, lock33, lock34, lock35, lock36, lock37, lock38, lock39,
                    lock40, lock41, lock42, lock43, lock44, lock45, lock46, lock47
                ) << 32)
                | (read_group!(
                    pagelock4, lock48, lock49, lock50, lock51, lock52, lock53, lock54, lock55,
                    lock56, lock57, lock58, lock59, lock60, lock61, lock62, lock63
                ) << 48)
        }
    }

    fn set_locks(&self, locks: u64) {
        macro_rules! write_group {
            ($register:ident, $offset:expr, $($field:ident : $index:expr),+ $(,)?) => {
                self.flash.$register().write(|v| {
                    v.set_key(KEY);
                    $(v.$field(locks & (1u64 << ($offset + $index)) != 0);)+
                });
            };
        }
        #[cfg(any(flash_cw32f002_v1, flash_cw32f020_v1))]
        write_group!(pagelock, 0, set_lock0: 0, set_lock1: 1, set_lock2: 2, set_lock3: 3, set_lock4: 4, set_lock5: 5, set_lock6: 6, set_lock7: 7);
        #[cfg(flash_cw32f003_v1)]
        write_group!(pagelock, 0, set_lock0: 0, set_lock1: 1, set_lock2: 2, set_lock3: 3, set_lock4: 4, set_lock5: 5, set_lock6: 6, set_lock7: 7, set_lock8: 8, set_lock9: 9);
        #[cfg(any(flash_v1, flash_cw32l010_v1, flash_cw32l011_v1, flash_cw32l012_v1))]
        write_group!(pagelock, 0, set_lock0: 0, set_lock1: 1, set_lock2: 2, set_lock3: 3, set_lock4: 4, set_lock5: 5, set_lock6: 6, set_lock7: 7, set_lock8: 8, set_lock9: 9, set_lock10: 10, set_lock11: 11, set_lock12: 12, set_lock13: 13, set_lock14: 14, set_lock15: 15);
        #[cfg(flash_cw32l031_v1)]
        write_group!(pagelock1, 0, set_lock0: 0, set_lock1: 1, set_lock2: 2, set_lock3: 3, set_lock4: 4, set_lock5: 5, set_lock6: 6, set_lock7: 7, set_lock8: 8, set_lock9: 9, set_lock10: 10, set_lock11: 11, set_lock12: 12, set_lock13: 13, set_lock14: 14, set_lock15: 15);
        #[cfg(flash_cw32l083_v1)]
        {
            write_group!(pagelock1, 0, set_lock0: 0, set_lock1: 1, set_lock2: 2, set_lock3: 3, set_lock4: 4, set_lock5: 5, set_lock6: 6, set_lock7: 7, set_lock8: 8, set_lock9: 9, set_lock10: 10, set_lock11: 11, set_lock12: 12, set_lock13: 13, set_lock14: 14, set_lock15: 15);
            write_group!(pagelock2, 16, set_lock16: 0, set_lock17: 1, set_lock18: 2, set_lock19: 3, set_lock20: 4, set_lock21: 5, set_lock22: 6, set_lock23: 7, set_lock24: 8, set_lock25: 9, set_lock26: 10, set_lock27: 11, set_lock28: 12, set_lock29: 13, set_lock30: 14, set_lock31: 15);
            write_group!(pagelock3, 32, set_lock32: 0, set_lock33: 1, set_lock34: 2, set_lock35: 3, set_lock36: 4, set_lock37: 5, set_lock38: 6, set_lock39: 7, set_lock40: 8, set_lock41: 9, set_lock42: 10, set_lock43: 11, set_lock44: 12, set_lock45: 13, set_lock46: 14, set_lock47: 15);
            write_group!(pagelock4, 48, set_lock48: 0, set_lock49: 1, set_lock50: 2, set_lock51: 3, set_lock52: 4, set_lock53: 5, set_lock54: 6, set_lock55: 7, set_lock56: 8, set_lock57: 9, set_lock58: 10, set_lock59: 11, set_lock60: 12, set_lock61: 13, set_lock62: 14, set_lock63: 15);
        }
    }

    fn operate(&mut self, address: u32, byte: u8, mode: Mode) -> Result<(), Error> {
        critical_section::with(|_| {
            self.check_state()?;
            let saved_locks = self.locks() & crate::FLASH_LOCK_MASK;
            let unlocked = saved_locks | (1u64 << (address / crate::FLASH_LOCK_GROUP_BYTES));
            let saved_cr2 = self.flash.cr2().read();
            // Cache/fetch must be disabled before changing MODE or touching the
            // array. RCC-owned WAIT is carried through every typed keyed write.
            let cache_disabled = self.set_cache(saved_cr2, false);
            self.clear_errors();
            let result = if !cache_disabled {
                Err(Error::Configuration)
            } else {
                self.set_mode(mode).and_then(|()| {
                    self.set_locks(unlocked);
                    if self.locks() & crate::FLASH_LOCK_MASK != unlocked {
                        return Err(Error::Protected);
                    }
                    self.trigger_byte(address, byte);
                    // No documented safe abort: after triggering, never return
                    // or attempt MODE/lock/cache cleanup while BUSY is asserted.
                    while self.busy() {
                        core::hint::spin_loop();
                    }
                    self.synchronize();
                    let flags = self.error_flags();
                    if flags.bits() == 0 {
                        Ok(())
                    } else {
                        Err(Error::Hardware(flags))
                    }
                })
            };
            let hardware_errors = match result {
                Err(Error::Hardware(flags)) => flags,
                _ => ErrorFlags(0),
            };
            // All exits below are idle. Restore only documented writable fields.
            self.set_locks(saved_locks);
            let locks_restored = self.locks() & crate::FLASH_LOCK_MASK == saved_locks;
            let mode_restored = self.set_mode(Mode::Read).is_ok();
            // L012 explicitly invalidates stale cache before restoration. The
            // manual defines the 1->0 protocol; doing it after each mutation is
            // our conservative choice, not a claimed mandatory operation step.
            let cache_restored = mode_restored && self.restore_cache(saved_cr2);
            if !mode_restored || !locks_restored || !cache_restored {
                Err(Error::Cleanup {
                    hardware_errors,
                    mode_restored,
                    locks_restored,
                    cache_restored,
                })
            } else {
                result
            }
        })
    }

    fn set_cache(&self, saved: pac::flash::regs::Cr2, restore: bool) -> bool {
        #[cfg(any(flash_v1, flash_cw32f020_v1, flash_cw32l012_v1))]
        {
            self.flash.cr2().write(|v| {
                v.set_key(KEY);
                v.set_wait(saved.wait());
                v.set_fetch(restore && saved.fetch());
                v.set_cache(restore && saved.cache());
                #[cfg(flash_cw32l012_v1)]
                v.set_cacheinvalid(false);
            });
            self.synchronize();
            let actual = self.flash.cr2().read();
            let restored = actual.wait() == saved.wait()
                && actual.fetch() == (restore && saved.fetch())
                && actual.cache() == (restore && saved.cache());
            #[cfg(flash_cw32l012_v1)]
            let restored = restored && !actual.cacheinvalid();
            restored
        }
        #[cfg(not(any(flash_v1, flash_cw32f020_v1, flash_cw32l012_v1)))]
        {
            let _ = restore;
            self.flash.cr2().read().wait() == saved.wait()
        }
    }

    fn restore_cache(&self, saved: pac::flash::regs::Cr2) -> bool {
        #[cfg(flash_cw32l012_v1)]
        {
            // Keep cache/fetch off through both writes and verify each state.
            self.flash.cr2().write(|v| {
                v.set_key(KEY);
                v.set_wait(saved.wait());
                v.set_cacheinvalid(true);
            });
            self.synchronize();
            let pulse = self.flash.cr2().read();
            let asserted = pulse.cacheinvalid()
                && !pulse.cache()
                && !pulse.fetch()
                && pulse.wait() == saved.wait();
            let released = self.set_cache(saved, false);
            if !asserted || !released {
                return false;
            }
        }
        self.set_cache(saved, true)
    }

    fn read_byte(&self, address: u32) -> u8 {
        #[cfg(target_arch = "arm")]
        unsafe {
            let value: u32;
            // The zero-based array is never converted to a Rust pointer/reference.
            core::arch::asm!("ldrb {value}, [{address}]", value = out(reg) value,
                address = in(reg) address, options(nostack, preserves_flags));
            value as u8
        }
        #[cfg(not(target_arch = "arm"))]
        {
            let _ = address;
            panic!("hardware FLASH read attempted on host")
        }
    }
    fn trigger_byte(&self, address: u32, value: u8) {
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("strb {value}, [{address}]", value = in(reg) u32::from(value),
                address = in(reg) address, options(nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            let _ = (address, value);
            panic!("hardware FLASH write attempted on host")
        }
    }
    fn synchronize(&self) {
        #[cfg(target_arch = "arm")]
        {
            cortex_m::asm::dsb();
            cortex_m::asm::isb();
        }
        #[cfg(not(target_arch = "arm"))]
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
