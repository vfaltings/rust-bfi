use crate::lexer::Token;
use crate::program::BFEnv;
use core::slice::Iter;

#[derive(Debug)]
pub enum Expr {
    IncrPtr,
    DecrPtr,
    IncrArr,
    DecrArr,
    Output,
    Input,
    While { body: Vec<Expr> },
}

impl Instruction for Expr {
    fn execute(&self, env: &mut BFEnv) {
        match self {
            Expr::IncrPtr => env.incr_ptr(),
            Expr::DecrPtr => env.decr_ptr(),
            Expr::IncrArr => env.incr_arr(),
            Expr::DecrArr => env.decr_arr(),
            Expr::Output => print!("{}", env.get_byte() as char),
            Expr::Input => env.read_byte(),
            Expr::While { body } => {
                while env.get_byte() != 0 {
                    for e in body {
                        e.execute(env);
                    }
                }
            }
        }
    }
}

pub trait Instruction {
    fn execute(&self, env: &mut BFEnv);
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Expr>, &'static str> {
    let mut result = Vec::new();

    let mut iter = tokens.iter();

    loop {
        let t = if let Some(v) = iter.next() { v } else { break };

        match t {
            Token::StartLoop => result.push(Expr::While {
                body: parse_while(&mut iter)?,
            }),
            Token::EndLoop => return Err("Parsing Error: unmatched loop bracket!"),
            _ => result.push(parse_simpletoken(t)?),
        }
    }

    Ok(result)
}

fn parse_simpletoken(token: &Token) -> Result<Expr, &'static str> {
    let res = match token {
        Token::Right => Expr::IncrPtr,
        Token::Left => Expr::DecrPtr,
        Token::Plus => Expr::IncrArr,
        Token::Minus => Expr::DecrArr,
        Token::Out => Expr::Output,
        Token::In => Expr::Input,
        _ => return Err("Parsing Error: Trying to parse while token as simple token"),
    };

    Ok(res)
}

fn parse_while(iter: &mut Iter<Token>) -> Result<Vec<Expr>, &'static str> {
    let mut result = Vec::new();

    loop {
        let t = if let Some(v) = iter.next() { v } else { break };

        match t {
            Token::StartLoop => result.push(Expr::While {
                body: parse_while(iter)?,
            }),
            Token::EndLoop => return Ok(result),
            _ => result.push(parse_simpletoken(t)?),
        }
    }

    Err("Parsing Error: unmatched loop bracket!")
}
