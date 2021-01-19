pub mod mdbook;

#[derive(Error, Debug)]
pub enum LalrdocError {
    #[error("cannot read grammar: {0}")]
    CannotReadGrammar(std::io::Error),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("parse error; run lalrpop for more")]
    ParseError,
}

pub trait ReferenceBuilder {
    fn build_reference(&self) -> Result<(), LalrdocError>;
}
