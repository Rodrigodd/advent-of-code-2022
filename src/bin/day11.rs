const INPUT: &str = include_str!("../../inputs/day11.txt");

fn main() {
    let input = INPUT;
    let mut monkeys = parse_monkeys(input);
    run_monkeys(&mut monkeys, false);
    let business = monkey_business(monkeys);
    println!("monkey business: {business}");

    let mut monkeys = parse_monkeys(input);
    run_monkeys(&mut monkeys, true);
    let business = monkey_business(monkeys);
    println!("monkey business (very worry): {business}");
}

#[test]
fn example() {
    let input = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    let mut monkeys = parse_monkeys(input);
    run_monkeys(&mut monkeys, false);
    let business = monkey_business(monkeys);
    assert_eq!(10605, business);

    let mut monkeys = parse_monkeys(input);
    run_monkeys(&mut monkeys, true);
    let business = monkey_business(monkeys);
    assert_eq!(2713310158, business);
}

fn monkey_business(monkeys: Vec<Monkey>) -> u128 {
    let mut score: Vec<_> =
        monkeys.iter().map(|x| x.inspected).collect();
    dbg!(&score);
    score.sort();
    score.reverse();
    score[0] * score[1]
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    square: bool,
    times: u128,
    plus: u128,
    divisible: u128,
    if_true: usize,
    if_false: usize,
    inspected: u128,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter_map(|x| {
            dbg!(x);
            let mut lines = x.lines();
            lines.next()?;
            let items = lines
                .next()?
                .trim()
                .strip_prefix("Starting items:")?
                .trim()
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();
            let op = lines
                .next()?
                .trim()
                .strip_prefix("Operation: new =")?;
            let square = op.starts_with(" old * old");
            let times = op
                .strip_prefix(" old *")
                .map_or(1, |x| dbg!(x.trim()).parse().unwrap_or(0));
            let plus = op
                .strip_prefix(" old +")
                .map_or(0, |x| x.trim().parse().unwrap());
            let divisible = lines
                .next()?
                .trim()
                .strip_prefix("Test: divisible by")?
                .trim()
                .parse()
                .ok()?;
            let if_true = lines
                .next()?
                .trim()
                .strip_prefix("If true: throw to monkey")?
                .trim()
                .parse()
                .ok()?;
            let if_false = lines
                .next()?
                .trim()
                .strip_prefix("If false: throw to monkey")?
                .trim()
                .parse()
                .ok()?;

            let monkey = Monkey {
                items,
                square,
                times,
                plus,
                divisible,
                if_true,
                if_false,
                inspected: 0,
            };
            dbg!(&monkey);
            Some(monkey)
        })
        .collect()
}

fn run_monkeys(monkeys: &mut Vec<Monkey>, very_worry: bool) {
    let times = if very_worry { 10_000 } else { 20 };
    let divide = if very_worry { 1 } else { 3 };
    let commom_divider: u128 =
        monkeys.iter().map(|x| x.divisible).product();
    for _ in 0..times {
        for i in 0..monkeys.len() {
            let d = monkeys[i].divisible;
            let times = monkeys[i].times;
            let plus = monkeys[i].plus;
            let square = monkeys[i].square as u128;
            monkeys[i].inspected += monkeys[i].items.len() as u128;
            let (mut if_true, mut if_false) = monkeys[i]
                .items
                .drain(..)
                .map(|x| {
                    ((x * x * square + x * times + plus) / divide)
                        % commom_divider
                })
                .partition::<Vec<_>, _>(|x| x % d == 0);
            let t = monkeys[i].if_true;
            let f = monkeys[i].if_false;
            monkeys[t].items.append(&mut if_true);
            monkeys[f].items.append(&mut if_false);
        }
    }
}
