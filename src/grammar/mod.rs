use std::str::FromStr;

pub enum Operator
{
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent
}

pub enum Token
{
    Operator(Operator),
    Literal(isize)
}

pub enum Expression
{
    Literal(isize),
    Expression
        (
            Operator,
            Box<Expression>,
            Box<Expression>
        )
}

pub fn get_token(input: &str) -> Result<Token, &str>
{
    match input
    {
        "+"         =>  Ok(Token::Operator(Operator::Plus)),
        "-"         =>  Ok(Token::Operator(Operator::Minus)),
        "*"         =>  Ok(Token::Operator(Operator::Asterisk)),
        "/"         =>  Ok(Token::Operator(Operator::Slash)),
        "%"         =>  Ok(Token::Operator(Operator::Percent)),
        input       =>  match FromStr::from_str(input)
        {
            Ok(number)      =>  Ok(Token::Literal(number)),
            Err(_)          =>  Err(input)
        }
    }
}
