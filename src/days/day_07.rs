use crate::utils::*;

#[derive(Debug, Clone)]
enum Value {
    Wire(String),
    Signal(u16),
}
use Value::*;
impl std::str::FromStr for Value {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = u16::from_str(s) {
            Ok(Signal(v))
        } else {
            Ok(Wire(s.to_string()))
        }
    }
}
impl RegexRepresentation for Value {
    const REGEX: &'static str = r"[a-z]+|\d+";
}

enum Instruction {
    Assign(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, Value),
    RShift(Value, Value),
    Not(Value),
}
use Instruction::*;
impl std::str::FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = if let Ok(a) = scanf!(s, "{Value}") {
            Assign(a)
        } else if let Ok((a, b)) = scanf!(s, "{Value} AND {Value}") {
            And(a, b)
        } else if let Ok((a, b)) = scanf!(s, "{Value} OR {Value}") {
            Or(a, b)
        } else if let Ok((a, b)) = scanf!(s, "{Value} LSHIFT {Value}") {
            LShift(a, b)
        } else if let Ok((a, b)) = scanf!(s, "{Value} RSHIFT {Value}") {
            RShift(a, b)
        } else if let Ok(a) = scanf!(s, "NOT {Value}") {
            Not(a)
        } else {
            return Err(format!("Could not parse instruction: {}", s));
        };
        Ok(ret)
    }
}
impl RegexRepresentation for Instruction {
    const REGEX: &'static str = r".*?";
}

fn get_value(
    val: &Value,
    wires: &mut HashMap<String, u16>,
    conn: &HashMap<&'static str, Instruction>,
) -> u16 {
    match val {
        Wire(name) => get_wire(&name, wires, conn),
        Signal(val) => *val,
    }
}

fn get_wire(
    name: &str,
    wires: &mut HashMap<String, u16>,
    conn: &HashMap<&'static str, Instruction>,
) -> u16 {
    if let Some(v) = wires.get(name) {
        return *v;
    }
    let val = match &conn[name] {
        Assign(a) => get_value(a, wires, conn),
        And(a, b) => get_value(a, wires, conn) & get_value(b, wires, conn),
        Or(a, b) => get_value(a, wires, conn) | get_value(b, wires, conn),
        LShift(a, b) => get_value(a, wires, conn) << get_value(b, wires, conn),
        RShift(a, b) => get_value(a, wires, conn) >> get_value(b, wires, conn),
        Not(a) => !get_value(a, wires, conn),
    };
    wires.insert(name.to_string(), val);
    val
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let mut wires = HashMap::new();
    let mut conn = input
        .lines()
        .map(|l| scanf!(*l, "{Instruction} -> {str}").unwrap())
        .map(|(instr, out)| (out, instr))
        .to_map();

    let a = get_wire("a", &mut wires, &conn);
    wires.clear();
    conn.insert("b", Assign(Signal(a)));

    pv!(get_wire("a", &mut wires, &conn));
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let mut wires = HashMap::new();
    let conn = input
        .lines()
        .map(|l| scanf!(*l, "{Instruction} -> {str}").unwrap())
        .map(|(instr, out)| (out, instr))
        .to_map();

    pv!(get_wire("a", &mut wires, &conn));
}
