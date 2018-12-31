#![no_std]
#![no_main]
use microbit;
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD9600;
use core::fmt::Write;
use core::panic::PanicInfo;
extern crate cortex_m_rt;
use cortex_m_rt::*;

extern{
    fn wait_for_interrupt();
}

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let mut gpio = p.GPIO.split();
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, rx) = serial::Serial::uart0(p.UART0, tx, rx,BAUD9600 ).split();
        let _ = write!(tx, "cortex_m_rt entry.\r\n");
        let _ = write!(tx,"heap_start : {:x}\r\n",cortex_m_rt::heap_start() as u32);
        unsafe{wait_for_interrupt();}
        let _ = write!(tx, "get interrupt");
        
    }
    loop {
        unsafe{wait_for_interrupt();}
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        unsafe{wait_for_interrupt();}
    }
}
