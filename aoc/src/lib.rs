use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub fn to_line_iterator(t: io::Lines<io::BufReader<File>>) -> impl Iterator<Item=String>
{
    return t.filter_map(|c| {
        if let Ok(string_content) = c {
            if string_content.is_empty()
            {
                None
            } else {
                Some(string_content)
            }
        } else {
            None
        }})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let content = read_lines("src/test-data/file.txt").filter_map(|c| {
            if let Ok(string_content) = c {
                Some(string_content)
            } else {
                None
            }
        });
        let line_vector: Vec<String> = content.collect();
        assert_eq!(line_vector, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_to_string_iterator(){
        let file_reader = read_lines("src/test-data/file.txt");
        let line_vector: Vec<String> = to_line_iterator(file_reader).collect();
        assert_eq!(line_vector, vec!["a", "b", "c", "d"]);
    }
}
