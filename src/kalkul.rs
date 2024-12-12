use std::io::BufRead;
use std::string::{String, FromUtf8Error};
use std::str::FromStr;
use std::char::ParseCharError;

#[derive(Debug)]
pub enum Error {
    ReadError,
    ParseError,
    NotEnoughElements,
    UnknownOperator,
}

impl From<ParseCharError> for Error {
    fn from(_e: ParseCharError) -> Error {
        Error::ParseError
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_e: FromUtf8Error) -> Error {
        Error::ParseError
    }
}

impl From<std::io::Error> for Error {
    fn from(_e: std::io::Error) -> Error {
        Error::ReadError
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum OpKind {
    Plus,
    Minus,
    Divide,
    Multiply,
    OpenParen,
    CloseParen,

    Unknown,
}

#[derive(Debug)]
struct Op {
    kind: OpKind,
    prec: u8,
}

impl Op {
    fn new(kind: OpKind) -> Self {
        let prec = match kind {
            OpKind::Plus        => 1,
            OpKind::Minus       => 1,
            OpKind::Divide      => 2,
            OpKind::Multiply    => 2,
            OpKind::OpenParen   => 3,
            OpKind::CloseParen  => 3,

            OpKind::Unknown     => 0,
        };

        Op {
            kind,
            prec,
        }
    }

    fn from_char(c: &char) -> Self {
        let kind = match *c {
            '+' => OpKind::Plus,
            '-' => OpKind::Minus,
            '/' => OpKind::Divide,
            '*' => OpKind::Multiply,
            '(' => OpKind::OpenParen,
            ')' => OpKind::CloseParen,
            _ => OpKind::Unknown
        };
        Op::new(kind)
    }
}

const CHAR_OPS : [char; 6] = [
    '+',
    '-',
    '/',
    '*',
    '(',
    ')',
];

struct Evaluator {
    nums: Vec<u8>,
    ops: Vec<Op>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            nums: Vec::new(),
            ops: Vec::new(),
        }
    }

    pub fn evaluate(&mut self) -> Result<u8> {
        match (self.pop_num(), self.pop_num()) {
            (None, None) => Err(Error::NotEnoughElements),
            (Some(_), None)    => Err(Error::NotEnoughElements),
            (None, Some(_))    => Err(Error::NotEnoughElements),
            (Some(lhs), Some(rhs)) => {
                if let Some(op) = self.pop_op() {
                    match op.kind {
                        OpKind::Unknown     => Err(Error::UnknownOperator),

                        OpKind::Plus        => Ok(rhs + lhs),
                        OpKind::Minus       => Ok(rhs - lhs),
                        OpKind::Divide      => Ok(rhs / lhs),
                        OpKind::Multiply    => Ok(rhs * lhs),
                        OpKind::OpenParen | OpKind::CloseParen => todo!(),
                    }
                } else {
                    Err(Error::NotEnoughElements)
                }
            }
        }
    }

    pub fn ops_empty(&self) -> bool {
        self.ops.len() == 0
    }

    pub fn push_op(&mut self, op: Op) {
        self.ops.push(op)
    }

    pub fn pop_op(&mut self) -> Option<Op> {
        self.ops.pop()
    }

    pub fn top_op(&self) -> Option<&Op> {
        self.ops.last()
    }

    pub fn push_num(&mut self, n: u8) {
        self.nums.push(n)
    }

    pub fn pop_num(&mut self) -> Option<u8> {
        self.nums.pop()
    }
}

fn is_num(c: &char) -> bool {
    c.is_digit(10)
}

fn is_op(c: &char) -> bool {
    for op in CHAR_OPS {
        if *c == op {
            return true;
        }
    }
    false
}

pub fn evaluate(mut src: impl BufRead) -> Result<u8> {
    let mut ev = Evaluator::new();

    for buf in src.split(b' ') {
        let token = char::from_str(String::from_utf8(buf?.clone())?.trim())?;
        if is_num(&token) {
            ev.push_num(token.to_string().parse().unwrap());
            println!("{:?}", ev.nums);
            continue;
        }
        if is_op(&token) {
            while !ev.ops_empty() {
                todo!();
            }
            let op = Op::from_char(&token);
            ev.push_op(op);
            println!("{:?}", ev.ops);
            continue;
        }
    }

    ev.evaluate()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{BufReader, Cursor};
    use std::iter::zip;

    #[test]
    fn test_single_op() {
        let exprs = [
            "1 + 1",
            "6 - 3",
            "2 * 3",
            "4 / 2",
        ];
        let answers = [
            2, 3, 6, 2,
        ];

        for (expr, ans) in zip(exprs, answers) {
            let src = BufReader::new(Cursor::new(expr));
            assert_eq!(ans, evaluate(src).unwrap());
        }
    }
}
