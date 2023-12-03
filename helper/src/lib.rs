use std::borrow::Cow;

use nom::{character::complete::digit1, combinator::map, IResult};

pub type Solution = fn(&str) -> u64;

pub trait Day {
    fn part1() -> Variants;

    fn part2() -> Variants;

    /// Pad or manipulate the input in ways that don't necessarily
    /// change it but do things that may be sound or unsound.
    fn pad_input(input: &str) -> Cow<str> {
        Cow::Borrowed(input)
    }
}

pub struct Variants {
    pub variants: Vec<Variant>,
}

pub struct Variant {
    pub name: &'static str,
    pub f: Solution,
}

impl Variants {
    pub fn basic(f: Solution) -> Self {
        Variants {
            variants: vec![Variant { name: "basic", f }],
        }
    }
}

pub fn test_part1<D: Day>(inputs: &[(&str, u64)]) {
    for variant in D::part1().variants {
        for input in inputs {
            let actual = (variant.f)(input.0);
            if actual != input.1 {
                panic!("failed: {}: {actual} != {}", variant.name, input.1);
            }
        }
    }
}

pub fn test_part2<D: Day>(inputs: &[(&str, u64)]) {
    for variant in D::part2().variants {
        for input in inputs {
            let actual = (variant.f)(input.0);
            if actual != input.1 {
                panic!("failed: {}: {actual} != {}", variant.name, input.1);
            }
        }
    }
}

#[macro_export]
macro_rules! tests {
    (
        $day_small:ident $day:ident;
        part1 {
            small => $p1small:expr;
            default => $p1default:expr;
        }
        part2 {
            small => $p2small:expr;
            default => $p2default:expr;
        }
    ) => {
        #[cfg(test)]
        mod $day_small {
            #[test]
            fn part1() {
                helper::test_part1::<super::$day>(&[
                    (include_str!("../input_small.txt"), $p1small),
                    (include_str!("../input.txt"), $p1default),
                ]);
            }

            #[test]
            fn part2() {
                helper::test_part2::<super::$day>(&[
                    (include_str!("../input_small.txt"), $p2small),
                    (include_str!("../input.txt"), $p2default),
                ]);
            }
        }
    };
}

pub fn integer(input: &str) -> IResult<&str, u64> {
    map(digit1, |d: &str| d.parse::<u64>().unwrap())(input)
}
