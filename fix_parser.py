use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::space1;
use nom::combinator::{opt, eof};
use nom::multi::many0;
use nom::sequence::{delimited, terminated, preceded};
use nom::{IResult, error::ErrorKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,
    pub redirects: Vec<Redirect>,
    pub pipeline: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redirect {
    pub from: RedirectType,
    pub to: String,
    pub append: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectType {
    Stdout,
    Stderr,
    Stdin,
}

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<ParsedCommand, String> {
        let input = input.trim();
        
        if input.is_empty() {
            return Err("Empty command".to_string());
        }

        match parse_command(input) {
            Ok((_, parsed)) => Ok(parsed),
            Err(e) => Err(format!("Parse error: {:?}", e)),
        }
    }

    pub fn parse_pipeline(input: &str) -> Vec<ParsedCommand> {
        input
            .split('|')
            .filter_map(|cmd| Self::parse(cmd).ok())
            .collect()
    }
}

fn parse_command(input: &str) -> IResult<&str, ParsedCommand> {
    let (input, _) = many0(space1)(input)?;
    let (input, command) = parse_command_name(input)?;
    let (input, args) = many0(terminated(parse_argument, many0(space1)))(input)?;
    let (input, redirects) = many0(parse_redirect)(input)?;
    let (_, _) = many0(preceded(space1, tag("|")))(input)?;
    
    Ok((input, ParsedCommand {
        command: command.to_string(),
        args: args.into_iter().map(|s| s.to_string()).collect(),
        redirects,
        pipeline: !redirects.is_empty(),
    }))
}

fn parse_command_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')(input)
}

fn parse_argument(input: &str) -> IResult<&str, &str> {
    let (input, _) = many0(space1)(input)?;
    
    if input.starts_with('"') {
        delimited(tag("\""), take_until("\""), tag("\""))(input)
    } else if input.starts_with("'") {
        delimited(tag("'"), take_until("'"), tag("'"))(input)
    } else {
        take_while1(|c: char| !c.is_whitespace() && !"|><&".contains(c))(input)
    }
}

fn parse_redirect(input: &str) -> IResult<&str, Redirect> {
    let (input, _) = many0(space1)(input)?;
    
    let (input, (redirect_type, append)) = if input.starts_with(">>") {
        (tag(">>")(input), (RedirectType::Stdout, true))
    } else if input.starts_with("2>>") {
        (tag("2>>")(input), (RedirectType::Stderr, true))
    } else if input.starts_with(">") {
        (tag(">")(input), (RedirectType::Stdout, false))
    } else if input.starts_with("2>") {
        (tag("2>")(input), (RedirectType::Stderr, false))
    } else if input.starts_with("<") {
        (tag("<")(input), (RedirectType::Stdin, false))
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(input, ErrorKind::Tag)));
    };
    
    let (input, _) = many0(space1)(input)?;
    let (input, target) = parse_argument(input)?;
    
    Ok((input, Redirect {
        from: redirect_type,
        to: target.to_string(),
        append,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let result = Parser::parse("ls -la");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.command, "ls");
        assert_eq!(cmd.args, vec!["-la"]);
    }

    #[test]
    fn test_command_with_args() {
        let result = Parser::parse("git status --short");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.command, "git");
        assert_eq!(cmd.args, vec!["status", "--short"]);
    }

    #[test]
    fn test_quoted_args() {
        let result = Parser::parse(r#"echo "hello world""#);
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.command, "echo");
        assert_eq!(cmd.args, vec!["hello world"]);
    }

    #[test]
    fn test_redirect() {
        let result = Parser::parse("ls > output.txt");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.command, "ls");
        assert_eq!(cmd.redirects.len(), 1);
        assert_eq!(cmd.redirects[0].from, RedirectType::Stdout);
        assert_eq!(cmd.redirects[0].to, "output.txt");
    }

    #[test]
    fn test_empty_input() {
        let result = Parser::parse("");
        assert!(result.is_err());
    }
}