use husky_primitive_literal_syntax::RawLiteralData;
use husky_text::RangedCustomIdentifier;
use husky_word::{Keyword, LiasonKeyword, Paradigm};

use super::*;

pub struct OfTokenKindPattern(pub TokenKind);
impl std::fmt::Display for OfTokenKindPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "token of kind ".fmt(f)?;
        self.0.fmt(f)
    }
}
impl AtomParserPattern for OfTokenKindPattern {
    type Output = ();

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        if let Some(Token { kind, .. }) = parser.token_stream.next() {
            if *kind == self.0 {
                return Ok(Some(()));
            }
        }
        Ok(None)
    }
}

pub struct ParadigmPattern;
impl std::fmt::Display for ParadigmPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "paradigm keyword".fmt(f)
    }
}
impl AtomParserPattern for ParadigmPattern {
    type Output = Paradigm;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        Ok(
            if let Some(Token {
                kind: TokenKind::Keyword(Keyword::Paradigm(paradigm)),
                ..
            }) = parser.token_stream.next()
            {
                Some(*paradigm)
            } else {
                None
            },
        )
    }
}

pub struct UsizeLiteralPattern;
impl std::fmt::Display for UsizeLiteralPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "usize literal".fmt(f)
    }
}
impl AtomParserPattern for UsizeLiteralPattern {
    type Output = usize;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        Ok(
            if let Some(Token {
                kind: TokenKind::Literal(data),
                ..
            }) = parser.token_stream.next()
            {
                match data {
                    RawLiteralData::Void => todo!(),
                    RawLiteralData::I32(i) => {
                        if *i < 0 {
                            None
                        } else {
                            Some(*i as usize)
                        }
                    }
                    RawLiteralData::Integer(i) => {
                        if *i < 0 {
                            None
                        } else {
                            Some(*i as usize)
                        }
                    }
                    RawLiteralData::I64(_) => todo!(),
                    RawLiteralData::Float(_) => None,
                    RawLiteralData::F32(_) => todo!(),
                    RawLiteralData::F64(_) => todo!(),
                    RawLiteralData::Bits(_) => todo!(),
                    RawLiteralData::B32(_) => todo!(),
                    RawLiteralData::B64(_) => todo!(),
                    RawLiteralData::Bool(_) => todo!(),
                }
            } else {
                None
            },
        )
    }
}

pub struct BeSpecialTokenPattern(pub SpecialToken);
impl std::fmt::Display for BeSpecialTokenPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AtomParserPattern for BeSpecialTokenPattern {
    type Output = ();

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        OfTokenKindPattern(self.0.into()).get_parsed(parser)
    }
}

#[macro_export]
macro_rules! be_special_token_patt {
    ($s: tt) => {{
        BeSpecialTokenPattern(husky_token::special_token!($s))
    }};
}

impl<'a, 'b, 'c> AtomParser<'a, 'b, 'c> {
    pub fn special(&mut self, target: SpecialToken) -> Option<()> {
        self.token_kind(target.into())
    }

    pub fn paradigm(&mut self) -> Option<Paradigm> {
        if let Some(Token {
            kind: TokenKind::Keyword(Keyword::Paradigm(paradigm)),
            ..
        }) = self.token_stream.next()
        {
            Some(*paradigm)
        } else {
            None
        }
    }

    pub fn usize_literal(&mut self) -> Option<usize> {
        if let Some(Token {
            kind: TokenKind::Literal(data),
            ..
        }) = self.token_stream.next()
        {
            match data {
                RawLiteralData::Void => todo!(),
                RawLiteralData::I32(i) => {
                    if *i < 0 {
                        None
                    } else {
                        Some(*i as usize)
                    }
                }
                RawLiteralData::Integer(i) => {
                    if *i < 0 {
                        None
                    } else {
                        Some(*i as usize)
                    }
                }
                RawLiteralData::I64(_) => todo!(),
                RawLiteralData::Float(_) => None,
                RawLiteralData::F32(_) => todo!(),
                RawLiteralData::F64(_) => todo!(),
                RawLiteralData::Bits(_) => todo!(),
                RawLiteralData::B32(_) => todo!(),
                RawLiteralData::B64(_) => todo!(),
                RawLiteralData::Bool(_) => todo!(),
            }
        } else {
            None
        }
    }

    pub fn custom_ident(&mut self) -> Option<RangedCustomIdentifier> {
        if let Some(Token {
            kind: TokenKind::Identifier(Identifier::Custom(ident)),
            range,
        }) = self.token_stream.next()
        {
            Some(RangedCustomIdentifier {
                ident: *ident,
                range: *range,
            })
        } else {
            None
        }
    }

    pub fn sema_custom_ident(
        &mut self,
        semantic_token_kind: SemanticTokenKind,
    ) -> Option<RangedCustomIdentifier> {
        if let Some(Token {
            kind: TokenKind::Identifier(Identifier::Custom(ident)),
            range,
        }) = self.token_stream.next()
        {
            self.atom_context
                .push_abs_semantic_token(AbsSemanticToken::new(semantic_token_kind, *range));
            Some(RangedCustomIdentifier {
                ident: *ident,
                range: *range,
            })
        } else {
            None
        }
    }

    pub fn token_kind(&mut self, target: TokenKind) -> Option<()> {
        if let Some(Token { kind, .. }) = self.token_stream.next() {
            if *kind == target {
                return Some(());
            }
        }
        None
    }

    pub fn liason(&mut self) -> Option<LiasonKeyword> {
        if let Some(Token { kind, .. }) = self.token_stream.next() {
            match kind {
                TokenKind::Keyword(Keyword::Liason(liason_keyword)) => {
                    return Some(*liason_keyword)
                }
                _ => (),
            }
        }
        None
    }
}
