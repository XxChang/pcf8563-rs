use crate::{DateTime, PCF8563, ADDRESS} ;
use hal::blocking::i2c::{Read, Write, WriteRead};

impl<I2C, E> PCF8563<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    pub fn get_datetime(&mut self) -> Result<DateTime, E> {
        let mut data = [0u8;7] ;
        self.read_dt_registers(&mut data)?;

        Ok(
            DateTime {
                sec:  bcd2dec( data[0] & 0x7Fu8 ),
                min:  bcd2dec( data[1] & 0x7Fu8 ),
                hour: bcd2dec( data[2] & 0x3Fu8 ),
                mday: bcd2dec( data[3] & 0x3Fu8 ),
                wday: bcd2dec( data[4] & 0x03u8 ),
                mon:  bcd2dec( data[5] & 0x1Fu8 ),
                year: bcd2dec( data[6] ) as u16 + if (data[5] & 0x80) == 0x80 {
                    2000
                }
                else
                {
                    1900
                }
            }
        )
    }

    pub fn set_datetime(&mut self, datetime: DateTime) -> Result<(), E> {
        let mut data = [0u8;8] ;
        let (yy, century) = match datetime.year {
            year if year >= 2000 => (year - 2000, 0x80),
            year => (year - 1900, 0x00),
        };

        data[0] = 0x02u8 ;
        data[1] = dec2bcd(datetime.sec) ;
        data[2] = dec2bcd(datetime.min) ;
        data[3] = dec2bcd(datetime.hour) ;
        data[4] = dec2bcd(datetime.mday) ;
        data[5] = dec2bcd(datetime.wday) ;
        data[6] = dec2bcd(datetime.mon) | century ;
        data[7] = dec2bcd(yy as u8) ;

        self.i2c.write(ADDRESS, &data)
    }

    fn read_dt_registers(&mut self, buf: &mut [u8; 7]) -> Result<(), E> {
        self.i2c.write_read(ADDRESS, &[0x02u8], buf)
    }
}

fn dec2bcd(value: u8) -> u8 {
    (value / 10 * 16) + (value % 10)
}

fn bcd2dec(value: u8) -> u8 {
    (value / 16 * 10) + (value % 16)
}

