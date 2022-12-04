use aoc::read_lines;
use std::collections::HashSet;

fn char_score(c: &char) -> i32 {
    if c.is_ascii_lowercase() {
        return 1 + (*c as i32 - 'a' as i32);
    } else if c.is_ascii_uppercase() {
        return 27 + (*c as i32 - 'A' as i32);
    } else {
        panic!("Bad letter");
    }
}
fn main() {
    let mut elf_duplicates: Vec<char> = vec![];

    let lines = read_lines("./data/3a.txt").unwrap();
    for l in lines {
        if let Ok(line_content) = l {
            let line_len = line_content.len();
            if line_len == 0 {
                continue;
            }
            let sack_len = line_len / 2;
            let line_chars: Vec<char> = line_content.chars().collect();
            let first_sack: HashSet<char> = line_chars[0..sack_len].iter().cloned().collect();
            let second_sack: HashSet<char> =
                line_chars[sack_len..line_len].iter().cloned().collect();
            let common_char = first_sack
                .intersection(&second_sack)
                .next()
                .unwrap()
                .clone();
            assert!(common_char.is_ascii_alphabetic());
            elf_duplicates.push(common_char);
        }
    }

    let total_score:i32 = elf_duplicates.iter().map(|r| char_score(r)).sum();
    println!("Total score: {}", total_score);
}
