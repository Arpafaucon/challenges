use aoc::{read_lines, to_line_iterator};
use std::{collections::HashSet, hash::Hash};

fn char_score(c: &char) -> i32 {
    if c.is_ascii_lowercase() {
        return 1 + (*c as i32 - 'a' as i32);
    } else if c.is_ascii_uppercase() {
        return 27 + (*c as i32 - 'A' as i32);
    } else {
        panic!("Bad letter");
    }
}

fn elf_common_char(elf_line: &str) -> char {
    let line_len = elf_line.len();
    let sack_len = line_len / 2;
    let line_chars: Vec<char> = elf_line.chars().collect();
    let first_sack: HashSet<char> = line_chars[0..sack_len].iter().cloned().collect();
    let second_sack: HashSet<char> = line_chars[sack_len..line_len].iter().cloned().collect();
    let common_char = first_sack.intersection(&second_sack).next().unwrap();
    assert!(common_char.is_ascii_alphabetic());
    common_char.clone()
}

fn main() {
    const ELF_GROUP_SIZE: usize = 3;
    let mut elf_individual_score: i32 = 0;
    let mut elf_group_score: i32 = 0;

    let lines: Vec<String> = to_line_iterator(read_lines("./data/3a.txt")).collect();
    assert_eq!(lines.len() % ELF_GROUP_SIZE, 0);

    for elf_group_lines in lines.chunks(ELF_GROUP_SIZE) {
        assert_eq!(elf_group_lines.len(), ELF_GROUP_SIZE);
        // answer for question A
        for elf_line in elf_group_lines {
            elf_individual_score += char_score(&elf_common_char(&elf_line));
        }

        // answer for question B
        let group_items: Vec<HashSet<char>> = elf_group_lines
            .iter()
            .map(|l| l.chars().collect())
            .collect();

        let token_set = multiple_set_intersection(&group_items);
        let token = token_set.iter().next().unwrap();
        elf_group_score += char_score(token);
    }

    println!("Individual score: {}", elf_individual_score);
    println!("Group score: {}", elf_group_score);
}

/// Intersection of the multiple sets
/// Using https://github.com/rust-lang/rfcs/issues/2023#issuecomment-739483074
fn multiple_set_intersection<T>(sets: &[HashSet<T>]) -> HashSet<T>
where
    T: Clone + Eq + Hash,
{
    let token_set = sets.iter().skip(1).fold(sets[0].clone(), |acc, e| {
        acc.intersection(e).cloned().collect()
    });
    token_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_set_intersection() {
        let mut sets: Vec<HashSet<char>> = Vec::new();
        sets.push(['a', 'b', 'c', 'd', 'e'].iter().cloned().collect());
        sets.push(['c', 'd', 'e'].iter().cloned().collect());
        sets.push(['d', 'a', 'e'].iter().cloned().collect());
        sets.push(['d', 'e', 'f', 'g'].iter().cloned().collect());

        let expected = HashSet::from(['d', 'e']);
        let actual = multiple_set_intersection(&sets);

        assert_eq!(actual, expected);
    }
}
