mod error;
mod token_type;
mod tokenizer;

pub use token_type::TokenType;
pub use tokenizer::Token;
pub use tokenizer::TokenData;

type ParseResult<T> = Result<T, error::ParseError>;
