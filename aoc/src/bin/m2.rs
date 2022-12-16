use aoc::read_lines;

#[derive(Debug, Clone, Copy)]
enum ElfMove {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(Debug, Clone, Copy)]
enum DuelStatus {
    LOSE,
    DRAW,
    WIN,
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

    fn duel_score(me: ElfMove, opponent: ElfMove) -> DuelStatus {
        let diff = (me.to_int() - opponent.to_int()).rem_euclid(3);
        println!("{}", diff);
        match diff {
            0 => DuelStatus::DRAW,
            1 => DuelStatus::WIN,
            2 => DuelStatus::LOSE,
            _ => panic!("should not happen"),
        }
    }

    fn build_from_target_status(opponent: ElfMove, status: DuelStatus) -> ElfMove {
        let offset = match status {
            DuelStatus::DRAW => 0,
            DuelStatus::WIN => 1,
            DuelStatus::LOSE => -1,
        };
        let our_move_ix = 1 + (opponent.to_int() - 1 + offset).rem_euclid(3);
        return ElfMove::from_int(our_move_ix).unwrap();
    }
}

impl DuelStatus {
    fn from_char(c: char) -> Option<DuelStatus> {
        match c {
            'X' => Some(Self::LOSE),
            'Y' => Some(Self::DRAW),
            'Z' => Some(Self::WIN),
            _ => None,
        }
    }

    fn score(self: Self) -> i8 {
        match self {
            Self::LOSE => 0,
            Self::DRAW => 3,
            Self::WIN => 6,
        }
    }
}

fn strategy_a(mut line_chars: std::str::Chars) -> i8 {
    let opponent_move = ElfMove::from_char(line_chars.nth(0).unwrap());
    assert_eq!(line_chars.nth(0), Some(' '));
    let our_move = ElfMove::from_char(line_chars.nth(0).unwrap());
    let match_score = our_move.to_int() + ElfMove::duel_score(our_move, opponent_move).score();
    println!("{:?} / {:?} -> {}", our_move, opponent_move, match_score);
    match_score
}

fn strategy_b(mut line_chars: std::str::Chars) -> i8 {
    let opponent_move = ElfMove::from_char(line_chars.nth(0).unwrap());
    assert_eq!(line_chars.nth(0), Some(' '));
    let target_status = DuelStatus::from_char(line_chars.nth(0).unwrap()).unwrap();
    let our_move = ElfMove::build_from_target_status(opponent_move, target_status);

    let match_score = our_move.to_int() + target_status.score();
    println!("{:?} / {:?} -> {}", our_move, opponent_move, match_score);
    match_score
}

fn main() {
    let lines = read_lines("./data/2a.txt");
    let mut total_score: i64 = 0;
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            let match_score = strategy_b(ip.chars());
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
        assert_eq!(ElfMove::duel_score(ElfMove::ROCK, ElfMove::PAPER).score(), 0);
        assert_eq!(ElfMove::duel_score(ElfMove::ROCK, ElfMove::SCISSORS).score(), 6);
        assert_eq!(ElfMove::duel_score(ElfMove::SCISSORS, ElfMove::SCISSORS).score(), 3);
    }
}
