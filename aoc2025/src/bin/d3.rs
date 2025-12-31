use aoc2025::read_lines;

fn read_input(path: &str) -> Vec<Vec<u8>> {
    let mut first_row_len: Option<usize> = None;
    let mut output = vec![];
    for line in read_lines(path) {
        let line = line.unwrap();
        if first_row_len == None {
            first_row_len = Some(line.len());
        }

        let line_chars: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        assert_eq!(line_chars.len(), first_row_len.unwrap());
        output.push(line_chars);
    }
    output
}

fn max_2digit_number_in_line(line: &[u8]) -> u8 {
    let (first_ix, first_val) = line[0..line.len() - 1]
        .iter()
        .enumerate()
        // could not use max_by naively because it selects the last value
        // so we work around that by sorting on (val, -ix) to keep the earliest index when values are equal
        .max_by(|x, y| (x.1, -(x.0 as i128)).cmp(&(y.1, -(y.0 as i128))))
        .unwrap();
    let second_val = line[first_ix + 1..line.len()].iter().max().unwrap();
    return 10 * first_val + second_val;
}

fn max_ndigit_number_in_line(n: u8, line: &[u8]) -> u128 {
    let mut total: u128 = 0;
    let mut last_digit_ix = 0;
    let line_len = line.len();
    assert!(n as usize <= line_len);
    for digit_ix in (0..n).rev() {
        let span_end = line.len() - digit_ix as usize;
        let (max_digit_ix_in_subrange, max_digit_val) = line[last_digit_ix..span_end]
            .iter()
            .enumerate()
            // could not use max_by naively because it selects the last value
            // so we work around that by sorting on (val, -ix) to keep the earliest index when values are equal
            .max_by(|x, y| (x.1, -(x.0 as i128)).cmp(&(y.1, -(y.0 as i128))))
            .unwrap();
        total += *max_digit_val as u128 * u128::pow(10, digit_ix.into());
        last_digit_ix = last_digit_ix + max_digit_ix_in_subrange + 1;
    }
    return total;
}

fn main() {
    let data = read_input("data/3/input.txt");
    let result_v1: u128 = data
        .iter()
        .map(|line| max_2digit_number_in_line(&line) as u128)
        .sum();
    println!("{}", result_v1);
    let result_v2: u128 = data
        .iter()
        .map(|line| max_ndigit_number_in_line(12, &line))
        .sum();
    println!("{}", result_v2);
}

#[cfg(test)]
mod tests {
    use crate::{max_2digit_number_in_line, max_ndigit_number_in_line, read_input};

    #[test]
    fn test_max_2digit_number() {
        assert_eq!(max_2digit_number_in_line(&vec![1, 2, 3]), 23);
        assert_eq!(max_2digit_number_in_line(&vec![4, 1, 3, 2, 9]), 49);
        assert_eq!(max_2digit_number_in_line(&vec![2, 3, 4, 7, 8]), 78);
    }

    #[test]
    fn test_max_ndigit_in_line() {
        // assert_eq!(max_ndigit_number_in_line(2, &vec![1,2,3]), 23);
        // assert_eq!(max_ndigit_number_in_line(3, &vec![1,2,3]), 123);
        assert_eq!(
            max_ndigit_number_in_line(12, &vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            987654321111
        );
        assert_eq!(
            max_ndigit_number_in_line(12, &vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            811111111119
        );
        assert_eq!(
            max_ndigit_number_in_line(12, &vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            434234234278
        );
        assert_eq!(
            max_ndigit_number_in_line(12, &vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            888911112111
        );
    }

    #[test]
    fn test_v1_on_ref() {
        let data = read_input("data/3/ref.txt");
        let result: u64 = data
            .iter()
            .map(|line| max_2digit_number_in_line(&line) as u64)
            .sum();
        assert_eq!(result, 357);
    }
}
