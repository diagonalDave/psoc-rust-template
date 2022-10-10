#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]


extern crate psoc6_hal;
use panic_halt as _;
use cortex_m_rt::entry;

use psoc6_hal::{
    delay::Delay,
    drivers::cpuss::{nvic::Nvic, Cpuss},
    drivers::ipc::{ChannelConfig, IpcChannel, MaskBits},
    gpio::GpioExt,
    pac,
    pac::{
        interrupt,
        Interrupt,
    },
    prelude::*,
};

/// TODO need to modify the build script to include feature cm0 and cm4.
/// so the linker doesn't fall over.
#[interrupt]
fn IOSS_INTERRUPTS_GPIO_0(){
    //do something fancy with the interrupt.
}
#[interrupt]
fn CPUSS_INTERRUPTS_IPC_14(){
    //do something to the last one.
}
#[interrupt]
fn CSD_INTERRUPT(){
    //do something to the last one.
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut nvic = Nvic::from(cp.NVIC);
    // # Safety: Unsafe because:
    //  - setting priority can break priority based critical sections.
    //  - unmasking can break mask based critical sections.
    #[allow(unsafe_code)]
    unsafe{
        nvic.configure_interrupt(
            Interrupt::CPUSS_INTERRUPTS_IPC_14,
            3u8
        );
    }
    nvic.clear_interrupt(Interrupt::CPUSS_INTERRUPTS_IPC_14);
    let gpio = p.GPIO.split();

    let mut led4 = gpio.p7_1.into_strong_output();

    let mut delay = Delay::new(cp.SYST);


    
    loop {
        led4.set_high().unwrap();
        delay.delay_ms(1000u32);
        led4.set_low().unwrap();
        delay.delay_ms(1000u32);
    }
}


