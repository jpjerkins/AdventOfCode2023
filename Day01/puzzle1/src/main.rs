use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let calibration_values = read_lines("./realdata.txt")
        .unwrap()
        .filter_map(|rs| rs.ok())
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|chrs| chrs.iter().filter_map(|c| c.to_string().parse::<i64>().ok()).collect::<Vec<i64>>())
        .filter_map(|ints| match (ints.first(), ints.last()) {
            ( Some::<&i64>(i1), Some::<&i64>(i2)) => Some(format!("{i1}{i2}")),
            _ => None,
        })
        .filter_map(|s| s.parse::<i64>().ok());
    // calibration_values.for_each(move |i| print!("{}\n", i));
    // print!("Count of calibration_values: {}", calibration_values.count());
    let answer = calibration_values.fold(0i64, |acc, i| acc + i);
    print!("Answer: {}", answer);

    // print!("Answer: {}", answer);
    print!("\n\nOld code:\n\n");

    // // File hosts.txt must exist in the current path
    // if let Ok(lines) = read_lines("./testdata.txt") {
    //     // Consumes the iterator, returns an (Optional) String
    //     for line in lines {
    //         if let Ok(ip) = line {
    //             println!("{}", ip);
    //         }
    //     }
    // }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
