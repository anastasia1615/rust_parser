use pest::Parser;
use markdown_parser::*;

fn main() -> anyhow::Result<()> {
    let input = "# I love photography";
    let parsed = MarkdownParser::parse(Rule::header1, input)?;
    println!("{:#?}", parsed);
    Ok(())
}
