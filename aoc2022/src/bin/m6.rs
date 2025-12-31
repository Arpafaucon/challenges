use aoc::read_lines;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, PartialEq, Eq, Debug)]
struct ScanOutput {
    packet_offset: i32,
    packet_token: String,
    message_offset: i32,
    message_token: String,
}

struct Scanner {
    queue: VecDeque<char>,
    capacity: usize,
}

impl Scanner {
    fn detect(&self) -> bool {
        let charset: HashSet<char> = self.queue.iter().cloned().collect();
        charset.len() == self.capacity
    }

    fn append(&mut self, char: char) {
        if self.queue.len() == self.capacity {
            self.queue.pop_front();
        }
        self.queue.push_back(char);
    }

    fn new(cap: usize) -> Scanner {
        Scanner {
            queue: VecDeque::new(),
            capacity: cap,
        }
    }

    fn token(&self) -> String {
        self.queue.iter().cloned().collect()
    }
}

fn scan_string(input: &str) -> ScanOutput {
    let mut scanning_packet = true;
    let mut packet_scanner = Scanner::new(4);
    let mut packet_offset = 0;
    let mut scanning_message = true;
    let mut message_scanner = Scanner::new(14);
    let mut message_offset = 0;

    let mut scanned_chars = 0;
    for char in input.chars() {
        scanned_chars += 1;
        println!("[{}] Processing {}", scanned_chars, char);

        if scanning_packet {
            packet_scanner.append(char);
            if packet_scanner.detect() {
                scanning_packet = false;
                packet_offset = scanned_chars
            }
        }
        if scanning_message {
            message_scanner.append(char);
            if message_scanner.detect() {
                scanning_message = false;
                message_offset = scanned_chars
            }
        }

        if !scanning_message && !scanning_packet {
            break;
        }
    }
    return ScanOutput {
        packet_offset: packet_offset,
        packet_token: packet_scanner.token(),
        message_offset: message_offset,
        message_token: message_scanner.token(),
    };
}

fn scan_file(filename: &str) -> ScanOutput {
    let line = read_lines(filename).map(|l| l.unwrap()).next().unwrap();
    scan_string(&line)
}

fn main() {
    let filename = "./data/6a.txt";
    let scan_out = scan_file(filename);
    println!("Scan result {:?}", scan_out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_sets() {
        {
            let out = scan_file("./data/6t0.txt");
            assert_eq!(out.packet_offset, 7);
            assert_eq!(out.packet_token, "jpqm");
        }
        {
            let out = scan_file("./data/6t1.txt");
            assert_eq!(out.packet_offset, 5);
            assert_eq!(out.packet_token, "vwbj");
        }
        {
            let out = scan_file("./data/6t2.txt");
            assert_eq!(out.packet_offset, 6);
            assert_eq!(out.packet_token, "pdvj");
        }
    }
}
