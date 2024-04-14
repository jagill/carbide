use logos::Logos;

#[derive(Logos, Copy, Clone, Debug, PartialEq)]
#[logos()]
pub enum TokenType {
    #[token("\n")]
    Newline,
    #[regex(r"[ \t\r\f]+")]
    Whitespace,

    #[token("bool")]
    Bool,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[regex(r"[0-9]+")]
    Int,

    #[token("==")]
    EqualEqual,
    #[token("!=")]
    BangEqual,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,

    #[token("if")]
    If,
    #[token("then")]
    Then,
    #[token("else")]
    Else,

    #[token("_")]
    Underscore,
    #[regex(r"[a-zA-Z_]+")]
    Identifier,
    UnknownToken,
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

    fn assert_tokens(input: &str, tokens: Vec<TokenType>) {
        let mut lex = TokenType::lexer(input);
        let mut output = Vec::new();
        while let Some(res) = lex.next() {
            match res.unwrap() {
                Newline | Whitespace => continue,
                token => output.push(token),
            }
        }
        assert_eq!(output, tokens);
    }

    #[test]
    fn test_bool_keywords() {
        assert_tokens(
            r"
        bool true or false
        and not

        ",
            vec![Bool, True, Or, False, And, Not],
        );
    }

    #[test]
    fn test_underscores() {
        assert_tokens("_", vec![Underscore]);
        assert_tokens("_a", vec![Identifier]);
    }

    #[test]
    fn test_ints() {
        assert_tokens("123", vec![Int]);
        assert_tokens("-123", vec![Minus, Int]);
        assert_tokens("--123", vec![Minus, Minus, Int]);
        assert_tokens("- -123", vec![Minus, Minus, Int]);
        assert_tokens("-+123", vec![Minus, Plus, Int]);
    }
}
