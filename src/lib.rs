use anyhow::Result;
use parser::parser::Parser;
use resolver::Resolver;
use runtime::Interpreter;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod resolver;
pub mod runtime;
pub mod token;
pub mod upside_down;

fn resolve_or_error(stmts: &mut [ast::Stmt]) -> Result<()> {
    if Resolver::new().resolve(stmts) {
        anyhow::bail!("Resolver reported one or more errors.");
    }

    Ok(())
}

pub fn interpret(src: &str) -> Result<()> {
    let mut lex = lexer::Lexer::new(lexer::source::Regular::new(src.chars()));
    let (tokens, _) = lex.lex();

    let mut parser = parser::parser::Parser::new(tokens);
    let mut stmts = parser.parse()?;

    resolve_or_error(&mut stmts)?;

    let mut iptr = Interpreter::new();
    iptr.interpret(stmts)?;

    Ok(())
}

pub fn interpret_repl(src: &str, interpreter: &mut Interpreter, parser: &mut Parser) -> Result<()> {
    let mut lex = lexer::Lexer::new(lexer::source::Regular::new(src.chars()));
    let (tokens, _) = lex.lex();

    parser.reset(tokens);
    let mut stmts = parser.parse()?;

    resolve_or_error(&mut stmts)?;

    interpreter.interpret(stmts)
}

pub fn interpret_upside_down(src: &str) -> Result<()> {
    let mut lex = lexer::Lexer::new(lexer::source::UpsideDown::new(src.chars()));
    let (tokens, _) = lex.lex();

    let mut parser = parser::parser::Parser::new(tokens);
    let mut stmts = parser.parse()?;

    resolve_or_error(&mut stmts)?;

    let mut iptr = Interpreter::new();
    iptr.interpret(stmts)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::interpret;

    #[test]
    fn resolver_errors_are_returned_to_the_caller() {
        let result = interpret("G'DAY MATE!\nGIMME missing;\nCHEERS C***!");

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Resolver reported"));
    }

    #[test]
    fn runtime_statement_errors_are_returned_to_the_caller() {
        let result = interpret("G'DAY MATE!\nIMPOHT ME FUNC NotAFunction;\nCHEERS C***!");

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("CAN'T FIND THE IMPORT NotAFunction"));
    }
}
