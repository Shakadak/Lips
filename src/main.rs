extern crate readline;

use readline::{readline, add_history};
use grammar::*;
use std::result::Result;

mod grammar;

fn main() {
    loop {
        let result = readline("Kiss! ");
        match result {
            Some(input)
                => {
                    add_history(input.as_slice());
                    for token in tokenize(input) {
                        match token {
                            Ok(token)
                                => {
                                    println!("      {:?}", token) }
                            Err(error)
                                => {
                                    println!("      {}", error);
                                    break },}}}
            None
                => {
                    break }}}}

fn tokenize(input: String) -> Vec<Result<Token, String>> {
    input
        .split(CharExt::is_whitespace)
        .map(assign_token)
        .collect() }
