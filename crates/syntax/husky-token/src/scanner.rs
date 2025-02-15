use std::{iter::Peekable, sync::Arc};

use crate::{raw_token_iter::RawTokenIter, tokenized_text::TokenGroup, *};

use husky_dev_utils::dev_src;
use husky_file::URange;
use husky_print_utils::p;
use husky_text::TextIndent;
use husky_word::WordInterner;
use wild_utils::ref_to_mut_ref;

#[derive(PartialEq, Eq)]
pub struct TokenizedLine {
    pub(crate) indent: TextIndent,
    pub(crate) tokens: URange,
}

impl std::fmt::Debug for TokenizedLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "TokenizedLine{{Indent({:?}), tokens: {:?}}}",
            &self.indent.get_raw(),
            &self.tokens,
        ))
    }
}

pub(crate) struct TokenScanner<'lex> {
    word_interner: &'lex WordInterner,
    tokens: Vec<Token>,
    tokenized_lines: Vec<TokenizedLine>,
    errors: Vec<LexError>,
}

impl<'lex> std::fmt::Debug for TokenScanner<'lex> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenScanner").finish()
    }
}

enum TokenScannerAction {
    Push,
    ReplaceLast,
}

impl<'token> TokenScanner<'token> {
    pub(crate) fn new(word_interner: &'token WordInterner) -> Self {
        Self {
            word_interner,
            tokens: vec![],
            tokenized_lines: vec![],
            errors: vec![],
        }
    }

    pub(crate) fn finish_with_tokens(self) -> Vec<Token> {
        self.tokens
    }

    pub(crate) fn scan(&mut self, line_index: usize, line: &str) {
        let start = self.tokens.len();
        let (indent, token_iter) = RawTokenIter::new(
            self.word_interner,
            line_index,
            line.chars().enumerate().peekable(),
        );
        self.push_tokens(token_iter);
        let end = self.tokens.len();
        self.tokenized_lines.push(TokenizedLine {
            indent,
            tokens: start..end,
        })
    }

    fn push_tokens(&mut self, iter: impl Iterator<Item = RawToken>) {
        for token in iter {
            let (action, token) = self.resolve_token(token);
            match action {
                TokenScannerAction::Push => self.tokens.push(token),
                TokenScannerAction::ReplaceLast => *self.tokens.last_mut().unwrap() = token,
            }
        }
    }

    fn resolve_token(&self, token: RawToken) -> (TokenScannerAction, Token) {
        let (action, kind) = match token.kind {
            RawTokenKind::Certain(token_kind) => (TokenScannerAction::Push, token_kind),
            RawTokenKind::SubOrMinus => (
                TokenScannerAction::Push,
                match self.right_convexity() {
                    Convexity::Convex => TokenKind::Special(SpecialToken::BinaryOpr(
                        BinaryOpr::Pure(PureBinaryOpr::Sub),
                    )),
                    Convexity::Concave | Convexity::Any => TokenKind::Special(SpecialToken::Minus),
                },
            ),
            RawTokenKind::Literal(lit) => match self.tokens.last().map(|t| t.kind) {
                Some(TokenKind::Special(SpecialToken::Minus)) => (
                    TokenScannerAction::ReplaceLast,
                    TokenKind::Literal(lit.negative().expect("todo")),
                ),
                _ => (TokenScannerAction::Push, TokenKind::Literal(lit)),
            },
            RawTokenKind::IllFormedLiteral(l) => {
                (TokenScannerAction::Push, TokenKind::IllFormedLiteral(l))
            }
        };
        (
            action,
            Token {
                range: token.range,
                kind,
            },
        )
    }

    fn right_convexity(&self) -> Convexity {
        match self.last_token_in_unfinished_line() {
            Some(token) => token.right_convexity(),
            None => Convexity::Concave,
        }
    }

    fn last_token_in_unfinished_line(&self) -> Option<&Token> {
        match self.tokenized_lines.last() {
            Some(line) => {
                if line.tokens.end == self.tokens.len() {
                    None
                } else {
                    assert!(line.tokens.end < self.tokens.len());
                    self.tokens.last()
                }
            }
            None => self.tokens.last(),
        }
    }

    fn last_token(&self, line: &TokenizedLine) -> &Token {
        &self.tokens[line.tokens.end - 1]
    }

    fn first_token(&self, line: &TokenizedLine) -> &Token {
        &self.tokens[line.tokens.start]
    }

    fn produce_line_groups(&mut self) -> Vec<TokenGroup> {
        let mut line_groups = Vec::new();
        line_groups.reserve_exact(self.tokenized_lines.len());
        let mut line_iter = self
            .tokenized_lines
            .iter()
            .filter(|line| line.tokens.len() > 0)
            .peekable();
        while let Some(first_line) = line_iter.next() {
            line_groups.push(
                unsafe { ref_to_mut_ref(self) }.produce_line_group(first_line, &mut line_iter),
            );
        }
        line_groups
    }

    fn produce_line_group<'a>(
        &mut self,
        first_line: &TokenizedLine,
        line_iter: &mut Peekable<impl Iterator<Item = &'a TokenizedLine>>,
    ) -> TokenGroup {
        let group_indent = first_line.indent;
        TokenGroup {
            indent: group_indent,
            tokens: first_line.tokens.start..{
                if self.last_token(first_line).kind == TokenKind::Special(SpecialToken::Colon) {
                    if let Some(line) = line_iter.peek() {
                        match line.indent.within(group_indent) {
                            Ok(is_within) => {
                                if !is_within {
                                    self.errors.push(LexError {
                                        message: format!("expect indentated lines after `:`"),
                                        range: self.last_token(first_line).range,
                                        dev_src: dev_src!(),
                                    });
                                }
                            }
                            Err(e) => self.errors.push(LexError {
                                message: format!("{:?}", e),
                                range: self.last_token(first_line).range,
                                dev_src: dev_src!(),
                            }),
                        }
                    } else {
                        self.errors.push(LexError {
                            message: format!("expect indentated lines after `:`"),
                            range: self.last_token(first_line).range,
                            dev_src: dev_src!(),
                        })
                    }
                    first_line.tokens.end
                } else {
                    loop {
                        if let Some(line) = line_iter.peek().map(|e| *e) {
                            match line.indent.within(group_indent) {
                                Ok(is_within) => {
                                    if is_within {
                                        line_iter.next();
                                        if self.last_token(line).kind
                                            == TokenKind::Special(SpecialToken::Colon)
                                        {
                                            break line.tokens.end;
                                        }
                                    } else {
                                        fn bind_to_last_line(kind: TokenKind) -> bool {
                                            match kind {
                                                TokenKind::Special(special) => match special {
                                                    SpecialToken::Ket(_) => true,
                                                    _ => false,
                                                },
                                                _ => false,
                                            }
                                        }

                                        if bind_to_last_line(self.first_token(line).kind.clone()) {
                                            line_iter.next();
                                            break line.tokens.end;
                                        } else {
                                            break line.tokens.start;
                                        }
                                    }
                                }
                                Err(e) => {
                                    self.errors.push(LexError {
                                        message: format!("{:?}", e),
                                        range: self.last_token(first_line).range,
                                        dev_src: dev_src!(),
                                    });
                                    line_iter.next();
                                }
                            }
                        } else {
                            break self.tokens.len();
                        }
                    }
                }
            },
        }
    }

    pub(crate) fn gen_tokenized_text(mut self) -> Arc<TokenizedText> {
        let line_groups = self.produce_line_groups();
        Arc::new(TokenizedText::new(line_groups, self.tokens, self.errors))
    }
}
