use thiserror::Error;

#[derive(Error, Debug)]
pub enum WineError {
    #[error("Memory allocation failed")]
    MemoryAllocation,
    
    #[error("Invalid memory address: {0}")]
    InvalidAddress(u64),
    
    #[error("Thread operation failed: {0}")]
    ThreadError(String),
    
    #[error("System call failed: {0}")]
    SyscallError(String),
    
    #[error("File operation failed: {0}")]
    FileError(String),
    
    #[error("Invalid PE file format")]
    InvalidPEFormat,
    
    #[error("DLL not found: {0}")]
    DllNotFound(String),
    
    #[error("Graphics operation failed: {0}")]
    GraphicsError(String),
    
    #[error("Security violation: {0}")]
    SecurityError(String),
}

pub type WineResult<T> = Result<T, WineError>; 