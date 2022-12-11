const INPUT: &str = include_str!("../../inputs/day10.txt");

fn main() {
    let input = INPUT;
    let instructions = to_instructions(input);
    let mut program = Program::default();
    let mut strenght = 0;
    for instr in instructions {
        program.do_step(instr);
        if (program.cycles + 20) % 40 == 0 {
            strenght += program.cycles * program.x;
            println!("{}", program.x)
        }
    }
    println!("strength sums are: {}", strenght);
}

#[test]
fn example() {
    let input = 
"addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13
addx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5
addx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1
addx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop
addx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop
noop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7
noop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop
noop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop
addx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop
noop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1
addx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3
addx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop
addx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1
addx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1
addx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1
addx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22
addx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop
noop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop
noop\nnoop";

    let instructions = to_instructions(input);
    let mut program = Program::default();
    let mut strenght = 0;
    for instr in instructions {
        program.do_step(instr);
        if (program.cycles + 20) % 40 == 0 {
            strenght += program.cycles * program.x;
            println!("{}", program.x)
        }
    }
    assert_eq!(13140, strenght)
}

struct Program {
    cycles: i32,
    x: i32,
}

impl Default for Program {
    fn default() -> Self {
        Self { cycles: 1, x: 1 }
    }
}
impl Program {
    fn do_step(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::AddX(dx) => {
                self.x += dx;
            }
            Instruction::Noop => {}
        }
        self.cycles += 1;
    }
}

enum Instruction {
    AddX(i32),
    Noop,
}

fn to_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for x in input.lines() {
        if let Some(addx) = x.strip_prefix("addx") {
            let count = addx.trim().parse().unwrap();
            instructions.push(Instruction::Noop);
            instructions.push(Instruction::AddX(count));
        } else if x.starts_with("noop") {
            instructions.push(Instruction::Noop);
        } else {
            panic!("unknow op '{x}'")
        }
    }
    instructions
}
