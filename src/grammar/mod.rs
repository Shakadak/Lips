use std::str::FromStr;
use std::fmt;

pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent }

pub enum Token {
    Operator(Operator),
    Literal(isize) }

pub enum Expression {
    Literal(isize),
    Expression (
            Operator,
            Box<Expression>,
            Box<Expression> )}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
                f,
                "{}",
                match *self {
                    Operator::Plus      =>  "+",
                    Operator::Minus     =>  "-",
                    Operator::Asterisk  =>  "*",
                    Operator::Slash     =>  "/",
                    Operator::Percent   =>  "%" })}}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
                f,
                "{}",
                match *self {
                    Token::Operator(ref op)     =>  format!("Operator({:?})", op),
                    Token::Literal(num)     =>  format!("Literal({})", num) })}}

pub fn assign_token(input: &str) -> Result<Token, String> {
    match input {
        "+"         =>  Ok(Token::Operator(Operator::Plus)),
        "-"         =>  Ok(Token::Operator(Operator::Minus)),
        "*"         =>  Ok(Token::Operator(Operator::Asterisk)),
        "/"         =>  Ok(Token::Operator(Operator::Slash)),
        "%"         =>  Ok(Token::Operator(Operator::Percent)),
        input       =>  match FromStr::from_str(input) {
            Ok(number)      =>  Ok(Token::Literal(number)),
            Err(_)          =>  Err(format!("Error: unassignable token ({})", input)) }}}
