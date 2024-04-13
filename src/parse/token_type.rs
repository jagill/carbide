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

    #[regex(r"[a-zA-Z_]+")]
    Identifier,
    UnknownToken,
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

    #[test]
    fn test_bool_keywords() {
        let mut lex = TokenType::lexer(
            r"
        bool true or false
        and not

        ",
        );
        let mut output = Vec::new();
        while let Some(res) = lex.next() {
            match res.unwrap() {
                Newline | Whitespace => continue,
                token => output.push(token),
            }
        }
        assert_eq!(output, vec![Bool, True, Or, False, And, Not]);
    }
}
