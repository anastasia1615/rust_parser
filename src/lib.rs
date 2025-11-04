use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MarkdownParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn unit_header() {
        let input = "# testing some shi";
        let result = MarkdownParser::parse(Rule::header1, input);
        assert!(result.is_ok());
    }
}
