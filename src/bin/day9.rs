const INPUT: &str = include_str!("../../inputs/day9.txt");

fn main() {
    let input = INPUT;

    let steps = to_steps(input);
    let positions = simulate(steps, 2);
    let count = positions
        .iter()
        .map(|x| *x.knots.last().unwrap())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    println!("unique positions of tail of 2-rope: {count}");

    let steps = to_steps(input);
    let positions = simulate(steps, 10);
    let count = positions
        .iter()
        .map(|x| *x.knots.last().unwrap())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    println!("unique positions of tail of 10-rope: {count}");
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
    let positions = simulate(steps, 2);
    let count = positions
        .iter()
        .map(|x| *x.knots.last().unwrap())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    assert_eq!(13, count);
}

#[test]
fn example2() {
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    let steps = to_steps(input);
    let positions = simulate(steps, 10);
    let mut tail = positions
        .iter()
        .map(|x| (x.knots[0], x.knots[1], x.knots[2]))
        .collect::<Vec<_>>();
    tail.dedup();
    for (i, x) in tail.iter().enumerate() {
        println!("{i:2}: {:?}", x);
    }
    let collect = positions
        .iter()
        .map(|x| *x.knots.last().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    let count = collect.len();
    assert_eq!(36, count);
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Rope {
    knots: Vec<Pos>,
}

#[derive(
    Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug,
)]
struct Pos {
    x: i32,
    y: i32,
}

fn pos(x: i32, y: i32) -> Pos {
    Pos { x, y }
}

fn simulate(steps: Vec<Step>, knots: usize) -> Vec<Rope> {
    let mut positions = Vec::new();
    if knots < 1 {
        unimplemented!();
    }
    positions.push(Rope {
        knots: vec![pos(0, 0); knots],
    });
    for step in steps {
        for _ in 0..step.count {
            let curr = positions.last().unwrap();
            let mut next = curr.clone();

            match step.direction {
                0 => next.knots[0].x += 1,
                1 => next.knots[0].y += 1,
                2 => next.knots[0].x -= 1,
                3 => next.knots[0].y -= 1,
                dir => panic!("unkown direction {dir}"),
            }
            for i in 1..knots {
                let a = next.knots[i];
                let b = next.knots[i - 1];
                let dx = b.x - a.x;
                let dy = b.y - a.y;
                if dx.abs() > 1 || dy.abs() > 1 {
                    next.knots[i].x += dx.signum();
                    next.knots[i].y += dy.signum();
                }
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
