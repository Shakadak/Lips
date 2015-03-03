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
            Some(input)     =>
            {
                add_history(input.as_slice());
                for token in evaluate(input)
                {
                    println!("      {:?}", token);
                }
            },
            None            => break,
        }
    }
}

fn evaluate(input: String) -> Vec<Token>
{
    input
        .split(CharExt::is_whitespace)
        .map(get_token)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()
}
