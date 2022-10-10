#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate psoc6_hal;

use cortex_m_rt::entry;
use panic_halt as _;

use psoc6_hal::{
    delay::Delay,
    drivers::cpuss::{interrupt::InterruptSource, nvic::Nvic, Cpuss},
    drivers::ipc::{ChannelConfig, IpcChannel, MaskBits},
    gpio::GpioExt,
    pac,
    pac::interrupt,
    prelude::*,
};

#[interrupt]
fn NVIC_MUX0_IRQn() {
    //do something fancy with the interrupt.
}
#[interrupt]
fn NVIC_MUX15_IRQn() {
    //do something to the last one.
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let cpuss = Cpuss::from(p.CPUSS);
    cpuss.configure_interrupt_mux(
        InterruptSource::IOSS_INTERRUPTS_GPIO_0,
        interrupt::NVIC_MUX0_IRQn,
    );
    let mut nvic = Nvic::from(cp.NVIC);
    // # Safety: Unsafe because:
    //  - setting priority can break priority based critical sections.
    //  - unmasking can break mask based critical sections.
    #[allow(unsafe_code)]
    unsafe {
        nvic.configure_interrupt(InterruptSource::IOSS_INTERRUPTS_GPIO_0, 4u8);
    }
    // IPC test
    let struct0_cfg = ChannelConfig {
        release_mask: MaskBits::struct0,
        notify_mask: MaskBits::struct0,
    };

    let ipc_channels = p.IPC.split();
    let syscall_cm0 = ipc_channels.syscall_cm0.into_released();

    let data = [0u32; 8];
    let d_ptr = &data as *const u32;
    let _syscall_cm0 = syscall_cm0
        .send_data_ptr(d_ptr, struct0_cfg.notify_mask)
        .unwrap();
    //end IPC test
    let gpio = p.GPIO.split();

    let mut led3 = gpio.p6_3.into_strong_output();

    let mut delay = Delay::new(cp.SYST);

    loop {
        led3.set_high().unwrap();
        delay.delay_ms(1000u32);
        led3.set_low().unwrap();
        delay.delay_ms(1000u32);
    }
}
