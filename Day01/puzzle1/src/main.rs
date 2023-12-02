use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let words: [&str; 9] = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    let word_lengths = words.map(|s| s.len());

    let numbers: [&str; 9] = [
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
    ];

    // let filename = "./testdata.txt";
    // let filename = "./realdata.txt";
    // let filename = "./testdata2.txt";
    let filename = "./realdata2.txt";
    let raw_lines = read_lines(filename).unwrap();

    let calibration_values = get_calibration_values(raw_lines, words, word_lengths, numbers);
    print_results(&calibration_values);
}

fn print_results(calibration_values_raw: &Vec::<i64>) {
    // calibration_values_raw.iter().for_each(move |i| print!("{}\n", i));
    print!("Count of calibration_values: {}\n", calibration_values_raw.iter().count());
    // let two_char_count = calibration_values_raw
    //     .iter()
    //     .map(|i| i.to_string().len())
    //     .filter(|count| &2 == count)
    //     .count();
    // print!("Count of calibration_values with two characters: {}\n", two_char_count);

    let answer = calibration_values_raw.iter().fold(0i64, |acc, i| acc + i);
    print!("Answer: {}\n", answer);
}

fn get_calibration_values(raw_lines: io::Lines<BufReader<File>>, words: [&str; 9], word_lengths: [usize; 9], numbers: [&str; 9]) -> Vec::<i64> {
    raw_lines
        .filter_map(|rs| rs.ok())
        .map(|s| sub_first_real_digit(s, words, word_lengths, numbers))
        .map(|s| sub_last_real_digit(s, words, word_lengths, numbers))
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|chrs| chrs.iter().filter_map(|c| c.to_string().parse::<i64>().ok()).collect::<Vec<i64>>())
        .filter_map(|ints| match (ints.first(), ints.last()) {
            ( Some::<&i64>(i1), Some::<&i64>(i2)) => Some(format!("{i1}{i2}")),
            _ => None,
        })
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

enum SubDigitResult {
    Found(String),
    NotFound(String),
}

fn sub_first_real_digit(mut s: String, words: [&str; 9], word_lengths: [usize; 9], numbers: [&str; 9]) -> String {
    // print!("Replacements in {}:\n", &s);
    for i in 0.. {
        match sub_leading_digit(&mut s, i, words, word_lengths, numbers) {
            SubDigitResult::Found(new_s) => {
                s = new_s;
                break;
            },
            SubDigitResult::NotFound(new_s) => {
                s = new_s;
            }
        }
        if i >= s.len() { break; }
    }
    // print!("  Returning {}\n", &s);
    s.to_string()
}

fn sub_leading_digit(s: &mut String, i: usize, words: [&str; 9], word_lengths: [usize; 9], numbers: [&str; 9]) -> SubDigitResult {
    let remaining_length = s.len() - i;
    for w in 0..9 {
        let len = min(word_lengths[w], remaining_length);
        if len < word_lengths[w] { continue; }
        if &(s[i..i+len]) == words[w] {
            s.replace_range(i..i+len, numbers[w]);
            // print!("  Replaced {} with {}\n", words[w], numbers[w]);
            return SubDigitResult::Found(s.to_string());
        }
    }
    for w in 0..9 {
        if &(s[i..i+1]) == numbers[w] {
            return SubDigitResult::Found(s.to_string());
        }
    }
    SubDigitResult::NotFound(s.to_string())
}

fn sub_last_real_digit(mut s: String, words: [&str; 9], word_lengths: [usize; 9], numbers: [&str; 9]) -> String {
    // print!("Replacements in {}:\n", &s);
    for i in 0.. {
        match sub_trailing_digit(&mut s, i, words, word_lengths, numbers) {
            SubDigitResult::Found(new_s) => {
                s = new_s;
                break;
            },
            SubDigitResult::NotFound(new_s) => {
                s = new_s;
            }
        }
        if i >= s.len() { break; }
    }
    // print!("  Returning {}\n", &s);
    s.to_string()
}

fn sub_trailing_digit(s: &mut String, i: usize, words: [&str; 9], word_lengths: [usize; 9], numbers: [&str; 9]) -> SubDigitResult {
    let current_length = s.len() - i;
    for w in 0..9 {
        let len = min(word_lengths[w], current_length);
        if len < word_lengths[w] { continue; }
        if &(s[s.len()-i-word_lengths[w]..s.len()-i]) == words[w] {
            s.replace_range(s.len()-i-word_lengths[w]..s.len()-i, numbers[w]);
            // print!("  Replaced {} with {}\n", words[w], numbers[w]);
            return SubDigitResult::Found(s.to_string());
        }
    }
    if current_length < 1 {
        SubDigitResult::NotFound(s.to_string());
    }
    for w in 0..9 {
        if &(s[s.len()-i-1..s.len()-i]) == numbers[w] {
            return SubDigitResult::Found(s.to_string());
        }
    }
    SubDigitResult::NotFound(s.to_string())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
