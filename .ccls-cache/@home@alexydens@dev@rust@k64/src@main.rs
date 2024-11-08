#![no_std]
#![no_main]

mod com_ports;

use core::arch::asm;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};
use limine::BaseRevision;

#[used]
#[link_section = ".requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests_start"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[link_section = ".requests_end"]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            for i in 0..100_u64 {
                let pixel_offset = i * framebuffer.pitch() + i * 4;
                *(framebuffer.addr().add(pixel_offset as usize) as *mut u32) = 0xFFFFFFFF;
            }
        }
    }

    /* Print to serial */
    com_ports::setup_port(com_ports::ComPort::Com1);
    com_ports::puts(com_ports::ComPort::Com1, "Hello world!\n");

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}
