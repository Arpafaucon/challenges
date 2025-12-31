use std::path::Path;

struct Inventory {
    fresh_ranges: Vec<(usize, usize)>,
    available: Vec<usize>,
}

impl Inventory {
    fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let content = std::fs::read_to_string(path).unwrap();
        let mut content_lines = content.lines().into_iter();

        let mut fresh_ranges: Vec<(usize, usize)> = vec![];
        let mut available: Vec<usize> = vec![];

        // consume fresh ranges
        for line in content_lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (range_start_str, range_end_str) = line.split_once("-").unwrap();
            let range_start: usize = range_start_str.parse().unwrap();
            let range_end: usize = range_end_str.parse().unwrap();
            fresh_ranges.push((range_start, range_end));
        }
        // consume available
        for line in content_lines.by_ref() {
            available.push(line.parse().unwrap());
        }


        // fresh_ranges.sort_unstable();
        return Inventory {
            fresh_ranges,
            available,
        };
    }

    fn build_cleaned(self: Self) -> Self
    {
        let Inventory {fresh_ranges, available} = self;
        let optimized_ranges = Self::optimize_ranges(fresh_ranges);
        Inventory {fresh_ranges:optimized_ranges, available}
    }

    fn optimize_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)>
    {
        ranges.sort_unstable_by_key(|(s,e)| (*s, -(*e as i128)));

        let mut cleaned_ranges : Vec<(usize, usize)>= vec![];
        cleaned_ranges.reserve_exact(ranges.len());

        // process ranges to make them non-overlapping
        // assuming they've been sorted by `start`
        for (elt_start , elt_end) in ranges {
            // if there is already an element in the array (most of most of the time)
            // check if we can merge with it
            if let Some((previous_start, previous_end)) = cleaned_ranges.pop()
            {
                let can_merge = elt_start <= previous_end;
                if can_merge
                {
                    cleaned_ranges.push( (previous_start, std::cmp::max(elt_end, previous_end)));
                } else {
                    cleaned_ranges.push( (previous_start, previous_end));
                    cleaned_ranges.push( (elt_start, elt_end));
                }
            } else {
                cleaned_ranges.push(( elt_start, elt_end));
            }
        }
        cleaned_ranges
    }

    fn check_fresh(self: &Self, val: usize) -> bool {
        let search_res = self
            .fresh_ranges
            .iter()
            .filter(|(range_start, range_end)| *range_start <= val && val <= *range_end)
            .take(1)
            .next();
        match search_res {
            Some(_) => true,
            None => false,
        }
    }

    fn list_fresh_available(self: &Self) -> Vec<usize> {
        self.available
            .iter()
            .cloned()
            .filter(|&elt| self.check_fresh(elt))
            .collect::<Vec<usize>>()
            .to_owned()
    }

    // only works for optimized ranges
    fn total_fresh_count(self: &Self) -> usize {
        self.fresh_ranges.iter().map(|(s,e)| e-s+1).sum()
    }
}

fn main() {
    let ii = Inventory::from_file("data/5/input.txt").build_cleaned();
    // println!("{:#?}", ii.available);
    // println!("{:#?}", ii.fresh_ranges);
    let fresh_available = ii.list_fresh_available();
    println!("{:#?}", fresh_available.len());
    let all_fresh_count = ii.total_fresh_count();
    println!("Total fresh count {}", all_fresh_count);

}

#[cfg(test)]
mod tests {
    use crate::Inventory;

    #[test]
    fn test_ref_v1() {
        let inventory = Inventory::from_file("data/5/ref.txt");
        assert_eq!(inventory.available, vec![1, 5, 8, 11, 17, 32]);
        assert_eq!(
            inventory.fresh_ranges,
            vec![(3, 5), (10, 14), (16, 20), (12, 18)]
        );
        assert_eq!(inventory.list_fresh_available(), vec![5, 11, 17]);
    }

    #[test]
    fn test_ref_v2() {
        let inventory = Inventory::from_file("data/5/ref.txt").build_cleaned();
        assert_eq!(inventory.fresh_ranges, vec![(3,5), (10, 20)]);
        assert_eq!(inventory.total_fresh_count(), 14);
    }
}
