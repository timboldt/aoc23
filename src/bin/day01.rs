#![warn(clippy::all)]

extern crate nom;

use nom::branch::alt;
use nom::bytes::complete::take;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::alphanumeric1;
use nom::character::complete::{alpha1, digit1};
use nom::character::is_alphabetic;
use nom::combinator::value;
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use nom::IResult;

fn digit_value(input: u8) -> u32 {
    input as u32 - '0' as u32
}

fn parse_calibration1(input: &[u8]) -> IResult<&[u8], u32> {
    let (rest, digits) =
        preceded(take_while(is_alphabetic), separated_list1(alpha1, digit1))(input)?;
    // First digit of first digit sequence.
    let first_digit = digit_value(*digits.first().unwrap().first().unwrap());
    // Last digit of last digit sequence.
    let last_digit = digit_value(*digits.last().unwrap().last().unwrap());
    //println!("first = {} last = {}", first_digit, last_digit);
    Ok((rest, first_digit * 10 + last_digit))
}

fn parse_calibration2(input: &[u8]) -> IResult<&[u8], u32> {
    let (_, first_digits) = many1(alt((
        alt((
            value(1u32, tag("one")),
            value(2u32, tag("two")),
            value(3u32, tag("three")),
            value(4u32, tag("four")),
            value(5u32, tag("five")),
            value(6u32, tag("six")),
            value(7u32, tag("seven")),
            value(8u32, tag("eight")),
            value(9u32, tag("nine")),
        )),
        alt((
            value(1u32, tag("1")),
            value(2u32, tag("2")),
            value(3u32, tag("3")),
            value(4u32, tag("4")),
            value(5u32, tag("5")),
            value(6u32, tag("6")),
            value(7u32, tag("7")),
            value(8u32, tag("8")),
            value(9u32, tag("9")),
            value(0xFFu32, take(1usize)),
        )),
    )))(input)?;

    let (rest, last_digits) = many1(alt((
        alt((
            value(1u32, tag("twone")),
            value(2u32, tag("eightwo")),
            value(3u32, tag("eighthree")),
            value(8u32, tag("oneight")),
            value(8u32, tag("threeight")),
            value(8u32, tag("fiveight")),
            value(8u32, tag("nineight")),
            value(1u32, tag("one")),
            value(2u32, tag("two")),
            value(3u32, tag("three")),
            value(4u32, tag("four")),
            value(5u32, tag("five")),
            value(6u32, tag("six")),
            value(7u32, tag("seven")),
            value(8u32, tag("eight")),
            value(9u32, tag("nine")),
        )),
        alt((
            value(1u32, tag("1")),
            value(2u32, tag("2")),
            value(3u32, tag("3")),
            value(4u32, tag("4")),
            value(5u32, tag("5")),
            value(6u32, tag("6")),
            value(7u32, tag("7")),
            value(8u32, tag("8")),
            value(9u32, tag("9")),
            value(0xFFu32, take(1usize)),
        )),
    )))(input)?;
    let first_digits: Vec<u32> = first_digits.into_iter().filter(|&x| x != 0xFFu32).collect();
    let last_digits: Vec<u32> = last_digits.into_iter().filter(|&x| x != 0xFFu32).collect();
    Ok((
        rest,
        first_digits.first().unwrap_or(&0u32) * 10 + last_digits.last().unwrap_or(&0u32),
    ))
}


fn main() {
    let input = include_str!("../../input/01.txt");
    let sum = input.lines().fold(0, |acc, s| {
        acc + parse_calibration1(s.as_bytes()).unwrap_or_default().1
    });
    println!("sum = {}", sum);

    let sum2 = input.lines().fold(0, |acc, s| {
        acc + parse_calibration2(s.as_bytes()).unwrap_or_default().1
    });
    println!("sum = {}", sum2);

    // let timer = Instant::now();
    // let p1 = part1(&parsed);
    // println!("Part 1: {:?}\n(elapsed: {:.2?})", p1, timer.elapsed());

    // let timer = Instant::now();
    // let p2 = part2(&parsed);
    // println!("Part 2: {:?}\n(elapsed: {:.2?})", p2, timer.elapsed());
}

// #[cfg(test)]
// mod tests {
//     const SAMPLE: &str = r"
//         1abc2
//         pqr3stu8vwx
//         a1b2c3d4e5f
//         treb7uchet";

//     #[test]
//     fn part1_works() {
//         let input = super::parse(SAMPLE);
//         assert_eq!(13, super::part1(&input));
//     }

//     #[test]
//     fn part2_works() {
//         let input = super::parse(SAMPLE);
//         assert_eq!(29, super::part2(&input));
//     }
// }
