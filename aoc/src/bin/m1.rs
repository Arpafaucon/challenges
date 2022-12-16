
use aoc::read_lines;

fn main() {
    let mut current_elf_calories: i64 = 0;
    let mut top_elf_calories: Vec<i64> = vec![];
    // File hosts must exist in current path before this produces output

    let lines = read_lines("./data/1a.txt");
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            println!("l: {} [ce {}]", ip, current_elf_calories);
            current_elf_calories = match ip.parse::<i64>() {
                Ok(qty) => current_elf_calories + qty,
                Err(_) => {
                    // next elf !
                    update_stack(&mut top_elf_calories, current_elf_calories);
                    0
                }
            };
        }
    }

    // finished parsing line: process the last elf
    update_stack(&mut top_elf_calories, current_elf_calories);

    println!("{:?}", top_elf_calories);
    let sum: i64 = top_elf_calories.iter().sum();
    println!("sum {:}", sum);
}

fn update_stack(stack: &mut Vec<i64>, input: i64) {
    if stack.len() < 3 {
        stack.push(input)
    } else {
        // find stack_min and its index
        let stack_min = stack.iter().min().unwrap();
        let stack_min_ix = stack.iter().position(|&r| r == *stack_min).unwrap();

        // replace it
        if input > *stack_min {
            stack[stack_min_ix] = input;
        }
    }
    println!("s {:?}", stack);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_update_stack() {
        let mut stack = vec![];
        update_stack(&mut stack, 1);
        assert_eq!(stack, vec![1]);
        update_stack(&mut stack, 2);
        assert_eq!(stack, vec![1, 2]);
        update_stack(&mut stack, 3);
        assert_eq!(stack, vec![1, 2, 3]);
        update_stack(&mut stack, 5);
        assert_eq!(stack, vec![5, 2, 3]);
    }
}
