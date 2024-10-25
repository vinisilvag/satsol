use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("received format `{0}` but only cnf is supported")]
    InputFormatNotSupported(String),

    #[error("header poorly formatted")]
    HeaderPoorlyFormatted,

    #[error("clause readed before header")]
    ClauseBeforeHeader,

    #[error("an unknown occurred")]
    Unknown,
}
