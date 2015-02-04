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
                evaluate(input);
            },
            Err(_)      => break,
        }
    }
}

fn evaluate(input: String)
{
    let split = input
        .split(|c: char| {c.is_whitespace()})
        .map(get_token)
        .filter(Result::is_ok)
        .map(Result::unwrap);
    let mut token = Vec::new();
    for chunk in split {token.push(chunk);}
}
