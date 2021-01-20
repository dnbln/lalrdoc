use grammar::parse_tree::{Grammar, NonterminalData, RepeatOp, Symbol, SymbolKind};

pub enum RefRendered {
    SomeRendered { rendered: String },
    NoneRendered,
    DontRenderAlternative,
}

impl RefRendered {
    pub(crate) fn try_display(&self) -> Option<String> {
        match self {
            SomeRendered { rendered } => Some(rendered.clone()),
            NoneRendered | DontRenderAlternative => None,
        }
    }
}

pub trait RefRender {
    fn render(&self, nonterminal_parent: &NonterminalData, grammar: &Grammar) -> RefRendered;
}

use self::RefRendered::*;
use itertools::Itertools;

impl RefRender for Symbol {
    fn render(&self, nonterminal_parent: &NonterminalData, grammar: &Grammar) -> RefRendered {
        match &self.kind {
            SymbolKind::Expr(expr) => SomeRendered {
                rendered: format!(
                    "({})",
                    expr.symbols
                        .iter()
                        .filter_map(|it| it.render(nonterminal_parent, grammar).try_display())
                        .join(" ")
                ),
            },
            SymbolKind::AmbiguousId(id) => {
                let name = id.to_string();
                if nonterminal_parent
                    .args
                    .iter()
                    .any(|it| it.to_string() == name)
                {
                    SomeRendered {
                        rendered: format!(
                            "[{name}](#{name_lowercase})",
                            name = name,
                            name_lowercase = name.to_lowercase()
                        ),
                    }
                } else {
                    SomeRendered {
                        rendered: format!("[{name}]({name}.md)", name = name),
                    }
                }
            }
            SymbolKind::Terminal(terminal) => SomeRendered {
                rendered: terminal.to_string(),
            },
            SymbolKind::Nonterminal(nonterminal) => {
                let nonterminal_name = nonterminal.to_string();

                SomeRendered {
                    rendered: format!("[{name}]({name}.md)", name = nonterminal_name),
                }
            }
            SymbolKind::Macro(m) => {
                // TODO: inline the macro #1
                SomeRendered {
                    rendered: format!(
                        "[{name}]({name}.md)&lt;{args}&gt;",
                        name = m.name.to_string(),
                        args = m
                            .args
                            .iter()
                            .filter_map(|it| it.render(nonterminal_parent, grammar).try_display())
                            .join(", ")
                    ),
                }
            }
            SymbolKind::Repeat(repeat) => repeat
                .symbol
                .render(nonterminal_parent, grammar)
                .try_display()
                .map(|it| {
                    format!(
                        "{}{}",
                        it,
                        match repeat.op {
                            RepeatOp::Star => "*",
                            RepeatOp::Plus => "<sub>+</sub>",
                            RepeatOp::Question => "<sub>?</sub>",
                        }
                    )
                })
                .map(|it| SomeRendered { rendered: it })
                .unwrap_or(NoneRendered),
            SymbolKind::Choose(sym) => sym.render(nonterminal_parent, grammar),
            SymbolKind::Name(_, sym) => sym.render(nonterminal_parent, grammar),
            SymbolKind::Lookahead => NoneRendered,
            SymbolKind::Lookbehind => NoneRendered,
            SymbolKind::Error => DontRenderAlternative,
        }
    }
}
