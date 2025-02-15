use std::ops::Deref;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermApplication {
    m: TermItd,
    n: TermItd,
    ty_itd: Option<Ty>,
}

impl Into<Term> for TermApplication {
    fn into(self) -> Term {
        Term::Application(self)
    }
}

impl TermApplication {
    pub fn ty_itd(&self) -> Option<Ty> {
        self.ty_itd
    }

    pub fn m(&self) -> TermItd {
        self.m
    }

    pub fn n(&self) -> TermItd {
        self.n
    }

    // pub(crate) fn ty_term(&self) -> TermCow {
    //     if let Some(ty_itd) = self.ty_itd {
    //         ty_itd.term().into()
    //     } else {
    //         match self.m.deref() {
    //             Term::Atom(a) => match a.variant() {
    //                 TermAtomVariant::Literal(_) => todo!(),
    //                 TermAtomVariant::Variable { variable_variant } => todo!(),
    //                 TermAtomVariant::Entity {} => todo!(),
    //                 TermAtomVariant::Category { category_kind } => {
    //                     self.n.as_universe().unwrap().next().into()
    //                 },
    //                 TermAtomVariant::Universe(_) => todo!(),
    //             },
    //             Term::Curry(_) => todo!(),
    //             Term::Abstraction(_) => todo!(),
    //             Term::Application(_) => todo!(),
    //         }
    //     }
    // }

    pub fn new(m: TermItd, n: TermItd) -> TermResult<Self> {
        if m.ty_itd().is_none() {
            match m.deref() {
                Term::Atom(a) => match a.variant() {
                    TermAtomVariant::Category(category_kind) => match n.deref() {
                        Term::Atom(b) => match b.variant() {
                            TermAtomVariant::Literal(_) => todo!(),
                            TermAtomVariant::Variable { variable_variant } => todo!(),
                            TermAtomVariant::Entity { .. } => todo!(),
                            TermAtomVariant::Category(category_kind) => todo!(),
                            TermAtomVariant::Universe(_) => Ok(Self { m, n, ty_itd: None }),
                        },
                        Term::Curry(_) => todo!(),
                        Term::Abstraction(_) => todo!(),
                        Term::Application(_) => todo!(),
                        Term::Subentity(_) => todo!(),
                        Term::TraitImpl(_) => todo!(),
                    },
                    TermAtomVariant::Universe(_) => todo!(),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        } else {
            todo!()
        }
    }
}

impl<'a> TermContext<'a> {
    pub(crate) fn sort(&self, universe: TermUniverse) -> TermItd {
        self.it_term(
            TermApplication {
                m: self.it_term(TermCategory::Sort.into()),
                n: self.it_term(universe.into()),
                ty_itd: None,
            }
            .into(),
        )
    }
}

impl std::fmt::Display for TermApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
