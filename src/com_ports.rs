#![allow(dead_code)]

use x86_64::instructions::port::*;

#[derive(Copy, Clone, Debug)]
#[repr(u16)]
pub enum ComPort {
    Com1 = 0x3f8,
    Com2 = 0x2f8,
    Com3 = 0x3e8,
    Com4 = 0x2e8,
    Com5 = 0x5f8,
    Com6 = 0x4f8,
    Com7 = 0x5e8,
    Com8 = 0x4e8,
}

#[derive(Copy, Clone, Debug)]
#[repr(u16)]
enum ComPortOffset {
    Data = 0,
    DivisorHigh = 1,
    FifoControl = 2,
    LineControl = 3,
    ModemControl = 4,
    LineStatus = 5,
    ModemStatus = 6,
    Scratch = 7,
}

const DEFUALT_BAUD_RATE: usize = 115_200;

pub fn set_baud_rate(com_port: ComPort, baud_rate: usize) {
    let divisor: u16 = (DEFUALT_BAUD_RATE / baud_rate) as u16;
    let mut divisor_low_port = Port::<u8>::new(com_port as u16 + ComPortOffset::Data as u16);
    let mut divisor_high_port = Port::<u8>::new(com_port as u16 + ComPortOffset::Data as u16 + 1);
    let mut line_control_port =
        Port::<u8>::new(com_port as u16 + ComPortOffset::LineControl as u16);
    unsafe {
        line_control_port.write(0x80);
        divisor_low_port.write(divisor as u8);
        divisor_high_port.write((divisor >> 8) as u8);
    }
}
pub fn setup_port(com_port: ComPort) {
    set_baud_rate(com_port, DEFUALT_BAUD_RATE);
    let mut line_control_port =
        Port::<u8>::new(com_port as u16 + ComPortOffset::LineControl as u16);
    let mut fifo_control_port =
        Port::<u8>::new(com_port as u16 + ComPortOffset::FifoControl as u16);
    let mut modem_control_port =
        Port::<u8>::new(com_port as u16 + ComPortOffset::ModemControl as u16);
    unsafe {
        line_control_port.write(0x03);
        fifo_control_port.write(0xC7);
        modem_control_port.write(0x0B);
    }
}
pub fn putc(com_port: ComPort, c: u8) {
    let mut line_status_port = Port::<u8>::new(com_port as u16 + ComPortOffset::LineStatus as u16);
    let mut data_port = Port::<u8>::new(com_port as u16 + ComPortOffset::Data as u16);
    unsafe {
        while (line_status_port.read() & 0x20) == 0 {}
        data_port.write(c);
    }
}
pub fn puts(com_port: ComPort, s: &str) {
    for c in s.bytes() {
        putc(com_port, c);
    }
}
