use std::sync::Arc;

use cstree::Syntax;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Syntax)]
#[repr(u32)]
pub enum SyntaxKind {
    Root,
    Newline,
    Text,
    Whitespace,
}

pub type ResolvedNode = cstree::syntax::ResolvedNode<SyntaxKind>;

use cstree::build::{GreenNodeBuilder, NodeCache};
use cstree::interning::{MultiThreadedTokenInterner, new_threaded_interner};
use cstree::syntax::SyntaxNode;

type Interner = Arc<MultiThreadedTokenInterner>;

struct Parser<'s, 'c> {
    tokens: Vec<(SyntaxKind, &'s str)>,
    pos: usize,
    builder: GreenNodeBuilder<'c, 'static, SyntaxKind, Interner>,
}

impl Parser<'_, '_> {
    fn peek(&self) -> Option<SyntaxKind> {
        self.tokens.get(self.pos).map(|t| t.0)
    }

    fn bump(&mut self) {
        let (kind, text) = self.tokens[self.pos];
        self.builder.token(kind, text);
        self.pos += 1;
    }

    fn parse_root(&mut self) {
        self.builder.start_node(SyntaxKind::Root);
        while let Some(_kind) = self.peek() {
            self.bump()
        }
        self.builder.finish_node();
    }
}

pub fn lex(src: &str) -> Vec<(SyntaxKind, &str)> {
    let mut out = Vec::new();
    let mut rest = src;
    while let Some(c) = rest.chars().next() {
        let (kind, len) = match c {
            '\n' => (SyntaxKind::Newline, 1),
            c if c.is_whitespace() => (
                SyntaxKind::Whitespace,
                take_while(rest, |c| c.is_whitespace() && c != '\n'),
            ),
            _ => (
                SyntaxKind::Text,
                take_while(rest, |c| !c.is_whitespace() && c != '\n'),
            ),
        };
        out.push((kind, &rest[..len]));
        rest = &rest[len..];
    }
    out
}

fn take_while(s: &str, pred: impl Fn(char) -> bool) -> usize {
    s.find(|c| !pred(c)).unwrap_or(s.len())
}

pub struct Grammar {
    cache: NodeCache<'static, Interner>,
}

impl Default for Grammar {
    fn default() -> Self {
        Grammar {
            cache: NodeCache::from_interner(Arc::new(new_threaded_interner())),
        }
    }
}

impl Grammar {
    pub fn parse(&mut self, src: &str) -> ResolvedNode {
        let mut p = Parser {
            tokens: lex(src),
            pos: 0,
            builder: GreenNodeBuilder::with_cache(&mut self.cache),
        };
        p.parse_root();
        let (green, _) = p.builder.finish();
        SyntaxNode::new_root_with_resolver(green, self.cache.interner().clone())
    }
}
