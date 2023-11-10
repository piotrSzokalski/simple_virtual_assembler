/// Possible states of Flag register
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Flag {
    ZERO,
    EQUAL,
    GREATER,
    LESSER,
}