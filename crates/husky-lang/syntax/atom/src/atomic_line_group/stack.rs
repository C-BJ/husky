use common::*;

use scope::{GenericArgument, ScopeKind};
use word::BuiltinIdentifier;

use crate::{convexity::Convexity, error::atom_error, scope_proxy::ScopeProxy, *};

pub(crate) struct AtomStack {
    atoms: Vec<Atom>,
}

impl Into<Vec<Atom>> for AtomStack {
    fn into(self) -> Vec<Atom> {
        self.atoms
    }
}

// get
impl AtomStack {
    pub fn new() -> Self {
        Self { atoms: Vec::new() }
    }

    pub(crate) fn convexity(&self) -> Convexity {
        if let Some(atom) = self.atoms.last() {
            convexity::right_side_convexity(&atom.kind)
        } else {
            Convexity::Concave
        }
    }

    pub(crate) fn is_convex(&self) -> bool {
        self.convexity() == Convexity::Convex
    }

    pub(crate) fn is_concave(&self) -> bool {
        self.convexity() == Convexity::Concave
    }
}

// push
impl AtomStack {
    pub(crate) fn push(&mut self, atom: Atom) -> Result<(), AtomError> {
        if convexity::compatible(self.convexity(), convexity::left_side_convexity(&atom.kind)) {
            self.atoms.push(atom);
            Ok(())
        } else {
            atom_err!(atom.text_range(), AtomRule::CompatibleConvexity,)
        }
    }

    pub(crate) fn end_list(&mut self, ket: Bracket, attr: ListEndAttr, ket_range: TextRange) {
        if self.is_convex() {
            self.push(Atom::new(ket_range.clone(), AtomKind::ListItem))
                .unwrap();
        }
        self.push(Atom::new(ket_range, AtomKind::ListEnd(ket, attr)))
            .unwrap();
    }

    pub(crate) fn end_list_or_make_type(
        &mut self,
        ket: Bracket,
        attr: ListEndAttr,
        mut tail: TextRange,
        scope_proxy: ScopeProxy,
    ) -> Result<(), AtomError> {
        match (ket, self.atoms.last()) {
            (
                Bracket::Par,
                Some(Atom {
                    kind: AtomKind::Scope(_, ScopeKind::Type),
                    ..
                }),
            ) => {
                let (attr, mut generics) = self.pop_par_list_of_types(&mut tail)?;
                let ident = match attr {
                    ListStartAttr::None => BuiltinIdentifier::Tuple,
                    ListStartAttr::Attach => {
                        generics.push(ScopeId::Builtin(BuiltinIdentifier::Void).into());
                        self.func_generic(attr)?
                    }
                };
                self.push(scope_proxy.builtin_type_atom(ident, generics, tail))
            }
            _ => Ok(self.end_list(ket, attr, tail)),
        }
    }

    pub(crate) fn start_list(&mut self, bra: Bracket, text_range: TextRange) {
        self.push(Atom::new(
            text_range,
            AtomKind::ListStart(
                bra,
                if self.is_convex() {
                    ListStartAttr::Attach
                } else {
                    ListStartAttr::None
                }
                .into(),
            ),
        ))
        .unwrap();
    }

    fn func_generic(&mut self, attr: ListStartAttr) -> AtomResult<BuiltinIdentifier> {
        let expectation = "expect Fp, Fn, FnMut, FnOnce";

        match attr {
            ListStartAttr::None => Ok(word::default_func_type()),
            ListStartAttr::Attach => {
                let last_atom = self.atoms.pop().unwrap();
                match last_atom.kind {
                    AtomKind::Scope(ScopeId::Builtin(ident), _) => match ident {
                        BuiltinIdentifier::Fp
                        | BuiltinIdentifier::Fn
                        | BuiltinIdentifier::FnMut
                        | BuiltinIdentifier::FnOnce => Ok(ident),
                        _ => atom_err!(last_atom.text_range(), expectation),
                    },
                    _ => atom_err!(last_atom.text_range(), expectation),
                }
            }
        }
    }

    fn pop(&mut self, follower: &mut TextRange) -> AtomResult<Atom> {
        let atom = self
            .atoms
            .pop()
            .ok_or(atom_error!(follower.clone(), "something before it"))?;
        *follower = atom.to(follower);
        Ok(atom)
    }

    fn pop_par_list_of_types(
        &mut self,
        tail: &mut TextRange,
    ) -> AtomResult<(ListStartAttr, Vec<GenericArgument>)> {
        let mut types = Vec::new();
        match self.pop(tail)?.kind {
            AtomKind::ListStart(Bracket::Par, attr) => return Ok((attr, Vec::new())),
            AtomKind::Scope(scope, ScopeKind::Type) => types.push(scope.into()),
            _ => atom_err!(tail, "left parenthesis or type")?,
        };
        loop {
            match self.pop(tail)?.kind {
                AtomKind::ListStart(Bracket::Par, attr) => {
                    types.reverse();
                    return Ok((attr, types));
                }
                AtomKind::ListItem => (),
                _ => atom_err!(tail, "left parenthesis or comma")?,
            }
            match self.pop(tail)?.kind {
                AtomKind::Scope(scope, ScopeKind::Type) => types.push(scope.into()),
                _ => atom_err!(tail, "type")?,
            }
        }
    }

    pub(crate) fn make_func_type(
        &mut self,
        scope_proxy: ScopeProxy,
        output: ScopeId,
        mut tail: TextRange,
    ) -> Result<(), AtomError> {
        let (attr, mut generics) = self.pop_par_list_of_types(&mut tail)?;
        generics.push(output.into());
        let func_type = self.func_generic(attr)?;
        self.push(scope_proxy.builtin_type_atom(func_type, generics, tail))
    }
}
