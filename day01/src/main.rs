use anyhow::Result;
use log::error;
use std::collections::HashSet;
use std::io;
use util::Input;

const INPUT_FILE_PATH: &str = "./day01/input";

/// Static list of spelled-out numbers.
static SPELLED_OUT_NUMBERS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Static mapping of spelled-out numbers to their digit representations.
static NUMBER_MAP: &[(&str, &str)] = &[
    ("zero", "0"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn main() -> Result<()> {
    let input = Input::load(INPUT_FILE_PATH)?;

    println!("Part 1:");
    println!("{}", part1(&input)?); // 53080

    println!("Part 2:");
    println!("{}", part2(&input)?); // 53268
    Ok(())
}

fn part1(input: &Input) -> Result<i32> {
    Ok(get_calibration_sum(input)?)
}

fn part2(input: &Input) -> Result<i32> {
    let no_spelled = replace_spelled_out_strings(input.lines());
    let digits_only = filter_digits_in_strings(&no_spelled);
    let first_and_last = filter_first_and_last_strings(&digits_only);
    Ok(sum_digits_in_strings(&first_and_last))
}

/// Extracts and returns all digits from a given string.
///
/// # Arguments
///
/// * `input` - A string slice that may contain any characters.
///
/// # Returns
///
/// Returns a new `String` containing only the digits from the input string.
fn filter_digits(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Returns a vector of strings that only contain digits.
///
/// # Arguments
///
/// * `input` - An array of strings, each of which may contain any characters.
///
/// # Returns
///
/// Returns a new `Vec` containing only the digits from each string in the input array.
fn filter_digits_in_strings<T: AsRef<str>>(input: &[T]) -> Vec<String> {
    input.iter().map(|s| filter_digits(s.as_ref())).collect()
}

/// Returns the first and last characters in a string of characters.
///
/// The function returns a string consisting of the first and last characters in the input string.
/// If the input string consists of a single character, then it is used as both the first and last.
/// If the input string is empty, then an empty string is returned.
///
/// # Arguments
///
/// * `input` - A string slice that may contain any characters.
///
/// # Returns
///
/// Returns a new `String` containing only the first and last characters.
fn filter_first_and_last(input: &str) -> String {
    let first: Option<char> = input.chars().next();
    let last: Option<char> = input.chars().last();

    match (first, last) {
        (Some(f), Some(l)) => format!("{}{}", f, l),
        _ => String::new(),
    }
}

/// Returns a vector of strings that contain ony the first and last characters.
///
/// # Arguments
///
/// * `input` - An array of strings, each of which may contain any characters.
///
/// # Returns
///
/// Returns a new `Vec` of strings containing only the first and last characters
/// from the original strings.
fn filter_first_and_last_strings<T: AsRef<str>>(input: &[T]) -> Vec<String> {
    input
        .iter()
        .map(|s| filter_first_and_last(s.as_ref()))
        .collect()
}

/// Returns the sum of numerical values in a collection of strings.
///
/// # Arguments
///
/// * `input` - An array of strings, each of which may contain a numerical value.
///
/// # Returns
///
/// Returns a new `i32` whose value is the sum of all digits.
fn sum_digits_in_strings<T: AsRef<str>>(input: &[T]) -> i32 {
    input
        .iter()
        .filter_map(|s| s.as_ref().parse::<i32>().ok())
        .sum()
}

/// Returns the calibration sum of a new-line-separated list of strings at a specified file path.
///
/// The function reads the text contents of the file and then processes the content as follows:
/// * Filters out all non-numerical characters in each line.
/// * Filters out all remaining characters except for the first and last in each line.
/// * Calculates and returns the sum of the resulting numerical values.
///
/// # Arguments
///
/// * `input` - A file path to read the data from.
///
/// # Returns
///
/// * Returns the sum of the resulting numerical values according to the described algorithm.
fn get_calibration_sum(input: &Input) -> Result<i32, io::Error> {
    let digits_only = filter_digits_in_strings(input.lines());
    let first_and_last = filter_first_and_last_strings(&digits_only);
    Ok(sum_digits_in_strings(&first_and_last))
}

/// Returns a digit representation for a spelled-out number (zero to nine).
///
/// # Arguments
///
/// * `spelled_out` - A spelled out number from zero to nine.
///
/// # Returns
///
/// An `Option` containing the digit as a string slice. Returns `None` if no match is found.
fn get_digit_for_spelled_out_number(spelled_out: &str) -> Option<&'static str> {
    NUMBER_MAP
        .iter()
        .find(|&&(word, _)| word == spelled_out)
        .map(|&(_, digit)| digit)
}

/// Finds a spelled-out number in a string starting from a specific index.
///
/// # Arguments
///
/// * `input` - The input string to search.
/// * `index` - The index to start searching from.
///
/// # Returns
///
/// An `Option` containing the spelled-out number as a string slice, starting from the given index.
fn get_spelled_out_number(input: &str, index: usize) -> Option<&'static str> {
    SPELLED_OUT_NUMBERS
        .iter()
        .find(|&&word| input[index..].starts_with(word))
        .copied()
}

/// Identifies the start indexes of all spelled-out numbers in a string.
///
/// # Arguments
///
/// * `input` - The input string to search.
///
/// # Returns
///
/// A `Vec<usize>` containing the start indexes of spelled-out number found.
fn get_spelled_out_number_indexes(input: &str) -> Vec<usize> {
    let mut indexes = vec![];
    for (index, _) in input.char_indices() {
        if get_spelled_out_number(input, index).is_some() {
            indexes.push(index)
        }
    }
    indexes
}

/// Replaces spelled-out numbers in a string with their digit representations.
///
/// # Arguments
///
/// * `input` - The input string containing spelled-out numbers.
///
/// # Returns
///
/// A `String` where spelled-out numbers are replaced with digits.
/// Unmatched parts of the string are unchanged.
fn replace_spelled_out(input: &str) -> String {
    let mut result = String::new();
    let mut total_chars_to_skip = 0;
    let number_indexes: HashSet<usize> =
        get_spelled_out_number_indexes(input).into_iter().collect();

    for (index, ch) in input.char_indices() {
        if number_indexes.contains(&index) {
            if let Some(spelled_out) = get_spelled_out_number(input, index) {
                if let Some(digit) = get_digit_for_spelled_out_number(spelled_out) {
                    result.push_str(digit);
                    total_chars_to_skip = spelled_out.len() - 1;
                    continue;
                } else {
                    error!(
                        "No digit representation found for spelled-out number {}",
                        spelled_out
                    )
                }
            }
        } else if total_chars_to_skip == 0 {
            result.push(ch);
        } else {
            total_chars_to_skip -= 1;
        }
    }
    result
}

/// Replaces spelled-out numbers (zero to nine) in each string of an input collection.
/// with their digit representations.
///
/// # Arguments
///
/// * `input` - An iterable collection of string references.
///
/// # Returns
///
/// A `Vec<String>` where each element is a string from the input collection with
/// spelled-out numbers replaced by digits.
fn replace_spelled_out_strings<T: AsRef<str>>(input: &[T]) -> Vec<String> {
    input
        .iter()
        .map(|s| replace_spelled_out(s.as_ref()))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{
        filter_digits, filter_digits_in_strings, filter_first_and_last_strings,
        get_calibration_sum, get_digit_for_spelled_out_number, get_spelled_out_number_indexes,
        part1, part2, replace_spelled_out, replace_spelled_out_strings,
    };
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        // Given an input of strings
        let input = Input::from_lines(&["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]);

        // Then it should return their calibration sum
        assert_eq!(part1(&input).unwrap(), 142);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        // Given an input of strings containing spelled out numbers
        let input = Input::from_lines(&[
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]);

        // Then it should return their calibration sum
        assert_eq!(part2(&input).unwrap(), 281);
        Ok(())
    }

    #[test]
    pub fn test_filter_digits() {
        // Given a string input with letters and digits
        let input: &str = "1abc2";

        // When filter_digits is called
        let actual: String = filter_digits(input);

        // Then it should return only digits
        assert_eq!(
            actual, "12",
            "filter_digits should return '12' for an input of '1abc2'"
        )
    }

    #[test]
    pub fn test_filter_digits_in_strings() {
        // Given an array of strings containing letters and digits
        let input: Vec<String> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let expected: Vec<&str> = vec!["12", "38", "12345", "7"];

        // When filter_digits_in_strings is called
        let actual: Vec<String> = filter_digits_in_strings(&input);

        // Then it should return an array of strings containing only the digits
        assert_eq!(
            actual, expected,
            "filter_digits_in_strings should return a vector of strings \
                   containing only digits"
        )
    }

    mod test_filter_first_and_last {
        use crate::filter_first_and_last;

        #[test]
        pub fn several_characters() {
            // Then it should return a string containing the first and last characters
            // when the input string contains several characters
            assert_eq!(
                filter_first_and_last("12345"),
                "15",
                "filter_first_and_last should return '15' for an input of '12345'"
            )
        }

        #[test]
        pub fn single_character() {
            // Then it should return a string containing the character twice
            // when the input string consists of a single character
            assert_eq!(
                filter_first_and_last("1"),
                "11",
                "filter_first_and_last should return '11' for an input of '1'"
            )
        }

        #[test]
        pub fn empty_string() {
            // Then it should return an empty string when the input is an empty string
            assert_eq!(
                filter_first_and_last(""),
                "",
                "filter_first_and_last should return an empty string \
                       when the input is an empty string"
            )
        }
    }

    #[test]
    pub fn test_filter_first_last_strings() {
        // Given a vector of strings that consists of numerical values
        let input: Vec<String> = vec!["1542", "308", "115", "7"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let expected: Vec<&str> = vec!["12", "38", "15", "77"];

        // When filter_first_last_strings is called
        let actual = filter_first_and_last_strings(&input);

        // Then it should return an array of strings,
        // each of which consists of the first and last digits in the numerical values
        assert_eq!(
            actual, expected,
            "filter_first_and_last_strings should return \
                   [\"12\", \"38\", \"15\", \"77\"] when  the input is \
                   [\"1542\", \"308\", \"115\", \"7\"]"
        )
    }

    mod test_sum_digits_in_strings {
        use crate::sum_digits_in_strings;

        #[test]
        pub fn positive_numbers() {
            // Given a vector of strings that consist of positive numerical values
            let input: Vec<String> = vec!["12", "38", "15", "77"]
                .into_iter()
                .map(|s| s.to_string())
                .collect();

            // When sum_digits_in_strings is called
            let actual: i32 = sum_digits_in_strings(&input);

            // Then it should return the sum of the digits in all the strings
            assert_eq!(
                actual, 142,
                "sum_digits_in_strings should return 142 \
                       for an input of [\"12\", \"38\", \"15\", \"77\"]"
            )
        }

        #[test]
        pub fn negative_numbers() {
            // Given a vector of strings that consist of negative numerical values
            let input: Vec<String> = vec!["-12", "-38", "-15", "-77"]
                .into_iter()
                .map(|s| s.to_string())
                .collect();

            // When sum_digits_in_strings is called
            let actual: i32 = sum_digits_in_strings(&input);

            // Then it should return the sum of the digits in all the strings
            assert_eq!(
                actual, -142,
                "sum_digits_in_strings should return -142 \
                       for an input of [\"-12\", \"-38\", \"-15\", \"-77\"]"
            )
        }

        #[test]
        pub fn mixed_numbers() {
            // Given a vector of strings that consist of positive and negative numerical values
            let input: Vec<String> = vec!["12", "-38", "15", "77"]
                .into_iter()
                .map(|s| s.to_string())
                .collect();

            // When sum_digits_in_strings is called
            let actual: i32 = sum_digits_in_strings(&input);

            // Then it should return the sum of the digits in all the strings
            assert_eq!(
                actual, 66,
                "sum_digits_in_strings should return 66 \
                       for an input of [\"12\", \"-38\", \"15\", \"77\"]"
            )
        }
    }

    #[test]
    pub fn test_get_calibration_sum() -> Result<()> {
        // Given an input of lines that consist of alphabetical and numerical characters
        let input = Input::from_lines(&["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]);

        // When get_calibration_sum is called
        let actual = get_calibration_sum(&input)?;

        // Then it should return the sum of each line's numerical value
        // which consists of the first and last digit of said line
        assert_eq!(
            actual, 142,
            "get_calibration_sum should return 142 for the provided input"
        );
        Ok(())
    }

    #[test]
    pub fn test_get_digit_for_spelled_out_number() {
        // Given a single spelled out number as a string
        let input = "eight";

        // When convert_to_digits is called
        let actual = get_digit_for_spelled_out_number(&input).unwrap();

        // Then it should return the spelled out number in digits
        assert_eq!(
            actual, "8",
            "convert_to_digits should return \"8\" for an input of \"eight\""
        )
    }

    mod test_get_spelled_out_number {
        use crate::get_spelled_out_number;

        #[test]
        pub fn test_spelled_out_number_with_index_at_start() {
            // Give a string that consists of a spelled-out number and an index at the start
            let input = "eight";
            let index = 0;

            // When get_spelled_out_number is called
            let actual = get_spelled_out_number(input, index).unwrap();

            // Then it should return the spelled-out number
            assert_eq!(
                actual, input,
                "get_spelled_out_number should return \
                \"eight\" for an input of \"eight\" and an index of 0"
            )
        }

        #[test]
        pub fn test_spelled_out_number_with_index_after_start() {
            // Give a string that consists of a spelled-out number and an index past the start
            let input = "eight";
            let index = 1;

            // When get_spelled_out_number is called
            let actual = get_spelled_out_number(input, index);

            // Then it should return None
            assert_eq!(
                actual, None,
                "get_spelled_out_number should return \
                None for an input of \"eight\" and an index of 1"
            )
        }

        #[test]
        pub fn test_spelled_out_number_with_index_before_start() {
            // Give a string that contains a spelled-out number and an index before its start
            let input = "abceight";
            let index = 1;

            // When get_spelled_out_number is called
            let actual = get_spelled_out_number(input, index);

            // Then it should return None
            assert_eq!(
                actual, None,
                "get_spelled_out_number should return \
                None for an input of \"abceight\" and an index of 1"
            )
        }
    }

    #[test]
    pub fn test_get_spelled_out_number_indexes() {
        // Given a string containing overlapping spelled-out numbers
        let input = "eightwo";

        // When get_spelled_out_number_indexes is called
        let actual = get_spelled_out_number_indexes(input);

        // Then it should return a vector containing the spelled-out number starting indexes
        assert_eq!(
            actual,
            vec![0, 4],
            "get_spelled_out_number_indexes should return a vector \
            containing 0 and 4 for an input string of \"eightwo\""
        )
    }

    #[test]
    pub fn test_replace_spelled_out() {
        // Given a string of spelled out numbers and numbers in their digital representation
        let input = "eightjzqzhrllg1oneightfck";

        // When replace_spelled_out is called
        let actual = replace_spelled_out(input);

        // Then it should replace all the spelled out numbers with their digital representations
        assert_eq!(
            actual, "8jzqzhrllg118fck",
            "replace_spelled_out should return \
                       \"8jzqzhrllg118fck\" for an input string of \"eightjzqzhrllg1oneightfck\""
        )
    }

    #[test]
    pub fn test_replace_spelled_out_strings() {
        // Given a vector of strings that contains spelled out and digital numerical values
        let input: Vec<String> = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        let expected: Vec<&str> = vec![
            "219",
            "823",
            "abc123xyz",
            "x2134",
            "49872",
            "z18234",
            "7pqrst6teen",
        ];

        // When replace_spelled_out_strings is called
        let actual = replace_spelled_out_strings(&input);

        // Then it should replace all the spelled out numbers with their digital representations
        assert_eq!(
            actual, expected,
            "replace_spelled_out_strings should return a vector with all \
                   the spelled out numbers converted to their digital representation."
        )
    }
}
