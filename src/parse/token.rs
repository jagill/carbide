use logos::Logos;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexError {
    #[default]
    UnknownToken,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexError)]
pub enum Token {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn test_bool_keywords() {
        let mut lex = Token::lexer(
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
