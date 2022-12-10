const INPUT: &str = include_str!("../../inputs/day9.txt");

fn main() {
    let input = INPUT;
    let steps = to_steps(input);
    let positions = simulate(steps);
    let count = positions
        .iter()
        .map(|x| x.tail)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    println!("unique positions of tail: {count}");
}

#[test]
fn example() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    let steps = to_steps(input);
    let positions = simulate(steps);
    let count = positions
        .iter()
        .map(|x| x.tail)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    assert_eq!(13, count);
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Rope {
    head: Pos,
    tail: Pos,
}
impl Rope {
    fn len(&self) -> i32 {
        let dx = (self.head.x - self.tail.x).abs();
        let dy = (self.head.y - self.tail.y).abs();
        dx.max(dy)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

fn pos(x: i32, y: i32) -> Pos {
    Pos { x, y }
}

fn simulate(steps: Vec<Step>) -> Vec<Rope> {
    let mut positions = Vec::new();
    positions.push(Rope {
        head: pos(0, 0),
        tail: pos(0, 0),
    });
    for step in steps {
        for _ in 0..step.count {
            let curr = positions.last().unwrap();
            let mut next = *curr;

            match step.direction {
                0 => next.head.x += 1,
                1 => next.head.y += 1,
                2 => next.head.x -= 1,
                3 => next.head.y -= 1,
                dir => panic!("unkown direction {dir}"),
            }
            if next.len() > 1 {
                next.tail = curr.head;
            }
            positions.push(next);
        }
    }
    positions
}

struct Step {
    // 0 to 3, indicates +x, +y, -x, -y
    direction: u8,
    count: u32,
}

fn to_steps(input: &str) -> Vec<Step> {
    input
        .lines()
        .map(|x| {
            let (dir, count) = x.split_once(' ').unwrap();
            Step {
                direction: match dir {
                    "R" => 0,
                    "D" => 1,
                    "L" => 2,
                    "U" => 3,
                    _ => panic!("unexpected dir: {dir}"),
                },
                count: count.parse().unwrap(),
            }
        })
        .collect()
}
