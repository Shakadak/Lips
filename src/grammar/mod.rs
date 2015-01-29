enum Operator
{
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent
}

enum Expression
{
    (Box<Expression>, Operator, Box<Expression>),
    Literal
}
