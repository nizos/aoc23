use anyhow::Result;
use regex::Regex;
use std::io;
use util::Input;

const INPUT_FILE_PATH: &str = "./day01/input";

fn main() -> Result<()> {
    let input = Input::load(INPUT_FILE_PATH)?;

    println!("Part 1:");
    println!("{}", part1(&input)?); // 53080

    println!("Part 2:");
    println!("{}", part2(&input)?); // 53255
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
        (Some(f), Some(l)) if f != l => format!("{}{}", f, l),
        (Some(c), _) => format!("{}{}", c, c),
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

/// Converts spelled-out numbers (zero to nine) in a string to their digit representations.
///
/// # Arguments
///
/// * `input` - A string slice reference containing the text to be processed.
///
/// # Returns
///
/// A `String` where spelled-out numbers have been replaced with their digit representations.
/// Non-matching strings are returned as is.
fn convert_to_digits(input: &str) -> String {
    const NUMBER_MAP: &[(&str, &str)] = &[
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

    NUMBER_MAP
        .iter()
        .find(|&&(word, _)| word == input)
        .map_or_else(|| input.to_string(), |&(_, digit)| digit.to_string())
}

/// Replaces spelled-out numbers (zero to nine) in a string with their digit representations.
///
/// # Arguments
///
/// * `input` - A string slice reference containing the text to be processed.
///
/// # Returns
///
/// A `String` where each spelled-out number from 'zero' to 'nine' is replaced with its
/// corresponding digit. If no replacements are made, the original string is returned unchanged.
fn replace_spelled_out(input: &str) -> String {
    let re = Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        convert_to_digits(caps.get(0).unwrap().as_str())
    })
    .to_string()
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
        convert_to_digits, filter_digits, filter_digits_in_strings, filter_first_and_last_strings,
        get_calibration_sum, part1, part2, replace_spelled_out, replace_spelled_out_strings,
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
    pub fn test_convert_to_digits() {
        // Given a single spelled out number as a string
        let input = "eight";

        // When convert_to_digits is called
        let actual = convert_to_digits(&input);

        // Then it should return the spelled out number in digits
        assert_eq!(
            actual, "8",
            "convert_to_digits should return \"8\" for an input of \"eight\""
        )
    }

    #[test]
    pub fn test_replace_spelled_out() {
        // Given a string of spelled out numbers and numbers in their digital representation
        let input = "abcone2threexyz";

        // When replace_spelled_out is called
        let actual = replace_spelled_out(input);

        // Then it should replace all the spelled out numbers with their digital representations
        assert_eq!(
            actual, "abc123xyz",
            "replace_spelled_out should return \
                       \"abc123xyz\" for an input string of \"abcone2threexyz\""
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
            "8wo3",
            "abc123xyz",
            "x2ne34",
            "49872",
            "z1ight234",
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
