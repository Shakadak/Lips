extern crate readline;

use readline::{readline, add_history};

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
                println!("{}", input)
            },
            Err(_)      => break,
        }
    }
}
