use std::ops::Range;

use crate::error::{VMError, VMResult};

/// For byte memory trait.
/// read1 will read the n'th byte from the address. The input must by 0 <= n < 4.
pub trait Memory {
    fn read(&self, address: u32) -> VMResult<u32>;
    fn write(&mut self, address: u32, value: u32) -> VMResult<()>;

    fn read1(&self, address: u32) -> VMResult<u8> {
        let byte = address % 4;
        let address = address - byte;

        let value = self.read(address)?;
        let shift = byte * 8;

        Ok((value >> shift) as u8)
    }

    fn write1(&mut self, address: u32, value: u8) -> VMResult<()> {
        let byte = address % 4;
        let address = address - byte;

        let shift = byte * 8;
        let mask = 0xFF << shift;

        let value = (value as u32) << shift;
        let old_value = self.read(address)?;

        self.write(address, (old_value & !mask) | (value & mask))
    }

    fn read2(&self, address: u32) -> VMResult<u16> {
        let byte = address % 4;
        let address = address - byte;

        // In case we are reading the last byte of the memory we need to make two call to self.read
        match byte {
            3 => {
                let low = self.read(address)?;
                let high = self.read(address + 4)?;
                Ok(((low << 8) | (high >> 24) & 0xff) as u16)
            }
            _ => {
                let value = self.read(address)?;
                let shift = byte * 8;
                Ok((value >> shift) as u16)
            }
        }
    }

    fn write2(&mut self, address: u32, value: u16) -> VMResult<()> {
        let byte = address % 4;
        let address = address - byte;

        match byte {
            3 => {
                let low = self.read(address)?;
                let high = self.read(address + 4)?;
                let low_value = 0xffffff00 & low;
                let high_value = 0x00ffffff & high;
                let low_value = low_value | ((value as u32) >> 8);
                let high_value = high_value | ((value as u32) << 24);
                self.write(address, low_value)
                    .and_then(|_| self.write(address + 4, high_value))
            }
            _ => {
                let shift = byte * 8;
                let mask = 0xFFFF << shift;
                let value = (value as u32) << shift;
                let old_value = self.read(address)?;
                self.write(address, (old_value & !mask) | (value & mask))
            }
        }
    }

    fn read4(&self, address: u32) -> VMResult<u32> {
        let bytes = address % 4;
        let address = address - bytes;
        match bytes {
            0 => self.read(address),
            _ => {
                let low = self.read(address)? as u64;
                let high = self.read(address + 4)? as u64;
                let shift = bytes * 8;
                Ok(((high << (32 - shift)) | (low >> shift)) as u32)
            }
        }
    }

    fn write4(&mut self, address: u32, value: u32) -> VMResult<()> {
        let bytes = address % 4;
        let address = address - bytes;
        match bytes {
            0 => self.write(address, value),
            _ => {
                let low = self.read(address)? as u64;
                let high = self.read(address + 4)? as u64;
                let shift = bytes * 8;
                let mask = 0xFFFFFFFF << shift;
                let value = (value as u64) << shift;
                let low = (low & !mask) | (value & mask);
                let high = (high & !(0xFFFFFFFF << (32 - shift))) | (value >> (32 - shift));
                self.write(address, low as u32)
                    .and_then(|_| self.write(address + 4, high as u32))
            }
        }
    }

    fn read8(&self, address: u32) -> VMResult<u64> {
        let low = self.read4(address)? as u64;
        let high = self.read4(address + 4)? as u64;
        Ok((high << 32) | low)
    }

    fn write8(&mut self, address: u32, value: u64) -> VMResult<()> {
        self.write4(address, value as u32)?;
        self.write4(address + 4, (value >> 32) as u32)
    }

    fn dump(&self, range: Range<usize>) {
        for i in range {
            match self.read(i as u32) {
                Ok(value) => println!("{:08x}: {:08x}", i, value),
                Err(_) => break,
            }
        }
    }
}

impl<T: AsRef<[u32]> + AsMut<[u32]>> Memory for T {
    fn read(&self, address: u32) -> VMResult<u32> {
        assert_eq!(address % 4, 0);
        let address = address / 4;

        self.as_ref()
            .get(address as usize)
            .copied()
            .ok_or(VMError::FailedToReadMemory(address))
    }

    fn write(&mut self, address: u32, value: u32) -> VMResult<()> {
        assert_eq!(address % 4, 0);
        let address = address / 4;

        if address as usize >= self.as_ref().len() {
            return Err(VMError::FailedToWriteMemory(address));
        }
        let slice = self.as_mut();
        slice[address as usize] = value;
        Ok(())
    }
}
