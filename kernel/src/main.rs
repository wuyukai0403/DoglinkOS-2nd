#![no_std]
#![no_main]

use core::arch::asm;
use limine::request::{RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;
use DoglinkOS_2nd::console::init as init_terminal;
use DoglinkOS_2nd::acpi::{init as init_acpi, parse_madt};
use DoglinkOS_2nd::apic::{io::init as init_ioapic, local::init as init_lapic};
use DoglinkOS_2nd::cpu::show_cpu_info;
use DoglinkOS_2nd::int::init as init_interrupt;
use DoglinkOS_2nd::mm::{init as init_mm, page_alloc::init as init_mm_ext};
use DoglinkOS_2nd::pcie::enumrate::{init as init_pcie, doit};
use DoglinkOS_2nd::println;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

/// Define the stand and end markers for Limine requests.
#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[link_section = ".requests_end_marker"]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[no_mangle]
extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());
    init_mm();
    init_terminal();
    println!(r"  ____                   _   _           _       ___    ____            ____                _
|  _ \    ___     __ _  | | (_)  _ __   | | __  / _ \  / ___|          |___ \   _ __     __| |
| | | |  / _ \   / _` | | | | | | '_ \  | |/ / | | | | \___ \   _____    __) | | '_ \   / _` |
| |_| | | (_) | | (_| | | | | | | | | | |   <  | |_| |  ___) | |_____|  / __/  | | | | | (_| |
|____/   \___/   \__, | |_| |_| |_| |_| |_|\_\  \___/  |____/          |_____| |_| |_|  \__,_|
                 |___/");
    init_mm_ext();
    init_interrupt();
    init_lapic();
    unsafe { init_acpi() };
    init_ioapic(parse_madt());
    init_pcie();
    show_cpu_info();
    doit(|bus, device, function, config| {
        let vendor_id = config.vendor_id;
        let device_id = config.device_id;
        println!("[INFO] kmain: found PCIe device: {:02x}:{:02x}.{} {:02x}{:02x}: {:04x}:{:04x}",
            bus, device, function,
            config.class_code, config.subclass,
            vendor_id, device_id);
    });
    let mut cnt = 0;
    doit(|_, _, _, _| cnt += 1);
    println!("[INFO] kmain: total {cnt} PCIe devices");
    DoglinkOS_2nd::blockdev::ramdisk::test();
    DoglinkOS_2nd::blockdev::ahci::init();
    DoglinkOS_2nd::mm::page_alloc::test();
    hang();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("panic: {:#?}", info);
    hang();
}

fn hang() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
