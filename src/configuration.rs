use crate::{CTRL1, CTRL2, PCF8563, ADDRESS} ;
use hal::blocking::i2c::{Read, Write, WriteRead};

impl<I2C, E> PCF8563<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    /// Enable the oscillator (set the clock running) (default).
    pub fn enable(&mut self) -> Result<(), E> {
        self.sc_ctrl1(CTRL1::IGN, CTRL1::STOP)
    }

    pub fn disable(&mut self) -> Result<(), E> {
        self.sc_ctrl1(CTRL1::STOP, CTRL1::IGN)
    }

    pub fn disable_power_on_reset(&mut self) -> Result<(), E> {
        self.sc_ctrl1(CTRL1::IGN, CTRL1::TESTC)
    }

    pub fn enable_power_on_reset(&mut self) -> Result<(), E> {
        self.sc_ctrl1(CTRL1::TESTC, CTRL1::IGN)
    }

    pub fn enable_alarm_interrupts(&mut self) -> Result<(), E> {
        self.sc_ctrl2(CTRL2::AIE, CTRL2::AF)
    }

    pub fn disable_alarm_interrupts(&mut self) -> Result<(), E> {
        self.sc_ctrl2(CTRL2::IGN, CTRL2::AIE)
    }

    fn sc_ctrl1(&mut self, set: CTRL1, clear: CTRL1) -> Result<(), E> {
        let mut ctrl1_stat = [0u8;1] ; 
        let address: u8 = 0x00 ;
        self.i2c.write_read(ADDRESS, &[address], &mut ctrl1_stat)?;
        let ctrl1_stat = (ctrl1_stat[0] & !clear.mask()) | set.mask() ;
        let mut buf = [address, ctrl1_stat] ;
        self.i2c.write(ADDRESS, &mut buf)
    }

    fn sc_ctrl2(&mut self, set: CTRL2, clear: CTRL2) -> Result<(), E> {
        let mut ctrl1_stat = [0u8;1] ; 
        let address: u8 = 0x01 ;
        self.i2c.write_read(ADDRESS, &[address], &mut ctrl1_stat)?;
        let ctrl1_stat = (ctrl1_stat[0] & !clear.mask()) | set.mask() ;
        let mut buf = [address, ctrl1_stat] ;
        self.i2c.write(ADDRESS, &mut buf)
    }

}
