#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ArenaError {
    ZeroCapacity,
    InvalidLayout,
    AllocationFailed,
}

impl std::fmt::Display for ArenaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArenaError::ZeroCapacity => write!(f, "Zero capacity"),
            ArenaError::InvalidLayout => write!(f, "Invalid layout"),
            ArenaError::AllocationFailed => write!(f, "Allocation failed"),
        }
    }
}

impl std::error::Error for ArenaError {}
