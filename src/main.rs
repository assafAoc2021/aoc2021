use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// mod d1;
mod d2;

fn main() {
    // d1::run();
    d2::run();
}

fn get_input(day: &str, test: &str) -> impl Iterator<Item=String> {
    let file_name = {
        let mut s = String::with_capacity(14);
        s.push('d');
        s.push_str(day);
        if !test.is_empty() {
            s.push_str("_test");
            s.push_str(test);
        }
        s.push_str(".txt");
        s
    };

    let p = {
        let mut t = PathBuf::new();
        t.push("inputs");
        t.push(file_name);
        t
    };
    let file = File::open(p).expect("file not found");
    BufReader::new(file).lines().map(|s| s.expect("error reading line"))
}
