use grammar::parse_tree::{Grammar, GrammarItem};
use parser::parse_grammar;
use reference_builder::mdbook::MdbookReferenceBuilder;
use reference_builder::{LalrdocError, ReferenceBuilder};
use std::path::PathBuf;

#[derive(Clap)]
pub enum ReferenceFormat {
    Mdbook { output: PathBuf },
}

impl ReferenceFormat {
    fn builder(self, grammar: Grammar) -> Box<dyn ReferenceBuilder> {
        match self {
            ReferenceFormat::Mdbook { output } => {
                Box::new(MdbookReferenceBuilder { output, grammar })
            }
        }
    }
}

#[derive(Clap)]
pub struct Cli {
    #[clap(parse(try_from_str))]
    pub lalrpop_grammar: PathBuf,

    #[clap(subcommand)]
    pub format: ReferenceFormat,
}

pub fn run(cli: Cli) -> Result<(), LalrdocError> {
    let Cli {
        lalrpop_grammar,
        format,
    } = cli;

    let lalrpop_grammar = std::fs::read_to_string(lalrpop_grammar.as_path())
        .map_err(LalrdocError::CannotReadGrammar)?;
    let lalrpop_grammar = parse_grammar(&lalrpop_grammar).map_err(|_| LalrdocError::ParseError)?;

    format.builder(lalrpop_grammar).build_reference()
}
