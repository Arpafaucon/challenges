#[derive(PartialEq, Eq)]
enum Cell {
    Roll,
    Empty,
}

const ROLL_SYM: char = '@';
const EMPTY_SYM: char = '.';

struct RollMap {
    map: Vec<Vec<Cell>>,
}

const NEIGHBOUR_THRESHOLD: u8 = 4;

impl RollMap {
    fn from_file(file: &str) -> RollMap {
        let file_content = std::fs::read_to_string(file).unwrap();
        let mut first_line_len = None;
        let mut map = Vec::<Vec<Cell>>::new();
        for l in file_content.lines() {
            let line_len = l.len();
            if first_line_len == None {
                first_line_len = Some(line_len);
            } else {
                assert_eq!(line_len, first_line_len.unwrap());
            }
            let mut map_line = Vec::<Cell>::new();
            map_line.reserve(line_len);
            for char in l.chars() {
                let cell = match char {
                    ROLL_SYM => Cell::Roll,
                    EMPTY_SYM => Cell::Empty,
                    _ => panic!("unsupported"),
                };
                map_line.push(cell);
            }
            map.push(map_line);
        }
        return RollMap { map };
    }

    #[allow(dead_code)]
    fn print(&self) {
        for line in self.map.iter() {
            for block in line {
                let char = match block {
                    Cell::Roll => ROLL_SYM,
                    Cell::Empty => EMPTY_SYM,
                };
                print!("{}", char);
            }
            println!();
        }
    }

    fn generate_valid_neighbours_coordinates(
        row: usize,
        col: usize,
        num_rows: usize,
        num_cols: usize,
    ) -> Vec<(usize, usize)> {
        const OFFSETS: [(i8, i8); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let out: Vec<(usize, usize)> = OFFSETS
            .iter()
            .map(|(r, c)| (row as i128 + *r as i128, col as i128 + *c as i128))
            .filter(|(r, c)| {
                *r >= 0 && (*r as usize) < num_rows && *c >= 0 && (*c as usize) < num_cols
            })
            .map(|(r, c)| (r.try_into().unwrap(), c.try_into().unwrap()))
            .collect();
        return out;
    }

    fn list_cells_with_max_x_neighbours(&self, neighbour_threshold: usize) -> Vec<(usize, usize)> {
        let num_rows = self.map.len();
        let num_cols = self.map[0].len();
        let mut accessible_rolls = vec![];
        for cell_row in 0..num_rows {
            for cell_col in 0..num_cols {
                if let Cell::Empty = self.map[cell_row][cell_col] {
                    // skip cells without rolls
                    continue;
                }
                let neighbour_stacks = RollMap::generate_valid_neighbours_coordinates(
                    cell_row, cell_col, num_rows, num_cols,
                )
                .iter()
                .filter(|(row, col)| self.map[*row][*col] == Cell::Roll)
                .count();
                if neighbour_stacks < neighbour_threshold {
                    accessible_rolls.push((cell_row, cell_col));
                }
            }
        }
        return accessible_rolls;
    }

    fn remove_rolls(&mut self, cells: &[(usize, usize)]) {
        for (cell_row, cell_col) in cells {
            assert!(self.map[*cell_row][*cell_col] == Cell::Roll);
            self.map[*cell_row][*cell_col] = Cell::Empty;
        }
    }
}

fn iterated_roll_removal(path: &str) -> Vec<Vec<(usize, usize)>> {
    let mut map = RollMap::from_file(path);
    let mut removal_steps: Vec<Vec<(usize, usize)>> = vec![];
    loop {
        let accessible_rolls = map.list_cells_with_max_x_neighbours(NEIGHBOUR_THRESHOLD as usize);
        let num_accessible = accessible_rolls.len();
        if num_accessible == 0 {
            break;
        }
        map.remove_rolls(&accessible_rolls);

        removal_steps.push(accessible_rolls);
    }
    return removal_steps;
}

fn main() {
    let removal_steps = iterated_roll_removal("data/4/input.txt");
    let first_removal_step_count = removal_steps[0].len();
    let total_removal_count: usize = removal_steps
        .iter()
        .map(|step_rolls| step_rolls.len())
        .sum();
    println!("First step removed {first_removal_step_count}");
    println!("All steps: removed {total_removal_count}");
}

#[cfg(test)]
mod tests {
    use crate::{NEIGHBOUR_THRESHOLD, RollMap, iterated_roll_removal};

    #[test]
    fn test_neighbour() {
        assert_eq!(
            RollMap::generate_valid_neighbours_coordinates(0, 0, 1, 1),
            vec![]
        );
        assert_eq!(
            RollMap::generate_valid_neighbours_coordinates(0, 0, 1, 2),
            vec![(0, 1)]
        );
        assert_eq!(
            RollMap::generate_valid_neighbours_coordinates(0, 1, 2, 2),
            vec![(0, 0), (1, 0), (1, 1)]
        );
        assert_eq!(
            RollMap::generate_valid_neighbours_coordinates(3, 4, 8, 8),
            vec![
                (2, 3),
                (3, 3),
                (4, 3),
                (2, 4),
                (4, 4),
                (2, 5),
                (3, 5),
                (4, 5)
            ]
        );
    }

    #[test]
    fn test_ref_v1() {
        let map = RollMap::from_file("data/4/ref.txt");
        let out = map.list_cells_with_max_x_neighbours(NEIGHBOUR_THRESHOLD as usize);
        assert_eq!(out.len(), 13);
    }
    #[test]
    fn test_ref_v2() {
        let removal_steps = iterated_roll_removal("data/4/ref.txt");
        let total_removed: usize = removal_steps.iter().map(|step| step.len()).sum();
        assert_eq!(total_removed, 43);
    }

    #[test]
    fn test_print() {
        let map = RollMap::from_file("data/4/ref.txt");
        // check no panic
        map.print();
    }
}
