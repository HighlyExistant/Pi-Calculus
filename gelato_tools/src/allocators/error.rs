use thiserror::Error;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum AllocError {
    #[error("Not enough size to perform the operation")]
    InsufficientSize,
    #[error("The deallocation does not belong to this allocator")]
    WrongAlloc,
}