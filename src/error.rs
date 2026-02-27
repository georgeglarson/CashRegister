use thiserror::Error;

#[derive(Debug, Error)]
pub enum CashRegisterError {
    #[error("line {line}: invalid dollar amount \"{input}\"")]
    InvalidAmount { line: usize, input: String },

    #[error("line {line}: paid ({paid}) is less than owed ({owed})")]
    Underpayment {
        line: usize,
        owed: String,
        paid: String,
    },

    #[error("line {line}: {detail}")]
    MalformedLine { line: usize, detail: String },

    #[error("{0}")]
    Io(#[from] std::io::Error),
}
