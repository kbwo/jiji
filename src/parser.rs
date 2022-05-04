use logos::Logos;

#[derive(Logos, Debug, Copy, Clone, PartialEq)]
pub enum JsonToken {
    // Characters
    #[token("{")]
    OpenCurly,
    #[token("}")]
    CloseCurly,
    #[token("[")]
    OpenSquare,
    #[token("]")]
    CloseSquare,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("null")]
    Null,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r"-?(0|([1-9][0-9]*))(\.[0-9]+)?([eE][-+]?[0-9]+)?")]
    Number,
    #[regex("\"((\\\\([\"\\\\/bfnrt]|u[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]))|[^\"\\\\\x00-\x1F])*\"")]
    String,
    #[token("\n")]
    Newline,
    #[regex("[ \t\r]+", logos::skip)]
    Whitespace,
    #[error]
    Error,
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read};

    use logos::Logos;

    use super::JsonToken;

    #[test]
    fn parse_correct_json() {
        let mut buf = String::new();
        File::open("test/example.package.json")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        let mut tokenizser = JsonToken::lexer(&buf);
        assert!(tokenizser.next().is_some());
    }

    #[test]
    fn validate_bad_json() {
        //unimplemented
    }
}
