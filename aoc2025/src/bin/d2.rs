use std::fs::read_to_string;

fn is_invalid_id_v1(number: u128) -> bool {
    let number_repr = number.to_string();
    let num_chars = number_repr.len();
    if num_chars % 2 != 0 {
        return false;
    }
    let mut first_half = number_repr;
    let second_half = first_half.split_off(num_chars / 2);
    return first_half == second_half;
}

fn is_repeated(input: &str, num_copies: usize) -> bool {
    assert!(num_copies > 1);
    let input_chars: Vec<u8> = input.bytes().collect(); // we expect numbers, so iterating on bytes is the same
    let input_len = input_chars.len();
    assert_eq!(input_len % num_copies, 0);
    assert!(input_len > 0);

    let copy_len = input_len / num_copies;
    for copy_ix in 1..num_copies {
        for char in 0..copy_len {
            if input_chars[char] != input_chars[copy_ix * copy_len + char] {
                return false;
            }
        }
    }
    return true;
}

fn is_invalid_id_v2(number: u128) -> bool {
    let number_repr = number.to_string();
    let num_chars = number_repr.len();
    for i in 2..=num_chars {
        if num_chars % i == 0 {
            if is_repeated(&number_repr, i) {
                return true;
            }
        }
    }
    return false;
}

fn list_invalid_ids_in_range(start: u128, end: u128, is_invalid: fn(u128) -> bool) -> Vec<u128> {
    (start..=end).filter(|val| is_invalid(*val)).collect()
}

fn get_ranges_from_file(path: &str) -> Vec<(u128, u128)> {
    let ranges: Vec<(u128, u128)> = read_to_string(path)
        .unwrap()
        .split(',')
        .map(|item| {
            let (start_str, end_str) = item.trim().split_once("-").unwrap();
            return (
                start_str.parse::<u128>().unwrap(),
                end_str.parse::<u128>().unwrap(),
            );
        })
        .collect();
    return ranges;
}

fn process_invalid_id(path: &str, invalid_id_detector: fn(u128) -> bool) -> u128 {
    let mut sum = 0;
    for (rmin, rmax) in get_ranges_from_file(path) {
        let invalid_ids = list_invalid_ids_in_range(rmin, rmax, invalid_id_detector);
        sum += invalid_ids.iter().sum::<u128>();
    }
    return sum;
}

fn main() {
    let path = "data/2/input.txt";
    let sum_v1 = process_invalid_id(path, is_invalid_id_v1);
    println!("Sum V1: {}", sum_v1);
    let sum_v2 = process_invalid_id(path, is_invalid_id_v2);
    println!("Sum V2: {}", sum_v2);
}

#[cfg(test)]
mod tests {
    use crate::{
        is_invalid_id_v1, is_invalid_id_v2, is_repeated, list_invalid_ids_in_range,
        process_invalid_id,
    };

    #[test]
    fn test_is_invalid_id_v1() {
        assert_eq!(is_invalid_id_v1(0), false);
        assert_eq!(is_invalid_id_v1(1), false);
        assert_eq!(is_invalid_id_v1(11), true);
        assert_eq!(is_invalid_id_v1(121), false);
        assert_eq!(is_invalid_id_v1(123123), true);
    }

    #[test]
    fn test_is_repeated() {
        assert!(is_repeated("123123", 2));
        assert!(!is_repeated("123123", 3));
        assert!(is_repeated("121212", 3));
        assert!(!is_repeated("112", 3))
    }

    #[test]
    fn test_check_range_v1() {
        assert_eq!(list_invalid_ids_in_range(0, 1, is_invalid_id_v1), vec![]);
        assert_eq!(list_invalid_ids_in_range(0, 11, is_invalid_id_v1), vec![11]);
        assert_eq!(
            list_invalid_ids_in_range(11, 22, is_invalid_id_v1),
            vec![11, 22]
        );
        assert_eq!(
            list_invalid_ids_in_range(95, 115, is_invalid_id_v1),
            vec![99]
        );
    }

    #[test]
    fn test_check_range_v2() {
        assert_eq!(list_invalid_ids_in_range(0, 1, is_invalid_id_v2), vec![]);
        assert_eq!(list_invalid_ids_in_range(0, 11, is_invalid_id_v2), vec![11]);
        assert_eq!(
            list_invalid_ids_in_range(11, 22, is_invalid_id_v2),
            vec![11, 22]
        );
        assert_eq!(
            list_invalid_ids_in_range(95, 115, is_invalid_id_v2),
            vec![99, 111]
        );
    }

    #[test]
    fn test_v1_on_ref() {
        assert_eq!(
            process_invalid_id("data/2/ref.txt", is_invalid_id_v1),
            1227775554
        );
    }

    #[test]
    fn test_v2_on_ref() {
        assert_eq!(
            process_invalid_id("data/2/ref.txt", is_invalid_id_v2),
            4174379265
        );
    }
}
