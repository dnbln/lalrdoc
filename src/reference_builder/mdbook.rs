use std::fmt::{self, Display};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use itertools::Itertools;

use grammar::parse_tree::{Alternative, Grammar, SymbolKind};
use reference_builder::ref_render::RefRender;
use reference_builder::{LalrdocError, ReferenceBuilder};

pub struct MdbookReferenceBuilder {
    pub output: PathBuf,
    pub grammar: Grammar,
}

#[derive(Default, Debug)]
struct PrecedenceData {
    level: usize,
    assoc: Option<String>,
}

impl Display for PrecedenceData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.level >= 2 {
            write!(
                f,
                "(precedence level = {}, associativity = {})",
                self.level,
                self.assoc.as_deref().unwrap_or("default")
            )
        } else if self.level >= 1 {
            write!(f, "(precedence level = {})", self.level)
        } else {
            write!(f, "")
        }
    }
}

fn get_precedence_data(alt: &Alternative) -> Option<PrecedenceData> {
    let precedence = alt
        .annotations
        .iter()
        .find(|it| it.id.to_string() == "precedence")
        .and_then(|it| it.arg.as_ref().map(|arg| arg.1.clone()))?;

    let level = precedence.parse().ok().unwrap_or(0_usize);

    let assoc = alt
        .annotations
        .iter()
        .find(|it| it.id.to_string() == "assoc")
        .filter(|_| level >= 2)
        .and_then(|it| it.arg.as_ref().map(|arg| arg.1.clone()));

    Some(PrecedenceData { level, assoc })
}

impl ReferenceBuilder for MdbookReferenceBuilder {
    fn build_reference(&self) -> Result<(), LalrdocError> {
        let mut nonterminal_names = vec![];
        self.grammar
            .items
            .iter()
            .filter_map(|it| it.as_nonterminal())
            .try_for_each(|nonterminal_data| -> Result<(), LalrdocError> {
                let (name, args) = (
                    nonterminal_data.name.to_string(),
                    nonterminal_data
                        .args
                        .iter()
                        .map(|arg| arg.to_string())
                        .collect_vec(),
                );
                nonterminal_names.push(name.clone());
                let doc_comments = nonterminal_data
                    .doc_comments
                    .iter()
                    .map(|it| &it[3..it.len()])
                    .join("\n");

                let mut f = File::create(self.output.join(format!("{}.md", name)))?;

                let alternatives_count = nonterminal_data.alternatives.len();
                let alternatives = nonterminal_data
                    .alternatives
                    .iter()
                    .enumerate()
                    .map(|(id, alt)| (id + 1, alt))
                    .map(|(id, alt)| {
                        let ind = if alternatives_count > 1 {
                            format!("({}) ", id)
                        } else {
                            "".to_string()
                        };

                        let symbols = alt
                            .expr
                            .symbols
                            .iter()
                            .filter_map(|it| {
                                it.render(nonterminal_data, &self.grammar).try_display()
                            })
                            .join(" ");

                        let precedence_data = get_precedence_data(alt)
                            .map(|it| it.to_string())
                            .unwrap_or_default();

                        let alternative_doc_comments = if !alt.doc_comments.is_empty() {
                            alt.doc_comments
                                .iter()
                                .map(|it| &it[3..it.len()])
                                .join("\n")
                        } else {
                            "".to_string()
                        };

                        format!(
                            "> {}{}{}\n{}\n",
                            ind, symbols, precedence_data, alternative_doc_comments
                        )
                    })
                    .join("\n");

                let args_str = if !args.is_empty() {
                    format!("<{}>", args.join(", "))
                } else {
                    "".to_string()
                };

                // TODO: add docs on nonterminal params
                let params_data = args.iter().map(|it| format!("## {}", it)).join("\n");

                write!(
                    &mut f,
                    "# {}{}\n{}\n{}\n{}",
                    name, args_str, params_data, alternatives, doc_comments
                )?;

                Ok(())
            })?;

        let mut f = File::create(self.output.join("SUMMARY.md"))?;

        write!(
            &mut f,
            "{}",
            nonterminal_names
                .iter()
                .map(|it| format!("- [{0}]({0}.md)", it))
                .join("\n")
        )?;

        Ok(())
    }
}
