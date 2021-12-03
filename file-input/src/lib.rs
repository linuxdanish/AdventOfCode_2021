use std::fs;
use std::io;
use std::io::BufRead;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// Function takes filename and gets the contents as a vector of strings.
pub fn file_contents_as_vec(filename: String) -> io::Result<Vec<String>> {
    let file = fs::File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    Ok(reader.lines().filter_map(io::Result::ok).collect())
}