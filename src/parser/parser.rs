use crate::parser::lexer::{Lexer, Tok};
use crate::Span;

#[derive(Debug)]
pub(crate) struct Statement {
    pub(crate) keyword_span: Span,
    pub(crate) keyword: String,
    pub(crate) argument_span: Span,
    pub(crate) argument: Option<String>,
    pub(crate) statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct ParseError {
    pub span: Option<Span>,
    pub message: String,
}

pub(crate) fn parse(input: &str) -> Result<Statement, ParseError> {
    let mut lexer = Lexer::new(input);
    let first_tok = next_tok_or_err(&mut lexer)?;
    let statement = parse_statement(&mut lexer, first_tok)?;
    if let Some(tok) = lexer.next() {
        return Err(ParseError {
            span: Some(tok.0),
            message: format!("Expected end of input, found {:?}", tok.1),
        });
    }

    Ok(statement)
}

fn next_tok_or_err(lexer: &mut Lexer) -> Result<(Span, Tok), ParseError> {
    let tok = lexer.next().ok_or_else(|| ParseError {
        span: None,
        message: "Expected token, found end of input".to_string(),
    })?;

    match tok.1 {
        Err(err) => Err(ParseError {
            span: Some(tok.0),
            message: format!("{:?}", err),
        }),
        Ok(token) => Ok((tok.0, token)),
    }
}

fn parse_statement(
    lexer: &mut Lexer,
    first_tok: (Span, Tok),
) -> Result<Statement, ParseError> {
    if let Tok::UString(keyword) = first_tok.1 {
        let keyword_span = first_tok.0;
        let mut argument: Option<String> = None;
        let mut statements: Vec<Statement> = vec![];

        let mut argument_or_statements = next_tok_or_err(lexer)?;
        let argument_span = argument_or_statements.0;
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
                            span: Some(next_tok.0),
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
                span: Some(argument_or_statements.0),
                message: "Expected semicolon or left brace, found something else".to_string(),
            });
        }

        Ok(Statement {
            keyword,
            keyword_span,
            argument,
            argument_span,
            statements,
        })
    } else {
        Err(ParseError {
            span: Some(first_tok.0),
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
