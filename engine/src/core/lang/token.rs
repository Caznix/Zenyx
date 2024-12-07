#[derive(Clone, Copy)]
pub struct Token<'source> {
    pub kind: TokenKind,
    pub lit: &'source str,
    // pos: Whatever,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Ident,

    IntLit,
    FloatLit,
    StrLit,

    Func,
    Var,
    Const,

    Dollar,
    Percent,
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Equal,
    EqualEqual,
    LF,
}
