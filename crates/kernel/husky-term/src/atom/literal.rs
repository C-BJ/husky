use std::ops::Deref;

use ordered_float::OrderedFloat;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermLiteral(TermItd);

impl std::ops::Deref for TermLiteral {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for TermLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data() {
            TermLiteralData::Void => "void".fmt(f),
            TermLiteralData::I32(v) => v.fmt(f),
            TermLiteralData::I64(v) => v.fmt(f),
            TermLiteralData::Float(v) => v.fmt(f),
            TermLiteralData::F32(v) => v.fmt(f),
            TermLiteralData::F64(v) => v.fmt(f),
            TermLiteralData::Bits(v) => v.fmt(f),
            TermLiteralData::B32(v) => v.fmt(f),
            TermLiteralData::B64(v) => v.fmt(f),
            TermLiteralData::Bool(v) => v.fmt(f),
        }
    }
}

impl TermLiteral {
    pub fn data(&self) -> &TermLiteralData {
        match self.deref() {
            Term::Atom(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::TraitImpl(_) => todo!(),
        }
    }

    pub fn i32_literal(db: &dyn TermDb, i: i32, menu2: &TermMenu2) -> TermItd {
        db.it_term(Term::Atom(TermAtom::new_literal(
            TermLiteralData::I32(i),
            menu2.i32(),
        )))
    }

    pub fn i64_literal(db: &dyn TermDb, i: i64, menu2: &TermMenu2) -> TermItd {
        db.it_term(Term::Atom(TermAtom::new_literal(
            TermLiteralData::I64(i),
            menu2.i64(),
        )))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TermLiteralData {
    Void,
    I32(i32),
    I64(i64),
    Float(OrderedFloat<f64>),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
    Bits(u64),
    B32(u32),
    B64(u64),
    Bool(bool),
}

impl std::fmt::Display for TermLiteralData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
