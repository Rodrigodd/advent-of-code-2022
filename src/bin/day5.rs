const INPUT: &str = include_str!("../../inputs/day5.txt");

struct Input {
    stacks: Vec<Vec<u8>>,
    moves: Vec<(usize, usize, usize)>,
}
impl Input {
    fn top(&self) -> Vec<u8> {
        self.stacks
            .iter()
            .filter_map(|x| x.last())
            .copied()
            .collect()
    }
}

fn main() {
    let input = to_input(INPUT);
    let input = apply_moves_9000(input);
    let top = input.top();
    let top = std::str::from_utf8(&top).unwrap();
    println!("top is: {top}");

    let input = to_input(INPUT);
    let input = apply_moves_9001(input);
    let top = input.top();
    let top = std::str::from_utf8(&top).unwrap();
    println!("top is: {top}");
}

#[test]
fn example() {
    let input_s = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   %   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    let input = to_input(input_s);
    let input = apply_moves_9000(input);
    assert_eq!(b"CMZ", input.top().as_slice());

    let input = to_input(input_s);
    let input = apply_moves_9001(input);
    assert_eq!(b"MCD", input.top().as_slice());
}

fn to_input(input: &str) -> Input {
    let (drawing, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = Vec::new();
    for line in drawing.split_inclusive('\n') {
        for (column, crate_) in
            line.as_bytes().chunks_exact(4).enumerate()
        {
            if !crate_[1].is_ascii_whitespace() {
                if stacks.len() < column + 1 {
                    stacks.resize(column + 1, Vec::new());
                }
                stacks[column].push(crate_[1]);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }

    let moves = moves
        .lines()
        .map(|x| {
            let mut words = x.split_whitespace();
            let _ = words.next();
            let count: usize = words.next().unwrap().parse().unwrap();
            let _ = words.next();
            let from: usize = words.next().unwrap().parse().unwrap();
            let _ = words.next();
            let to: usize = words.next().unwrap().parse().unwrap();
            (count, from - 1, to - 1)
        })
        .collect();

    Input { stacks, moves }
}

fn apply_moves_9000(input: Input) -> Input {
    let Input { mut stacks, moves } = input;
    for (count, from, to) in moves {
        for _ in 0..count {
            let crate_ = stacks[from].pop().unwrap();
            stacks[to].push(crate_);
        }
    }
    Input {
        stacks,
        moves: Vec::new(),
    }
}

fn apply_moves_9001(input: Input) -> Input {
    let Input { mut stacks, moves } = input;
    for (count, from, to) in moves {
        let (from, to) = if from < to {
            let (a, b) = stacks.split_at_mut(to);
            (&mut a[from], &mut b[0])
        } else {
            let (a, b) = stacks.split_at_mut(from);
            (&mut b[0], &mut a[to])
        };
        let l = from.len();
        let crates = from.drain(l - count..l);
        to.extend(crates);
    }
    Input {
        stacks,
        moves: Vec::new(),
    }
}
