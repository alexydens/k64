#![no_std]
#![no_main]

mod com_ports;
mod gdt;

use limine::request::{RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests_start"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[link_section = ".requests_end"]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    com_ports::setup_port(com_ports::ComPort::Com1);
    com_ports::puts(com_ports::ComPort::Com1, "SUCCESS: Initialized COM1!\n");

    let mut gdt: gdt::Gdt = gdt::Gdt::new();
    gdt.init();
    com_ports::puts(com_ports::ComPort::Com1, "SUCCESS: Initialized GDT!\n");

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    let message = _info.message().as_str().unwrap();
    com_ports::puts(com_ports::ComPort::Com1, message);
    hcf();
}

fn hcf() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
