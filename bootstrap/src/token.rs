use std::fmt::Debug;

#[derive(Clone)]
pub enum Token<'a> {
    Function(&'a str),
    Variable(&'a str),
    Illegal(&'a str),
    Eof(&'a str),
    Ident(&'a str),
    Int(&'a str),
    Assign(&'a str),
    Plus(&'a str),
    Minus(&'a str),
    Exp(&'a str),
    Mul(&'a str),
    Div(&'a str),
    Comma(&'a str),
    Semicolon(&'a str),
    LParen(&'a str),
    RParen(&'a str),
    LBrace(&'a str),
    RBrace(&'a str),
    LCurly(&'a str),
    RCurly(&'a str),
}
impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Self::Illegal(v) => format!("Illegal : {}", v),
            Self::Function(v) => format!("Function: {}", v),
            Self::Variable(v) => format!("Variable: {}", v),
            Self::Eof(v) => format!("Eof : {}", v),
            Self::Ident(v) => format!("Ident : {}", v),
            Self::Int(v) => format!("Int : {}", v),
            Self::Assign(v) => format!("Assign : {}", v),
            Self::Plus(v) => format!("Plus : {}", v),
            Self::Minus(v) => format!("Minus: {}", v),
            Self::Exp(v) => format!("Exponent: {}", v),
            Self::Mul(v) => format!("Multiply: {}", v),
            Self::Div(v) => format!("Divide: {}", v),
            Self::Comma(v) => format!("Comma : {}", v),
            Self::Semicolon(v) => format!("Semicolon : {}", v),
            Self::LParen(v) => format!("LParen : {}", v),
            Self::RParen(v) => format!("RParen : {}", v),
            Self::LBrace(v) => format!("LBrace : {}", v),
            Self::RBrace(v) => format!("RBrace : {}", v),
            Self::LCurly(v) => format!("LCurly : {}", v),
            Self::RCurly(v) => format!("RCurly : {}", v),
        };
        write!(f, "{}", str)
    }
}
