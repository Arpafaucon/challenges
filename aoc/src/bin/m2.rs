use aoc::read_lines;

#[derive(Debug, Clone, Copy)]
enum ElfMove {
    ROCK,
    PAPER,
    SCISSORS,
}

impl ElfMove {
    fn from_char(c: char) -> ElfMove {
        match c {
            'A' | 'X' => ElfMove::ROCK,
            'B' | 'Y' => ElfMove::PAPER,
            'C' | 'Z' => ElfMove::SCISSORS,
            _ => panic!("unsupported"),
        }
    }

    fn to_int(self: &Self) -> i8 {
        match self {
            ElfMove::ROCK => 1,
            ElfMove::PAPER => 2,
            ElfMove::SCISSORS => 3,
        }
    }

    fn from_int(i: i8) -> Option<ElfMove> {
        match i {
            1 => Some(ElfMove::ROCK),
            2 => Some(ElfMove::PAPER),
            3 => Some(ElfMove::SCISSORS),

            _ => None,
        }
    }

    fn duel_score(me: ElfMove, opponent: ElfMove) -> i8 {
        let diff = (me.to_int() - opponent.to_int()).rem_euclid(3);
        println!("{}", diff);
        match diff {
            0 => 3,
            1 => 6, // win
            2 => 0,
            _ => panic!("should not happen"),
        }
    }
}

fn strategy_a(mut line_chars: std::str::Chars) -> i8 {
    let opponent_move = ElfMove::from_char(line_chars.nth(0).unwrap());
    assert_eq!(line_chars.nth(0), Some(' '));
    let our_move = ElfMove::from_char(line_chars.nth(0).unwrap());
    let match_score = our_move.to_int() + ElfMove::duel_score(our_move, opponent_move);
    println!("{:?} / {:?} -> {}", our_move, opponent_move, match_score);
    match_score
}

fn main() {
    let lines = read_lines("./data/2a.txt").unwrap();
    let mut total_score: i64 = 0;
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            let match_score = strategy_a(ip.chars());
            total_score += match_score as i64;
        }
    }
    println!("Total score {}", total_score);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_elf_move() {
        assert_eq!(ElfMove::duel_score(ElfMove::ROCK, ElfMove::PAPER), 0);
        assert_eq!(ElfMove::duel_score(ElfMove::ROCK, ElfMove::SCISSORS), 6);
        assert_eq!(ElfMove::duel_score(ElfMove::SCISSORS, ElfMove::SCISSORS), 3);
    }
}
