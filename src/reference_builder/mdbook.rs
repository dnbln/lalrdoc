use grammar::parse_tree::{Grammar, SymbolKind};
use itertools::Itertools;
use reference_builder::{LalrdocError, ReferenceBuilder};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct MdbookReferenceBuilder {
    pub output: PathBuf,
    pub grammar: Grammar,
}

impl ReferenceBuilder for MdbookReferenceBuilder {
    fn build_reference(&self) -> Result<(), LalrdocError> {
        let mut nonterminal_names = vec![];
        self.grammar
            .items
            .iter()
            .filter_map(|it| it.as_nonterminal())
            .try_for_each(|it| -> Result<(), LalrdocError> {
                let (name, args) = (
                    it.name.to_string(),
                    it.args.iter().map(|arg| arg.to_string()).collect_vec(),
                );
                nonterminal_names.push(name.clone());
                let doc_comments = it.doc_comments.iter().map(|it| &it[3..it.len()]).join("\n");

                let mut f = File::create(self.output.join(format!("{}.md", name)))?;

                let alternatives_count = it.alternatives.len();
                let alternatives = it
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

                        let symbols = format!("`{}`", &alt.expr);
                        format!("> {}{}", ind, symbols)
                    })
                    .join("\n>\n");

                let args_str = if !args.is_empty() {
                    format!("<{}>", args.join(", "))
                } else {
                    "".to_string()
                };

                write!(
                    &mut f,
                    "# {}{}\n{}\n{}",
                    name, args_str, alternatives, doc_comments
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
