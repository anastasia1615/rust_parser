use pest::Parser;
use anyhow::anyhow;
use markdown_parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_header() -> anyhow::Result<()> {
        let mut got = MarkdownParser::parse(Rule::header1, "# I love beer")?;
        let got = got.next().ok_or_else(|| anyhow!("No pairs"))?;
        assert_eq!(got.as_str(), "# I love beer");
        assert!(MarkdownParser::parse(Rule::header1, "# ").is_err());
        assert!(MarkdownParser::parse(Rule::header1, "I").is_err());
        Ok(())
    }
}
