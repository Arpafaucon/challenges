
use std::str::FromStr;

use aoc::{read_lines, to_line_iterator};

// Implement a struct to extract the range specifications
#[derive(Debug, PartialEq, Eq)]
struct SectionRange {
    start: i32,
    end: i32, // included
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSectionRangeError;

impl SectionRange {
    fn contains(self: &Self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }
    fn overlaps(self: &Self, other: &Self) -> bool {
        !(self.start > other.end || other.start > self.end)
    }
}

// We push it a little further by using the native parsing pattern
impl FromStr for SectionRange {
    type Err = ParseSectionRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once('-').ok_or(ParseSectionRangeError)?;

        let x_fromstr = x.parse::<i32>().map_err(|_| ParseSectionRangeError)?;
        let y_fromstr = y.parse::<i32>().map_err(|_| ParseSectionRangeError)?;

        Ok(SectionRange {
            start: x_fromstr,
            end: y_fromstr,
        })
    }
}

fn main() {
    let mut contained_assignments = 0i32;
    let mut overlapping_assignments = 0i32;
    for line in to_line_iterator(read_lines("./data/4a.txt")) {
        let (range1, range2) = line.split_once(',').unwrap();
        let section1 = range1.parse::<SectionRange>().unwrap();
        let section2 = range2.parse::<SectionRange>().unwrap();

        if section1.contains(&section2) || section2.contains(&section1) {
            contained_assignments += 1;
        }
        if section1.overlaps(&section2) {
            overlapping_assignments += 1;
        }
    }
    println!("A: {}", contained_assignments);
    println!("B: {}", overlapping_assignments);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_section_parsing() {
        let expected = Ok(SectionRange { start: 1, end: 2 });
        // Explicit call
        assert_eq!(SectionRange::from_str("1-2"), expected);
        // Implicit calls, through parse
        assert_eq!("1-2".parse(), expected);
        assert_eq!("1-2".parse::<SectionRange>(), expected);
        // Invalid input string
        assert!(SectionRange::from_str("(1 2)").is_err());
    }

    #[test]
    fn test_ops() {
        let a = SectionRange { start: 2, end: 3 };
        let b = SectionRange { start: 1, end: 3 };
        let c = SectionRange { start: 4, end: 5 };

        assert!(b.contains(&a));
        assert!(!a.contains(&b));
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));

        assert!(!a.contains(&c));
        assert!(!a.overlaps(&c));
    }
}
