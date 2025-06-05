//! The discrete tokens of the bottoms up language
//!

use std::ops::Range;

pub struct Token<T> {
    pub start_offset: usize,
    pub symbol: Symbol<T>,
}

impl<T> Token<T>
where
    T: AsRef<[u8]>,
{
    pub fn as_range(&self) -> Range<usize> {
        self.start_offset..(self.start_offset + self.symbol.len())
    }
}

pub enum Symbol<T> {
    Keyword(Keyword),
    Ident(T),
    Operator(Operator),
    Integer(T),
    String(T),
    Comment(T),
}

impl<T> Symbol<T>
where
    T: AsRef<[u8]>,
{
    pub fn len(&self) -> usize {
        match self {
            Self::Keyword(k) => k.len(),
            Self::Operator(o) => o.len(),
            Self::String(s) => s.as_ref().len() + 2,
            Self::Comment(s) => s.as_ref().len() + 4,
            Self::Integer(s) | Self::Ident(s) => s.as_ref().len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        false
    }
}

pub enum Keyword {}

impl Keyword {
    pub fn len(&self) -> usize {
        0
    }

    pub fn is_empty(&self) -> bool {
        false
    }
}

pub enum Operator {
    Plus,
    Dash,
    Star,
    ForwardSlash,
    Percent,
    DoubleGt,
    DoubleLt,
    LtGt,
    Gt,
    Lt,
    DoubleEq,
    Eq,
    BangEq,
    // LeftParen,
    // RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    ThinArrow,
}

impl Operator {
    pub fn len(&self) -> usize {
        match self {
            Self::Plus
            | Self::Dash
            | Self::Star
            | Self::ForwardSlash
            | Self::Percent
            | Self::Gt
            | Self::Lt
            | Self::Eq
            // | Self::RightParen
            // | Self::LeftParen 
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket => 1,
            Self::DoubleGt
            | Self::DoubleLt
            | Self::LtGt
            | Self::DoubleEq
            | Self::BangEq
            | Self::ThinArrow => 2,
        }
    }

    pub fn is_empty(&self) -> bool {
        false
    }
}
