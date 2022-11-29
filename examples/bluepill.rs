#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal as _ ;

#[entry]
fn main() -> ! {
    loop {
        
    }
}
