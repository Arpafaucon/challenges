use aoc2025::read_lines;

const DIAL_MOD: i16 = 100;

fn unpack_instruction(line: &str) -> i32 {
    let direction = line.chars().nth(0).unwrap();
    let multiplier: i16 = if direction == 'R' { 1 } else { -1 };
    let qty = line[1..].parse::<i16>().unwrap();
    return (qty * multiplier).try_into().unwrap();
}

/// Rotate dial, and count the number of zero crossing
/// We count them when we arrive on the digit 0
fn rotate_dial(current: i16, offset: i32) -> (i16, u16) {
    let mut current = current;
    let mut crossings = 0;
    if offset > 0 {
        let mut remaining_offset = offset;
        while remaining_offset > 0 {
            let clicks_to_next_crossing = DIAL_MOD - current;
            let move_clicks: i16 = if remaining_offset >= clicks_to_next_crossing.into() {
                crossings += 1;
                clicks_to_next_crossing
            } else {
                remaining_offset as i16 // cast OK, since we're < clicks_to_next_threshold
            };

            current = (current + move_clicks).rem_euclid(DIAL_MOD);
            remaining_offset -= move_clicks as i32;
        }
    } else if offset < 0 {
        let mut remaining_clicks = -offset;
        while remaining_clicks > 0 {
            let clicks_to_next_crossing = if current != 0 { current } else { DIAL_MOD };
            let move_clicks = if remaining_clicks >= clicks_to_next_crossing.into() {
                crossings += 1;
                clicks_to_next_crossing
            } else {
                remaining_clicks as i16 // cast OK, since we're < clicks_to_next_threshold
            };
            current = (current - move_clicks).rem_euclid(DIAL_MOD); // naive modulo returns in range [-DIAL_MOD; DIAL_MOD-1]
            remaining_clicks -= move_clicks as i32;
        }
    }
    return (current, crossings);
}

fn process_dial_v1(path: &str) -> u16 {
    let mut current_pos: i16 = 50;
    let mut times_pointing_zero: u16 = 0;
    for line in read_lines(path) {
        if line.is_err() {
            break;
        }
        let content = line.unwrap();
        let offset = unpack_instruction(&content);
        let (new_pos, _crossings) = rotate_dial(current_pos, offset);
        current_pos = new_pos;
        if current_pos == 0 {
            times_pointing_zero += 1;
        }
    }
    return times_pointing_zero;
}

fn process_dial_v2(path: &str) -> u16 {
    let mut current_pos: i16 = 50;
    let mut times_pointing_zero: u16 = 0;
    for line in read_lines(path) {
        if line.is_err() {
            break;
        }
        let content = line.unwrap();
        let offset = unpack_instruction(&content);
        let (new_pos, crossings) = rotate_dial(current_pos, offset);
        current_pos = new_pos;
        times_pointing_zero += crossings;
    }
    return times_pointing_zero;
}

fn main() {
    let times_pointing_zero = process_dial_v1("data/1/input.txt");
    println!("v1: {}", times_pointing_zero);
    println!("v2: {}", process_dial_v2("data/1/input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::rotate_dial;

    use super::process_dial_v1;

    #[test]
    fn test_example_v1() {
        let out = process_dial_v1("data/1/ref.txt");
        assert_eq!(out, 3);
    }

    #[test]
    fn test_rotate_dial() {
        assert_eq!(rotate_dial(0, 0), (0, 0));
        assert_eq!(rotate_dial(0, 99), (99, 0));
        assert_eq!(rotate_dial(0, 100), (0, 1));
        assert_eq!(rotate_dial(0, 199), (99, 1));
        assert_eq!(rotate_dial(0, -1), (99, 0));
        assert_eq!(rotate_dial(0, -99), (1, 0));
        assert_eq!(rotate_dial(0, -100), (0, 1));
        assert_eq!(rotate_dial(0, -101), (99, 1));
        assert_eq!(rotate_dial(99, 1), (0, 1));
        assert_eq!(rotate_dial(99, 101), (0, 2));
    }
}
