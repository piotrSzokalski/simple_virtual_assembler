// ___________________________________________

// TODO Move to different file and make it work

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// Types of registers, value is an index
pub enum Register {
    /// General purpose register used to store values
    GENERAL(usize),
    /// Register used for I/O
    PORT(usize)
}

impl Register {
    //pub fn C
}

// __________________________________________
