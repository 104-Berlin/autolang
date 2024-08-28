use thiserror::Error;

pub type VMResult<T> = std::result::Result<T, VMError>;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("Failed to read memory at {0:X}")]
    FailedToReadMemory(u32),

    #[error("Failed to write memory at {0:X}")]
    FailedToWriteMemory(u32),

    #[error("Failed to parse from instruction {0:X}")]
    FailedParsingInstruction(u32),

    #[error("Invalid opcode {0:X}")]
    InvalidOpCode(u8),

    #[error("Invalid register {0:X}")]
    InvalidRegister(u8),

    #[error("Label not found '{0}'")]
    LabelNotFound(String),

    #[error("Block already defined '{0}'")]
    BlockAlreadyDefined(usize),

    #[error("Unresolved label '{0}'")]
    UnresolvedLabel(String),

    #[error("Block ({0}) not found")]
    BlockNotFound(usize),
}
