use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("fn")]
    KwFn,
    #[token("let")]
    KwLet,
    #[token("mut")]
    KwMut,
    #[token("pub")]
    KwPub,
    #[token("struct")]
    KwStruct,
    #[token("enum")]
    KwEnum,
    #[token("impl")]
    KwImpl,
    #[token("use")]
    KwUse,
    #[token("mod")]
    KwMod,
    #[token("return")]
    KwReturn,
    #[token("if")]
    KwIf,
    #[token("else")]
    KwElse,
    #[token("match")]
    KwMatch,
    #[token("for")]
    KwFor,
    #[token("while")]
    KwWhile,
    #[token("loop")]
    KwLoop,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_string())]
    StringLiteral(String),

    #[regex(r"[0-9]+", |lex| lex.slice().to_string())]
    NumberLiteral(String),

    // Punctuation
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(";")]
    Semi,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("=")]
    Eq,
    #[token("->")]
    Arrow,
    #[token("::")]
    PathSep,
    #[token("!")]
    Bang,

    // Comments
    #[regex(r"//.*", logos::skip)]
    Comment,
}

pub fn tokenize(input: &str) -> Vec<(Token, std::ops::Range<usize>)> {
    let lexer = Token::lexer(input);
    lexer
        .spanned()
        .filter_map(|(tok, span)| {
            match tok {
                Ok(token) => Some((token, span)),
                Err(_) => None, // Skip errors/unknown tokens for now
            }
        })
        .collect()
}
