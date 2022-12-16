use color_eyre::owo_colors::OwoColorize;
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::separated_list1;
use nom::sequence::*;
use nom::IResult;

#[derive(Clone, Debug)]
pub struct Monkey {
    items: Vec<u64>,
    inspections: u64,
    operation: Operation,
    divisor: u64,
    true_target: usize,
    false_target: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Sum(Term, Term),
    Prod(Term, Term),
}

impl Operation {
    // old here is using the previous value, otherwise it will be given the Term::constant value
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Sum(l, r) => l.eval(old) + r.eval(old),
            Operation::Prod(l, r) => l.eval(old) * r.eval(old),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Term {
    Old,
    Constant(u64),
}
impl Term {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Constant(c) => c,
        }
    }
}

fn parse_term(s: &str) -> IResult<&str, Term> {
    alt((value(Term::Old, tag("old")), map(u64, Term::Constant)))(s)
}

fn parse_operation(s: &str) -> IResult<&str, Operation> {
    let (i, (l, _, op, _, r)) = preceded(
        tag("new = "),
        tuple((parse_term, space1, one_of("*+"), space1, parse_term)),
    )(s)?;
    let op = match op {
        '*' => Operation::Prod(l, r),
        '+' => Operation::Sum(l, r),
        _ => unreachable!(),
    };
    Ok((i, op))
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (i, _) = tuple((tag("Monkey "), u64, tag(":\n")))(s)?;
    let (i, (_, _, items, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), u64),
        tag("\n"),
    ))(i)?;
    let (i, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, tag("\n")))(i)?;
    let (i, (_, _, divisor, _)) = tuple((space1, tag("Test: divisible by "), u64, tag("\n")))(i)?;
    let (i, (_, _, true_target, _)) =
        tuple((space1, tag("If true: throw to monkey "), u64, tag("\n")))(i)?;
    let (i, (_, _, false_target, _)) =
        tuple((space1, tag("If false: throw to monkey "), u64, tag("\n")))(i)?;
    let m = Monkey {
        items,
        inspections: 0,
        operation,
        divisor,
        true_target: true_target as usize,
        false_target: false_target as usize,
    };
    Ok((i, m))
}
pub fn parse_monkeys(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n"), parse_monkey)(i)
}

fn do_round(monkeys: &mut [Monkey], divisor_product: u64) {
    let num_monkeys = monkeys.len();

    for i in 0..num_monkeys {
        let mc;
        {
            let monkey = &mut monkeys[i];
            mc = monkey.clone();
        }
        for mut item in mc.items.iter().copied() {
            item = mc.operation.eval(item);
            item %= divisor_product;
            if item % mc.divisor == 0 {
                monkeys[mc.true_target].items.push(item);
            } else {
                monkeys[mc.false_target].items.push(item);
            }
            monkeys[i].inspections += 1;
        }
        monkeys[i].items.clear();
    }
}
pub fn day_eleven() {
    let lines = include_str!("../input/day11-input.txt");
    // parse 'Monkey 0:'
    //
    let mut monkeys = all_consuming(parse_monkeys)(lines).unwrap().1;

    let divisor_product = monkeys.iter().map(|m| m.divisor).product::<u64>();
    dbg!(divisor_product);
    for _ in 0..10_000 {
        do_round(&mut monkeys, divisor_product);
    }
    let mut counts = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    counts.sort();

    let result = counts.pop().unwrap() * counts.pop().unwrap();
    println!("result: {result}");

    //println!("{:?}", monkeys);
}
