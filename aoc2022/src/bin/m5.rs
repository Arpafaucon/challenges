use aoc::read_lines;
use std::{collections::VecDeque, vec::Vec};

type SupplyField = Vec<Vec<char>>;

#[derive(Debug)]
struct CargoInstruction {
    source: i32,
    dest: i32,
    num: i32,
}

impl CargoInstruction {
    fn regex() -> regex::Regex {
        regex::Regex::new(r"^move (?P<num>\d+) from (?P<source>\d+) to (?P<dest>\d)$").unwrap()
    }

    fn from_str(s: impl Iterator<Item = String>, regex: &regex::Regex) -> Vec<Self> {
        s.filter_map(|l| match regex.captures(&l) {
            Some(c) => Some(CargoInstruction {
                source: c.name("source").unwrap().as_str().parse().unwrap(),
                dest: c.name("dest").unwrap().as_str().parse().unwrap(),
                num: c.name("num").unwrap().as_str().parse().unwrap(),
            }),
            None => None,
        })
        .collect::<Vec<Self>>()
    }
}

fn apply_instruction(field: &mut SupplyField, instruction: &CargoInstruction, mode_9001: bool) {
    println!("{:?}", instruction);
    let mut stack: VecDeque<char> = VecDeque::new();
    for _i in 0..instruction.num {
        let cargo_char = field[(instruction.source - 1) as usize].pop().unwrap();
        stack.push_back(cargo_char);
    }

    for _i in 0..instruction.num {
        let char = if mode_9001 {
            stack.pop_back()
        } else {
            stack.pop_front()
        }
        .unwrap();
        field[(instruction.dest - 1) as usize].push(char);
    }
}

fn print_field(field: &SupplyField) {
    for (stack_ix, stack_content) in field.iter().enumerate() {
        print!("s {} [{}]: ", stack_ix, stack_content.len());
        for cargo in stack_content {
            print!("{} ", cargo);
        }
        println!("");
    }
}

fn top_of_field(field: &SupplyField) -> String {
    print_field(&field);
    let mut out = String::new();
    for stack_content in field.iter() {
        if let Some(stack_top) = stack_content.last() {
            out.push(stack_top.to_owned());
        }
    }
    out
}

fn main() {
    let filename = "./data/5a.txt";
    let mode_9001 = true;

    let mut line_iter = read_lines(filename).map(|l| l.unwrap());

    let mut stack_lines: Vec<String> = line_iter
        .by_ref()
        .take_while(|l| {
            // println!("{}", l);
            !l.is_empty()
        })
        .collect();

    let stack_layout = stack_lines.pop().unwrap();
    let num_stacks: i32 = String::from(stack_layout.chars().last().unwrap())
        .parse()
        .unwrap();
    println!("Detected {} stacks", num_stacks);

    let mut field: SupplyField = Vec::new();
    for _i in 0..num_stacks {
        field.push(vec![]);
    }
    for l in stack_lines.iter().rev() {
        // println!("Parsing line '{}'", l);
        let mut stack_ix = 0;
        let mut char_ix = 1;
        let line_chars: Vec<char> = l.chars().collect();
        let line_len = line_chars.len();
        while char_ix < line_len {
            // println!("Processing s{} c{}", stack_ix, char_ix);
            let cargo_letter = line_chars[char_ix];
            // ignore the spaces
            if cargo_letter.is_alphabetic() {
                field[stack_ix].push(cargo_letter);
            }

            // next
            stack_ix += 1;
            char_ix += 4; // 3 in-between chars ']', ' ', '['
        }
    }
    println!("Stack parsing result");
    print_field(&field);

    let operations = CargoInstruction::from_str(line_iter, &CargoInstruction::regex());
    println!("operations: {}", operations.len());

    for o in &operations {
        apply_instruction(&mut field, o, mode_9001);
        print_field(&field);
    }
    println!("_____________________");
    println!("top: {}", top_of_field(&field));

    // println!("operations");
    // for o in operations {
    //     println!("{:?}", o);
    // }
}
