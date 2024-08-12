pub use logos::*;

#[derive(Clone, Copy, Debug, PartialEq, Logos)]
#[logos(skip r"\/\/[^\n]\n")]
#[logos(skip r"[ \t\r\n]+")]
pub enum Token {
    #[token("let")]
    Let,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Divide,
    #[token("^")]
    Xor,
    #[token("!")]
    Not,
    #[token("&")]
    And,
    #[token("&&")]
    AndAnd,
    #[token("|")]
    Or,
    #[token("||")]
    OrOr,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    Ne,
    #[token("<")]
    Lt,
    #[token("<=")]
    Le,
    #[token(">")]
    Gt,
    #[token(">=")]
    Ge,

    #[token(";")]
    Semicolon,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")] // TODO: better name
    LeftSquare,
    #[token("]")]
    RightSquare,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,

    // TODO: underscore ignoring
    #[regex(r"[0-9_]+", callback = |lex| lex.slice().parse::<u128>().unwrap())]
    #[regex(r"0x[0-9a-fA-F_]+", callback = |lex| u128::from_str_radix(&lex.slice()[2..], 16).unwrap())]
    #[regex(r"0b[01_]+", callback = |lex| u128::from_str_radix(&lex.slice()[2..], 2).unwrap())]
    #[regex(r"0o[0-7_]+", callback = |lex| u128::from_str_radix(&lex.slice()[2..], 8).unwrap())]
    Integer(u128),

    #[regex(r"[0-9]*\.[0-9]+", callback = |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[regex(r"[a-zA-Z_][0-9a-zA-Z_]*", priority = 3)]
    Ident,
}

pub enum SplitIter {
    Once(core::iter::Once<Token>),
    Multi(core::slice::Iter<'static, Token>),
}

impl Token {
    pub fn split_tokens(self) -> SplitIter {
        match self {
            Self::AndAnd => SplitIter::Multi([Self::And; 2].iter()),
            Self::OrOr => SplitIter::Multi([Self::Or; 2].iter()),
            Self::Eq => SplitIter::Multi([Self::Assign; 2].iter()),
            Self::Ne => SplitIter::Multi([Self::Not, Self::Assign].iter()),
            Self::Le => SplitIter::Multi([Self::Lt, Self::Assign].iter()),
            Self::Ge => SplitIter::Multi([Self::Gt, Self::Assign].iter()),
            _ => SplitIter::Once(core::iter::once(self)),
        }
    }

    pub fn len(self) -> Option<usize> {
        match self {
            Self::Let => Some(3),

            Self::Plus => Some(1),
            Self::Minus => Some(1),
            Self::Asterisk => Some(1),
            Self::Divide => Some(1),
            Self::Xor => Some(1),
            Self::Not => Some(1),
            Self::And => Some(1),
            Self::AndAnd => Some(2),
            Self::Or => Some(1),
            Self::OrOr => Some(2),
            Self::Assign => Some(1),
            Self::Eq => Some(2),
            Self::Ne => Some(2),
            Self::Lt => Some(1),
            Self::Le => Some(2),
            Self::Gt => Some(1),
            Self::Ge => Some(2),

            Self::Semicolon => Some(1),
            Self::LeftParen => Some(1),
            Self::RightParen => Some(1),
            Self::LeftSquare => Some(1),
            Self::RightSquare => Some(1),
            Self::LeftBrace => Some(1),
            Self::RightBrace => Some(1),

            Self::Integer(_) | Self::Float(_) | Self::Ident => None,
        }
    }
}

impl Iterator for SplitIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Once(i) => i.next(),
            Self::Multi(i) => i.next().copied(),
        }
    }
}


#[cfg(test)]
#[test]
fn test_lexer() {
    let src = "let let0=1==+!0xaB-0o0456*34/^&|&&||";
    let expected = [
        Token::Let,
        Token::Ident,
        Token::Assign,
        Token::Integer(1),
        Token::Eq,
        Token::Plus,
        Token::Not,
        Token::Integer(0xab),
        Token::Minus,
        Token::Integer(0o456),
        Token::Asterisk,
        Token::Integer(34),
        Token::Divide,
        Token::Xor,
        Token::And,
        Token::Or,
        Token::AndAnd,
        Token::OrOr,
    ];

    for (l, e) in Token::lexer(src).zip(expected) {
        assert_eq!(l, Ok(e));
    }
}
