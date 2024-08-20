use crate::error::{VMError, VMResult};

/// For byte memory trait.
/// read1 will read the n'th byte from the address. The input must by 0 <= n < 4.
pub trait Memory {
    fn read(&self, address: u32) -> VMResult<u32>;
    fn write(&mut self, address: u32, value: u32) -> VMResult<()>;

    fn read1(&self, address: u32, byte: u8) -> VMResult<u8> {
        if byte >= 4 {
            return Err(VMError::FailedToReadMemory(address));
        }
        let value = self.read(address)?;
        let shift = byte * 8;
        Ok((value >> shift) as u8)
    }

    fn write1(&mut self, address: u32, byte: u8, value: u8) -> VMResult<()> {
        if byte >= 4 {
            return Err(VMError::FailedToWriteMemory(address));
        }
        let shift = byte * 8;
        let mask = 0xFF << shift;
        let value = (value as u32) << shift;
        let old_value = self.read(address)?;
        self.write(address, (old_value & !mask) | (value & mask))
    }

    fn read2(&self, address: u32, half: u8) -> VMResult<u16> {
        if half >= 2 {
            return Err(VMError::FailedToReadMemory(address));
        }
        let value = self.read(address)?;
        let shift = half * 16;
        Ok((value >> shift) as u16)
    }

    fn write2(&mut self, address: u32, half: u8, value: u16) -> VMResult<()> {
        if half >= 2 {
            return Err(VMError::FailedToWriteMemory(address));
        }
        let shift = half * 16;
        let mask = 0xFFFF << shift;
        let value = (value as u32) << shift;
        let old_value = self.read(address)?;
        self.write(address, (old_value & !mask) | (value & mask))
    }

    fn read4(&self, address: u32) -> VMResult<u32> {
        self.read(address)
    }

    fn write4(&mut self, address: u32, value: u32) -> VMResult<()> {
        self.write(address, value)
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
}

impl<T: AsRef<[u32]> + AsMut<[u32]>> Memory for T {
    fn read(&self, address: u32) -> VMResult<u32> {
        self.as_ref()
            .get(address as usize)
            .copied()
            .ok_or(VMError::FailedToReadMemory(address))
    }

    fn write(&mut self, address: u32, value: u32) -> VMResult<()> {
        if address as usize >= self.as_ref().len() {
            return Err(VMError::FailedToWriteMemory(address));
        }
        let slice = self.as_mut();
        slice[address as usize] = value;
        Ok(())
    }
}
