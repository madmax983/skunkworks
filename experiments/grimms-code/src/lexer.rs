use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    // Use callback to handle greedy matching safely
    #[regex(r"//", line_comment, priority = 5)]
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", priority = 5)]
    Comment,

    // Keywords - must come before Ident
    #[token("fn")]
    #[token("struct")]
    #[token("enum")]
    #[token("union")]
    #[token("impl")]
    #[token("trait")]
    #[token("type")]
    #[token("let")]
    #[token("mut")]
    #[token("const")]
    #[token("static")]
    #[token("pub")]
    #[token("use")]
    #[token("mod")]
    #[token("match")]
    #[token("if")]
    #[token("else")]
    #[token("loop")]
    #[token("while")]
    #[token("for")]
    #[token("in")]
    #[token("return")]
    #[token("break")]
    #[token("continue")]
    #[token("crate")]
    #[token("super")]
    #[token("self")]
    #[token("Self")]
    #[token("where")]
    #[token("unsafe")]
    #[token("async")]
    #[token("await")]
    #[token("dyn")]
    #[token("move")]
    #[token("ref")]
    #[token("box")]
    #[token("extern")]
    Keyword,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    #[regex(r#""([^"\\]|\\.)*""#)]
    String,

    #[regex(r"'([^'\\]|\\.)'")]
    Char,

    #[regex(r"[0-9][0-9_]*(\.[0-9][0-9_]*)?([eE][+-]?[0-9_]+)?(f32|f64|i32|u32|i64|u64|isize|usize)?")]
    Number,

    // Punctuation
    #[token(".")]
    #[token(",")]
    #[token(";")]
    #[token(":")]
    #[token("::")]
    #[token("->")]
    #[token("=>")]
    #[token("#")]
    #[token("!")]
    #[token("?")]
    #[token("{")]
    #[token("}")]
    #[token("[")]
    #[token("]")]
    #[token("(")]
    #[token(")")]
    #[token("<")]
    #[token(">")]
    #[token("=")]
    #[token("==")]
    #[token("!=")]
    #[token("<=")]
    #[token(">=")]
    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("/")]
    #[token("%")]
    #[token("&")]
    #[token("|")]
    #[token("^")]
    #[token("<<")]
    #[token(">>")]
    #[token("&&")]
    #[token("||")]
    #[token("+=")]
    #[token("-=")]
    #[token("*=")]
    #[token("/=")]
    #[token("%=")]
    #[token("&=")]
    #[token("|=")]
    #[token("^=")]
    #[token("<<=")]
    #[token(">>=")]
    Punct,

    // Catch-all for single characters that didn't match anything else
    #[regex(r".", priority = 0)]
    Unknown,
}

fn line_comment(lex: &mut logos::Lexer<Token>) {
    let remainder = lex.remainder();
    if let Some(pos) = remainder.find('\n') {
        lex.bump(pos);
    } else {
        lex.bump(remainder.len());
    }
}
