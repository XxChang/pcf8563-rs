#![no_std]

mod datetime;
mod configuration;

extern crate embedded_hal as hal;

use hal::blocking::i2c::{Read, Write, WriteRead};

const ADDRESS: u8 = 0x51 ;

pub struct PCF8563<I2C> {
    i2c: I2C,
}

macro_rules! Mask {
    ($t: ident) => {
        impl $t {
            #[inline]
            fn mask(&self) -> u8 {
                *self as u8
            }
        }        
    };
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum CTRL1 {
    IGN   = 0b0000_0000, 
    TEST1 = 0b1000_0000,
    STOP  = 0b0010_0000,
    TESTC = 0b0000_1000,
}

Mask!(CTRL1) ;

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum CTRL2 {
    IGN   = 0b0000_0000,
    TP  = 0b0001_0000,
    AF  = 0b0000_1000,
    TF  = 0b0000_0100,
    AIE = 0b0000_0010,
    TIE = 0b0000_0001,
}

Mask!(CTRL2) ;

impl<I2C, E> PCF8563<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        PCF8563 { i2c }
    }

    pub fn destory(self) -> I2C {
        self.i2c
    }
}

/// Date and Time
pub struct DateTime {
    pub sec: u8,
    pub min: u8,
    pub hour: u8,
    pub mday: u8,
    pub wday: u8,
    pub mon: u8,
    pub year: u16,
}

impl Default for DateTime {
    fn default() -> Self {
        DateTime { sec: 0, min: 0, hour: 0, mday: 0, wday: 0, mon: 0, year: 1970 }
    }
}
