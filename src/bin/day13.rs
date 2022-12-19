const INPUT: &str = include_str!("../../inputs/day13.txt");

fn main() {
    let input = INPUT;
    let input = parse_input(input);
    let ordereds = find_ordereds(&input);
    println!("the sum is: {}", ordereds.iter().sum::<usize>());
    println!("decoder key: {}", decoder_key(input));
}

#[test]
fn example() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    let input = parse_input(input);
    let ordereds = find_ordereds(&input);
    assert_eq!(&[1, 2, 4, 6][..], ordereds.as_slice());
    assert_eq!(140, decoder_key(input));
}

fn find_ordereds(input: &[(Value, Value)]) -> Vec<usize> {
    let input: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(
            |(i, (a, b))| if a <= b { Some(i + 1) } else { None },
        )
        .collect();
    input
}

fn decoder_key(input: Vec<(Value, Value)>) -> usize {
    let a = Value::List(vec![Value::List(vec![Value::Int(2)])]);
    let b = Value::List(vec![Value::List(vec![Value::Int(6)])]);
    let mut packets: Vec<Value> = input
        .into_iter()
        .chain([(a.clone(), b.clone())].into_iter())
        .flat_map(|(a, b)| [a, b].into_iter())
        .collect();

    packets.sort();

    let a = packets.iter().position(|x| *x == a).unwrap();
    let b = packets.iter().position(|x| *x == b).unwrap();

    (a + 1) * (b + 1)
}

#[derive(Clone)]
enum Value {
    Int(u32),
    List(Vec<Value>),
}
impl PartialOrd for Value {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Int(a), b @ Value::List(_)) => {
                Value::List(vec![Value::Int(*a)]).partial_cmp(b)
            }
            (a @ Value::List(_), Value::Int(b)) => {
                a.partial_cmp(&Value::List(vec![Value::Int(*b)]))
            }
            (Value::List(a), Value::List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    let cmp = a.partial_cmp(b)?;
                    if !cmp.is_eq() {
                        return Some(cmp);
                    }
                }
                a.len().partial_cmp(&b.len())
            }
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Eq for Value {}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_value(mut s: &str) -> (Value, &str) {
    if s.starts_with('[') {
        s = &s[1..]; // strip '['
        let mut list = Vec::new();
        if s.starts_with(']') {
            s = &s[1..];
            return (Value::List(list), s);
        }
        loop {
            let (v, t) = parse_value(s);
            s = t;
            list.push(v);
            if s.starts_with(']') {
                s = &s[1..];
                break;
            }
            s = &s[1..]; // strip ','
        }
        (Value::List(list), s)
    } else {
        let (i, _) =
            s.char_indices().find(|x| !x.1.is_ascii_digit()).unwrap();
        (Value::Int(s[0..i].parse().unwrap()), &s[i..])
    }
}

fn parse_input(input: &str) -> Vec<(Value, Value)> {
    input
        .split("\n\n")
        .map(|x| {
            let mut lines = x.lines();
            (
                parse_value(lines.next().unwrap()).0,
                parse_value(lines.next().unwrap()).0,
            )
        })
        .collect()
}
