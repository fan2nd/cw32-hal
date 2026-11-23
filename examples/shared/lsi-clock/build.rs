use std::{env, fs, path::PathBuf};

fn main() {
    let metadata = &cw32_metapac::metadata::METADATA;
    assert_eq!(metadata.memory.len(), 1, "select one exact package");
    assert!(matches!(
        metadata.name,
        "CW32F002F3P7"
            | "CW32F002F3U7"
            | "CW32F003F4P7"
            | "CW32F003F4U7"
            | "CW32F003E4P7"
            | "CW32F020C6U7"
            | "CW32F030C8T7"
            | "CW32A030C8T7"
            | "CW32L010F8P6"
            | "CW32L010F8U6"
            | "CW32L010Y8M6"
            | "CW32L011K8T6"
            | "CW32L011K8U6"
            | "CW32L012C8T6"
            | "CW32L012C8U6"
            | "CW32L031C8T6"
            | "CW32L031C8U6"
            | "CW32L031F8U6"
            | "CW32R031C8U6"
            | "CW32L052C8T6"
            | "CW32L052R8S6"
            | "CW32L052R8T6"
            | "CW32L083RBT6"
            | "CW32L083RCT6"
            | "CW32L083RCS6"
            | "CW32L083MCT6"
            | "CW32L083VCT6"
    ));
    assert!(metadata.peripherals.iter().any(|p| {
        p.clock_limits
            .as_ref()
            .is_some_and(|c| c.lsi_sysclk.is_some())
    }));
    let exact_memory = match metadata.name {
        // Own F002 DS Rev1.2 tables 3-1 and 6-1.
        "CW32F002F3P7" | "CW32F002F3U7" => Some((16 * 1024, 2 * 1024, false)),
        // Own F003 DS Rev1.9 PDF pages 5, 8, 27 and 62, including E4P7.
        "CW32F003F4P7" | "CW32F003F4U7" | "CW32F003E4P7" => Some((20 * 1024, 3 * 1024, false)),
        // Own L010 DS CN V1.3 PDF pages 8 and 24-25.
        "CW32L010F8P6" | "CW32L010F8U6" | "CW32L010Y8M6" => {
            let rtc = metadata
                .peripherals
                .iter()
                .find(|p| p.name == "RTC")
                .unwrap();
            let calendar = rtc.rtc_calendar.as_ref().unwrap();
            assert_eq!(calendar.source, "HSIOSC");
            assert_eq!(calendar.nominal_hz, 48_000_000);
            Some((64 * 1024, 4 * 1024, true))
        }
        // Own L011 DS CN V1.1: both exact packages retain raw-HSIOSC RTC.
        "CW32L011K8T6" | "CW32L011K8U6" => {
            let rtc = metadata
                .peripherals
                .iter()
                .find(|p| p.name == "RTC")
                .unwrap();
            let calendar = rtc.rtc_calendar.as_ref().unwrap();
            assert_eq!(calendar.source, "HSIOSC");
            assert_eq!(calendar.nominal_hz, 96_000_000);
            Some((64 * 1024, 6 * 1024, true))
        }
        // Own L012 DS CN V1.0: both exact packages retain raw-HSIOSC RTC.
        "CW32L012C8T6" | "CW32L012C8U6" => {
            let rtc = metadata
                .peripherals
                .iter()
                .find(|p| p.name == "RTC")
                .unwrap();
            let calendar = rtc.rtc_calendar.as_ref().unwrap();
            assert_eq!(calendar.source, "HSIOSC");
            assert_eq!(calendar.nominal_hz, 96_000_000);
            Some((64 * 1024, 8 * 1024, true))
        }
        // Own L031 DS Rev1.9 PDF pages 10, 32 and 77-78.
        "CW32L031C8T6" | "CW32L031C8U6" | "CW32L031F8U6" => Some((64 * 1024, 8 * 1024, true)),
        // Own R031 DS CN V1.2 PDF pages 11 and 35, and own PDSC.
        "CW32R031C8U6" => Some((64 * 1024, 8 * 1024, true)),

        // Own L052 DS CN V1.3: exact LQFP48 and both LQFP64 packages.
        "CW32L052C8T6" | "CW32L052R8S6" | "CW32L052R8T6" => Some((64 * 1024, 8 * 1024, true)),
        // Own L083 DS CN V1.9 tables 3-1/9-1: RBT6 differs from the C-density parts.
        "CW32L083RBT6" => Some((128 * 1024, 24 * 1024, true)),
        "CW32L083RCT6" | "CW32L083RCS6" | "CW32L083MCT6" | "CW32L083VCT6" => {
            Some((256 * 1024, 24 * 1024, true))
        }
        _ => None,
    };
    if let Some((flash_size, ram_size, has_rtc)) = exact_memory {
        assert_eq!(
            metadata.peripherals.iter().any(|p| p.name == "RTC"),
            has_rtc
        );
        // Retain each exact package's real linker limits.
        let banks = metadata.memory[0];
        assert_eq!(banks.len(), 2);
        let flash = banks.iter().find(|bank| bank.name == "FLASH").unwrap();
        let ram = banks.iter().find(|bank| bank.name == "RAM").unwrap();
        assert_eq!((flash.address, flash.size), (0x0000_0000, flash_size));
        assert_eq!((ram.address, ram.size), (0x2000_0000, ram_size));
    }
    let mut memory = String::from("MEMORY {\n");
    for bank in metadata.memory[0] {
        assert!(matches!(bank.name, "FLASH" | "RAM"));
        memory.push_str(&format!(
            "  {} : ORIGIN = {:#x}, LENGTH = {}\n",
            bank.name, bank.address, bank.size
        ));
    }
    memory.push_str("}\n");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("memory.x"), memory).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rerun-if-changed=build.rs");
}
