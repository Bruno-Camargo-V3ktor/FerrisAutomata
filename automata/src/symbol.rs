#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Symbol {
    Empty,
    Letter(char),
}
