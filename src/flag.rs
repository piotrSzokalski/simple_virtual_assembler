/// Possible states of Flag register
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Flag {
    ZERO,
    EQUAL,
    GREATER,
    LESSER,
}