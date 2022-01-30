use derive_more::Deref;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_till, take_until, take_while1};
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::{complete, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};
use nom::Parser;
use nom_supreme::error::ErrorTree;
use thiserror::Error;

mod chip;
mod connection;
mod pin_decl;
#[cfg(test)]
mod test_tools;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;
type PResult<'a, O> = nom::IResult<Span<'a>, O, ErrorTree<Span<'a>>>;

pub struct Chip<'a> {
    in_pins: Vec<Pin<'a>>,
    out_pins: Vec<Pin<'a>>,
    logic: Implementation<'a>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Implementation<'a> {
    Builtin(Symbol<'a>),
    Native(Vec<Connection<'a>>),
}

#[derive(Eq, PartialEq, Debug)]
pub struct Builtin<'a> {
    name: Symbol<'a>,
    clocked: Option<Vec<Symbol<'a>>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Pin<'a> {
    name: Symbol<'a>,
    size: Option<u16>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Connection<'a> {
    chip_name: Symbol<'a>,
    inputs: Vec<Argument<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Argument<'a> {
    internal: Symbol<'a>,
    internal_bus: Option<BusRange>,
    external: Symbol<'a>,
    external_bus: Option<BusRange>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Value {
    True,
    False,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Symbol<'a> {
    Name(Span<'a>),
    Value(Value),
    Number(usize),
}

#[derive(Deref, Eq, PartialEq, Debug)]
struct Name<'a>(&'a str);

impl<'a> TryFrom<Span<'a>> for Symbol<'a> {
    type Error = HdlParseError<'a>;

    fn try_from(value: Span<'a>) -> Result<Self, Self::Error> {
        // a valid symbol must be in only ascii characters, as well as consisting of no whitespace
        if value.is_ascii() && value.chars().all(|c| !c.is_ascii_whitespace()) {
            Ok(if let Ok(num) = usize::from_str_radix(*value, 10) {
                Symbol::Number(num)
            } else {
                match *value {
                    "true" => Symbol::Value(Value::True),
                    "false" => Symbol::Value(Value::False),
                    _ => Symbol::Name(value),
                }
            })
        } else {
            Err(HdlParseError::BadSymbol(value))
        }
    }
}

fn symbol(arg: Span) -> PResult<Span> {
    delimited(
        multispace0,
        take_while1(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9')),
        multispace0,
    )(arg)
}

#[derive(Debug, Eq, PartialEq)]
struct BusRange {
    start: u16,
    end: u16,
}

#[derive(Error, Debug, PartialEq)]
pub enum HdlParseError<'a> {
    #[error("Symbol `{0}` is not a valid symbol")]
    BadSymbol(Span<'a>),
}

fn skip_comma(arg: Span) -> PResult<()> {
    opt(complete(tuple((
        char(','),
        take_till(|c: char| !c.is_ascii_whitespace()),
    ))))
    .map(|_| ())
    .parse(arg)
}

fn generic_space1(arg: Span) -> PResult<()> {
    many0(alt((
        multispace1,
        complete(delimited(tag("/*"), take_until("*/"), tag("*/"))),
        complete(preceded(tag("//"), is_not("\n"))),
    )))
    .map(|_| ())
    .parse(arg)
}

fn generic_space0(arg: Span) -> PResult<()> {
    opt(generic_space1).map(|_| ()).parse(arg)
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_detect_symbol() {
//         assert_eq!(symbol(Span::new("abcdef ghijkl")), Ok((Span::new("ghijkl"), Span::new("abcdef"))));
//         assert_eq!(symbol(Span::new("1234, ghijkl")), Ok((Span::new(", ghijkl"), Span::new("1234"))));
//         assert_eq!(symbol(Span::new("abcd")), Ok((Span::new(""), Span::new("abcd"))));
//         assert_eq!(symbol(Span::new("AbCd")), Ok((Span::new(""), Span::new("AbCd"))));
//         assert!(matches!(symbol(Span::new("")), Err(_)))
//     }
//
//     #[test]
//     fn create_symbol() {
//         assert_eq!(Symbol::try_from(Span::new("breh")), Ok(Symbol::Name(Span::new("breh"))));
//         assert_eq!(Symbol::try_from(Span::new("12345")), Ok(Symbol::Number(12345)));
//         assert_eq!(Symbol::try_from(Span::new("false")), Ok(Symbol::Value(Value::False)));
//         assert!(matches!(
//             Symbol::try_from(Span::new("u r bad")),
//             Err(HdlParseError::BadSymbol(_))
//         ));
//     }
//
//     #[test]
//     fn test_generic_space0() {
//         assert_eq!(generic_space0(Span::new("/* // bruh */  abc")), Ok((Span::new("abc"), ())));
//         assert_eq!(generic_space0(Span::new("//abc\ndef")), Ok((Span::new("def"), ())));
//         assert_eq!(generic_space0(Span::new("/* word */")), Ok((Span::new(""), ())));
//         assert_eq!(generic_space0(Span::new("/* // word */")), Ok((Span::new(""), ())));
//         assert_eq!(generic_space0(Span::new("// /* word */")), Ok((Span::new(""), ())));
//         assert_eq!(generic_space0(Span::new("// word")), Ok((Span::new(""), ())));
//         assert_eq!(generic_space0(Span::new("// word\na")), Ok((Span::new("a"), ())));
//         assert_eq!(generic_space0(Span::new("//*")), Ok((Span::new(""), ())));
//     }
// }
