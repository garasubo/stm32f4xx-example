#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::serial::{Read, Write};
use stm32f4xx_hal::{
    gpio::GpioExt, rcc::RccExt, serial::config::Config, serial::Serial, stm32::Peripherals,
    time::U32Ext,
};

extern crate panic_halt;

#[entry]
unsafe fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpoid = p.GPIOD.split();
    let pd8 = gpoid.pd8.into_alternate_af7();
    let pd9 = gpoid.pd9.into_alternate_af7();
    let config = Config::default().baudrate(115_200.bps());
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut serial = Serial::usart3(p.USART3, (pd8, pd9), config, clocks).unwrap();
    loop {
        while !serial.is_rxne() {}
        if let Ok(b) = serial.read() {
            while !serial.is_txe() {}
            serial.write(b).unwrap();
        }
    }
}
