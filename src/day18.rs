use aoc_runner_derive::{aoc, aoc_generator};
use nom::{branch::alt, bytes::complete::tag, character::complete::digit1, IResult};

#[derive(Debug, Clone)]
pub enum Token {
    Val(usize),
    Add,
    Mult,
    Paren(Vec<Token>),
}

impl Default for Token {
    fn default() -> Self {
        Token::Val(0)
    }
}

fn parse_val(input: &str) -> IResult<&str, Token> {
    let (input, v) = digit1(input)?;
    // println!("found val: {:?}", input);
    Ok((input, Token::Val(v.parse().unwrap())))
}

fn parse_add(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag(" + ")(input)?;
    // println!("found add: {:?}", input);
    Ok((input, Token::Add))
}

fn parse_mult(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag(" * ")(input)?;
    // println!("found mult: {:?}", input);
    Ok((input, Token::Mult))
}

fn parse_paren(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("(")(input)?;
    let (input, inner) = parse_eqn(input)?;
    let (input, _) = tag(")")(input)?;

    // println!("found_paren: {:?}", input);

    Ok((input, Token::Paren(inner)))
}

fn parse_eqn(input: &str) -> IResult<&str, Vec<Token>> {
    let mut input = input;
    let mut tokens = Vec::new();

    while !input.is_empty() && !input.starts_with(')') {
        let (rest, tok) = alt((parse_val, parse_add, parse_mult, parse_paren))(input)?;
        input = rest;
        tokens.push(tok);
    }

    Ok((input, tokens))
}

fn solve(input: &[Token]) -> usize {
    let mut state = 0;
    let mut pos = 0;
    while pos < input.len() {
        match input[pos] {
            Token::Val(v) => state = v,
            Token::Add => {
                state += match input[pos + 1].clone() {
                    Token::Val(v) => v,
                    Token::Paren(ref eqn) => solve(eqn),
                    _ => unreachable!(),
                };
                pos += 1;
            }
            Token::Mult => {
                state *= match input[pos + 1] {
                    Token::Val(v) => v,
                    Token::Paren(ref eqn) => solve(eqn),
                    _ => unreachable!(),
                };
                pos += 1;
            }
            Token::Paren(ref eqn) => state = solve(eqn),
        }

        pos += 1;
    }
    state
}

fn solve2(input: &[Token]) -> usize {
    let mut state = Vec::new();

    input.iter().for_each(|tok| match tok {
        Token::Val(v) => state.push(*v),
        Token::Add => {
            let first = state.pop().unwrap();
            let second = state.pop().unwrap();
            state.push(first + second);
        }
        Token::Mult => {
            let first = state.pop().unwrap();
            let second = state.pop().unwrap();
            state.push(first * second);
        }
        _ => unreachable!(),
    });

    state[0]
}

enum Precedence {
    Higher,
    Equal,
    Lower,
}

fn order(first: Token, second: Token) -> Precedence {
    match first {
        Token::Add => match second {
            Token::Add => Precedence::Equal,
            Token::Mult => Precedence::Higher,
            _ => unreachable!(),
        },
        Token::Mult => match second {
            Token::Add => Precedence::Lower,
            Token::Mult => Precedence::Equal,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn shunting_yard(input: &[Token]) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operators = Vec::<Token>::new();
    input.iter().for_each(|tok| match tok {
        Token::Val(v) => output.push(Token::Val(*v)),
        Token::Add | Token::Mult => {
            let mut other = operators.pop();
            while other.is_some() {
                match order(tok.clone(), other.clone().unwrap()) {
                    Precedence::Higher => {
                        operators.push(other.unwrap());
                        break;
                    }
                    Precedence::Equal | Precedence::Lower => {
                        output.push(other.unwrap());
                    }
                }
                other = operators.pop();
            }
            operators.push(tok.clone());
        }
        Token::Paren(eqn) => output.append(&mut shunting_yard(eqn)),
    });

    operators
        .iter()
        .rev()
        .for_each(|op| output.push(op.clone()));

    output
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|line| parse_eqn(line).unwrap().1)
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_p1(input: &[Vec<Token>]) -> usize {
    // println!("{:?}", input);
    input.iter().map(|op| solve(op)).sum()
}

#[aoc(day18, part2)]
pub fn solve_p2(input: &[Vec<Token>]) -> usize {
    input.iter().map(|op| solve2(&shunting_yard(op))).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    fn p1_wrapper(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_p1(&parsed)
    }

    fn p2_wrapper(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_p2(&parsed)
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1_wrapper("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(p1_wrapper("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(p1_wrapper("2 * 3 + (4 * 5)"), 26);
        assert_eq!(p1_wrapper("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            p1_wrapper("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            p1_wrapper("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2_wrapper("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(p2_wrapper("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(p2_wrapper("2 * 3 + (4 * 5)"), 46);
        assert_eq!(p2_wrapper("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            p2_wrapper("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            p2_wrapper("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
