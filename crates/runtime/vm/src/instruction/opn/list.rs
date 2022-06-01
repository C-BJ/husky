use entity_route::SpatialArgument;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ListOpr {
    TupleInit,
    NewVec,
    NewDict,
    Call,
    Index,
    ModuloIndex,
    StructInit,
    MethodCall {
        ranged_ident: RangedCustomIdentifier,
        generic_arguments: Vec<SpatialArgument>,
    },
}

impl Into<RawOpnVariant> for ListOpr {
    fn into(self) -> RawOpnVariant {
        RawOpnVariant::List(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListStartAttr {
    None,
    Attach,
    MethodAttach {
        ranged_ident: RangedCustomIdentifier,
        generic_arguments: Vec<SpatialArgument>,
    },
}

impl ListStartAttr {
    pub fn attached(&self) -> bool {
        match self {
            ListStartAttr::None => false,
            ListStartAttr::Attach => true,
            ListStartAttr::MethodAttach { .. } => true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ListEndAttr {
    None,
    Attach,
    Modulo,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Par,
    Box,
    Angle,
    Curl,
}

impl Bracket {
    pub fn bra_code(&self) -> &'static str {
        match self {
            Bracket::Par => "(",
            Bracket::Box => "[",
            Bracket::Angle => "<",
            Bracket::Curl => "{",
        }
    }

    pub fn ket_code(&self) -> &'static str {
        match self {
            Bracket::Par => ")",
            Bracket::Box => "]",
            Bracket::Angle => ">",
            Bracket::Curl => "}",
        }
    }
}
