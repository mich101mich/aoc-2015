use crate::utils::*;

fn register(r: char) -> usize {
    r as usize - 'a' as usize
}

enum Instruction {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}
use Instruction::*;
impl std::str::FromStr for Instruction {
    type Err = ();
    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let ret = if let Ok(reg) = scanf!(l, "hlf {char}") {
            Hlf(register(reg))
        } else if let Ok(reg) = scanf!(l, "tpl {char}") {
            Tpl(register(reg))
        } else if let Ok(reg) = scanf!(l, "inc {char}") {
            Inc(register(reg))
        } else if let Ok(offset) = scanf!(l, "jmp {isize}") {
            Jmp(offset)
        } else if let Ok((reg, offset)) = scanf!(l, "jie {char}, {isize}") {
            Jie(register(reg), offset)
        } else if let Ok((reg, offset)) = scanf!(l, "jio {char}, {isize}") {
            Jio(register(reg), offset)
        } else {
            return Err(());
        };
        Ok(ret)
    }
}

fn execute(instructions: &[Instruction], registers: &mut [usize]) {
    let mut ip = 0isize;
    loop {
        if ip < 0 || ip as usize >= instructions.len() {
            break;
        }
        match instructions[ip as usize] {
            Hlf(r) => registers[r] /= 2,
            Tpl(r) => registers[r] *= 3,
            Inc(r) => registers[r] += 1,
            Jmp(offset) => {
                ip += offset;
                continue;
            }
            Jie(r, offset) => {
                if registers[r] % 2 == 0 {
                    ip += offset;
                    continue;
                }
            }
            Jio(r, offset) => {
                if registers[r] == 1 {
                    ip += offset;
                    continue;
                }
            }
        }
        ip += 1;
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let instructions = input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .to_vec();

    let mut registers = [1, 0];

    execute(&instructions, &mut registers);

    pv!(registers[1]);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let instructions = input
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .to_vec();

    let mut registers = [0, 0];

    execute(&instructions, &mut registers);

    pv!(registers[1]);
}
