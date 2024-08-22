use crate::parser::lexer::{Lexer, Loc, Tok};

#[derive(Debug)]
pub(crate) struct Statement {
    //pub(crate) keyword_loc: (Loc, Loc),
    pub(crate) keyword: String,
    //pub(crate) argument_loc: (Loc, Loc),
    pub(crate) argument: Option<String>,
    pub(crate) statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct ParseError {
    pub loc: Option<usize>,
    pub message: String,
}

pub(crate) fn parse(input: &str) -> Result<Statement, ParseError> {
    let mut lexer = Lexer::new(input);
    let first_tok = next_tok_or_err(&mut lexer)?;
    let statement = parse_statement(&mut lexer, first_tok)?;
    if let Some(tok) = lexer.next() {
        return Err(ParseError {
            loc: Some(tok.0),
            message: "Expected end of input, found ".to_string(),
        });
    }

    Ok(statement)
}

fn next_tok_or_err(lexer: &mut Lexer) -> Result<(usize, Tok, usize), ParseError> {
    let tok = lexer.next().ok_or_else(|| ParseError {
        loc: None,
        message: "Expected token, found end of input".to_string(),
    })?;

    match tok.1 {
        Err(err) => Err(ParseError {
            loc: Some(tok.0),
            message: format!("{:?}", err),
        }),
        Ok(token) => Ok((tok.0, token, tok.2)),
    }
}

fn parse_statement(
    lexer: &mut Lexer,
    first_tok: (usize, Tok, usize),
) -> Result<Statement, ParseError> {
    if let Tok::UString(keyword) = first_tok.1 {
        let mut argument: Option<String> = None;
        let mut statements: Vec<Statement> = vec![];

        let mut argument_or_statements = next_tok_or_err(lexer)?;
        if let Tok::UString(arg) = argument_or_statements.1 {
            argument = Some(arg);
            argument_or_statements = next_tok_or_err(lexer)?;
        } else if let Tok::QString(arg) = argument_or_statements.1 {
            let mut arg_joined = arg;
            loop {
                argument_or_statements = next_tok_or_err(lexer)?;
                if let Tok::Plus = argument_or_statements.1 {
                    let next_tok = next_tok_or_err(lexer)?;
                    if let Tok::QString(arg) = next_tok.1 {
                        arg_joined += &arg;
                    } else {
                        return Err(ParseError {
                            loc: Some(next_tok.0),
                            message: "Expected string, found something else".to_string(),
                        });
                    }
                } else {
                    break;
                }
            }
            argument = Some(arg_joined);
        }

        if let Tok::Semicolon = argument_or_statements.1 {
        } else if let Tok::LBrace = argument_or_statements.1 {
            loop {
                let next_tok = next_tok_or_err(lexer)?;
                if let Tok::RBrace = next_tok.1 {
                    break;
                }
                statements.push(parse_statement(lexer, next_tok)?);
            }
        } else {
            return Err(ParseError {
                loc: Some(argument_or_statements.0),
                message: "Expected semicolon or left brace, found something else".to_string(),
            });
        }

        Ok(Statement {
            keyword,
            argument,
            statements,
        })
    } else {
        Err(ParseError {
            loc: Some(first_tok.0),
            message: "Expected keyword, found something else".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"
leaf host-name {
    type string;
    description
        "Hostname for this system." +
        'This is a string';
    test-multiline
        "This is a
           multiline string";
    test-singlequote
        'This is a single"
           \" quote string';
}
        "#;
        print!("{:?}", parse(input));
    }
}
