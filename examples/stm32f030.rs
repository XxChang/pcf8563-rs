#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use hal::{gpio, delay};
use panic_rtt_target as _;

use stm32f0xx_hal as hal;

use pcf8563::{PCF8563, DateTime} ;

use crate::hal::{i2c::I2c, delay::Delay, pac, prelude::*};

use cortex_m_rt::entry;

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!() ;
    let p = pac::Peripherals::take().unwrap() ;

    let cp =cortex_m::Peripherals::take().unwrap() ;

    let mut flash = p.FLASH ;
    let mut rcc = p.RCC.configure().freeze(&mut flash) ;

    let mut delay = Delay::new(cp.SYST, &rcc);

    let gpiob = p.GPIOB.split(&mut rcc) ;

    let (sda, scl) = cortex_m::interrupt::free(move |cs| {
                    (gpiob.pb9.into_alternate_af1(cs),
                    
                    gpiob.pb8.into_alternate_af1(cs))
            } ) ;
    
    let i2c = I2c::i2c1(p.I2C1, (scl, sda), 100.khz(), &mut rcc) ;
    
    let mut rtc = PCF8563::new(i2c) ;

    loop {
        match rtc.get_datetime() {
            Ok(datetime) => {
                rprintln!("{}-{:02}-{:02} {:02}:{:02}:{:02}", datetime.year, datetime.mon, datetime.mday, datetime.hour, datetime.min, datetime.sec);
            },
            Err(error) => {
                rprintln!("Could not read pcf8563 due to error: {:?}", error);
            }
        }
        delay.delay_ms(1_000_u16) ;
    }
}
