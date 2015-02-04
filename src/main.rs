extern crate readline;

use readline::{readline, add_history};
use grammar::*;
use std::result::Result;

mod grammar;

fn main()
{
    loop
    {
        let result = readline("Kiss! ");
        match result
        {
            Ok(input)   =>
            {
                add_history(input.as_slice());
                for token in evaluate(input)
                {
                    match token
                    {
                        Token::Literal(x)   =>  println!("      Literal({})", x),
                        Token::Operator(_)  =>  println!("      Operator")
                    }
                }
            },
            Err(_)      => break,
        }
    }
}

fn evaluate(input: String) -> Vec<Token>
{
    input
        .split(|c: char| {c.is_whitespace()})
        .map(get_token)
        .filter(Result::is_ok)
        .map(Result::unwrap).collect()
}
