use chrono::{DateTime, Utc};
use colored::Colorize;
use std::f64;
use std::fs::{self, File};
use std::io::{self, BufRead, Read};
// custom
use crate::util::config::get_db_file_path;
use crate::util::display::display_err_reading_file;
use crate::util::enums::{IntFloat, StatusColorType};

// ******************* INTERNAL FUNCTIONS *************
/**
 * Function to check if the float point number has just one decimal place
 * @params {f64} input float number
 * @returns {bool} is float point number or not
 */

// FIXME: Remove use of regex, do without regex
fn is_float_with_single_digit_decimal_places(num: f64) -> bool {
    // let regex = Regex::new(r"^-?\d+\.\d$").unwrap();
    // regex.is_match(num.to_string().as_str())
    let result = num - num.floor();
    let digit = (result * 10.0) as i32;
    digit > 0 && digit < 9
}

/**
 * TESTING: Using BufReader for reading large files
 * FOR TESTING PURPOSE
 */
#[allow(dead_code)]
fn read_file_with_lines() -> Result<Vec<String>, String> {
    let source_json = "jotdown-db.json";
    let file: File = File::open(source_json).unwrap();
    // another approach - using io::BufReader for large files
    let reader = io::BufReader::new(file);
    let result: Vec<String> = reader
        .lines()
        .into_iter()
        .map(|item| item.unwrap())
        .collect();
    Ok(result)
}

/**
 * TESTING: Using Vectors fith file reads
 * FOR TESTING PURPOSE
 */
#[allow(dead_code)]
fn return_lines_only_if_contains_string(test: &str) -> Result<Vec<String>, String> {
    let source_json = "jotdown-db.json";
    let file: String = fs::read_to_string(source_json).unwrap();
    let mut results: Vec<String> = Vec::new();
    for line in file.lines() {
        if line.contains(test) {
            results.push(line.to_string());
        }
    }
    Ok(results)
}

/**
 * TESTING: Trying out colorized util library
 * FOR TESTING PURPOSE ONLY
 */
#[allow(dead_code)]
fn colored_crate_features() {
    "this is blue".blue();
    "this is red".red();
    "this is red on blue".red().on_blue();
    "this is also red on blue".on_blue().red();
    "you can use truecolor values too!".truecolor(0, 255, 136);
    "background truecolor also works :)".on_truecolor(135, 28, 167);
    "you can also make bold comments".bold();
    println!(
        "{} {} {}",
        "or use".cyan(),
        "any".italic().yellow(),
        "string type".cyan()
    );
    "or change advice. This is red".yellow().blue().red();
    "or clear things up. This is default color and style"
        .red()
        .bold()
        .clear();
    "purple and magenta are the same".purple().magenta();
    "bright colors are also allowed"
        .bright_blue()
        .on_bright_white();
    "you can specify color by string"
        .color("blue")
        .on_color("red");
    "and so are normal and clear".normal().clear();
    String::from("this also works!").green().bold();
    let result: String = format!(
        "{}",
        "format works as expected. This will be padded".on_truecolor(135, 28, 167)
    );
    let output: String = format!(
        "{}",
        "and this will be green but truncated to 3 chars"
            .red()
            .bold()
            .clear()
    );
    println!("{}", result);
    println!("{}", output);
}

// ******************* PUBLIC FUNCTIONS *************

/**
* Helper methods ***********************************
* ...contains lessons from Linkedin - Rust - File Manipulation
*/

/**
* helper fn to read file contents to string
* @returns {String} file contents as string
*/
pub fn read_file_from_path() -> String {
    let file_path: String = get_db_file_path();
    let mut file: File = File::open(file_path).unwrap();
    let mut json_string: String = String::new();
    file.read_to_string(&mut json_string)
        .expect(&*display_err_reading_file());
    json_string
}

/**
* helper fn to get enum type based on string / phrase
* @param {String} word / phrase
* @returns {StatusColorType} enum
*/
pub fn get_status_type(phrase: String) -> StatusColorType {
    return match phrase.as_str() {
        "@important" => StatusColorType::Important,
        "@today" => StatusColorType::Today,
        "@week" => StatusColorType::Week,
        "@month" => StatusColorType::Month,
        "@overdue" => StatusColorType::Overdue,
        _ => StatusColorType::Info,
    };
}

// Parser to find "@tag" specifically at the end of the input
fn parse_tag_at_end(input: &str) -> &str {
    input.split_whitespace().rev() // Split the string into words and reverse iterate
        .find(|&word| word.starts_with('@')) // Find the first word (in reverse) that starts with '@'
        .unwrap_or("") // If not found, return the original input
}

/**
* helper fn to retrieve tags from a given sentence
* @param {String} text
* @returns tuple {(String, String)} - remaining text, tag
* Using 2 pointers
*/
pub fn get_tag_annotation_from_string(text: &String) -> (String, String, String) {
    let mut remaining: &str = "";
    let mut preceded: &str = "";
    let tag_word = parse_tag_at_end(text);
    if tag_word == "" || tag_word == "@" {
        (text.to_owned(), "".to_owned(), "".to_owned())
    } else {
        if let Some(idx) = text.find(tag_word) {
            preceded = &text[..idx-1].trim();
            remaining = &text[idx + tag_word.len()..];
        }
        (preceded.to_owned(), tag_word.to_owned(), remaining.to_owned())
    }
}

/**
* helper fn to highlight text with annotations
* @param {String} input text
* @returns {String} highlighted text
*/
pub fn highlight_text(text: &String) -> String {
    let (preceded, tag, remaining) = get_tag_annotation_from_string(&text);
    // edge case - if no tag annotation
    if tag == "" {
        return preceded.clone();
    }
    // FIXME: small cost of cloning - tags are not long sentences
    let mut result = String::new();
    let tag_clone = tag.clone();
    let text_highlight: String = get_status_type(tag).highlight_color(tag_clone);
    result.push_str(preceded.as_str());
    result.push_str(" ");
    result.push_str(text_highlight.as_str());
    result.push_str(remaining.as_str());
    result
}

/**
* return current date time in iso string format
* @returns {String} current date in ISO string
*/
pub fn get_current_date_time_iso() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339()
}

/**
*
*/
pub fn check_string_is_i32_or_f64(string: &str) -> Option<IntFloat> {
    if let Ok(i32_value) = string.parse::<i32>() {
        return Some(IntFloat::Int(i32_value));
    }
    if let Ok(f64_value) = string.parse::<f64>() {
        if is_float_with_single_digit_decimal_places(f64_value) {
            return Some(IntFloat::Float(f64_value));
        } else {
            return None;
        }
    }
    None
}

/**
 * function to retrieve left hand side real number of a float
 * @params {f64} input number
 * @returns {i32} result int number
 */
#[allow(dead_code)]
pub fn floor_of_a_number(number: f64) -> i32 {
    number.trunc() as i32
}

/**
* function to get fractional number of a float
* @params {f64} input fraction number
* @returns {i32} integar decimal number
*/
#[allow(dead_code)]
pub fn get_fractional_number(flt_val: f64) -> i32 {
    let result = (flt_val - flt_val.floor()) * 10.0;
    result.floor() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_read_file_contains_default_keys() {
        let result: String = read_file_from_path();
        assert_eq!(true, result.as_str().contains("tags"));
        assert_eq!(true, result.as_str().contains("todos"));
        assert_eq!(true, result.as_str().contains("tasks"));
        assert_eq!(true, result.as_str().contains("reminders"));
    }

    // TODO: to only run this test locally
    #[test]
    #[ignore]
    fn test_read_file_into_strings_list() {
        let result: Vec<String> = read_file_with_lines().unwrap();
        assert_eq!(true, result.len() > 0);
    }

    #[test]
    fn test_tag_parsing() {
        let input = "This is a string @tag";
        let result = parse_tag_at_end(input);
        assert_eq!("@tag", result);
    }

    #[test]
    fn test_colored_crate_features() {
        colored_crate_features();
    }

    #[test]
    fn test_enum_from_string() {
        let input: String = String::from("@week");
        let result = get_status_type(input);
        assert_eq!(result, StatusColorType::Week);
    }

    #[test]
    fn test_parse_tag_string() {
        let input: &str = "This is a simple tag annotation @tag";
        let tag_word = parse_tag_at_end(input);
        assert_eq!("@tag", tag_word);
    }

    #[test]
    fn test_tag_when_no_annotation() {
        let input: String = String::from("text with no annotation");
        let result = get_tag_annotation_from_string(&input);
        assert_eq!(result.0, input);
    }

    #[test]
    fn test_tag_when_annotation_contains_week() {
        let input: String = String::from("text with annotation @week");
        let result = get_tag_annotation_from_string(&input);
        assert_eq!(result.0, "text with annotation");
        assert_eq!(result.1, "@week");
    }

    #[test]
    fn test_highlight_text_with_no_tag_annotation() {
        let input: String = String::from("Text with no tag annotation");
        let result = highlight_text(&input);
        assert_eq!(input, result);
    }

    #[test]
    fn test_highlight_text_with_important_annotation() {
        let input: String = String::from("Text with tag annotation as @important");
        let result = highlight_text(&input);
        println!("highlighted text: {}", &result);
        // assert_eq!(input, result);
    }

    #[test]
    fn test_is_float_with_single_digit_decimal_places_returns_false() {
        let input: f64 = 1.04;
        let result = is_float_with_single_digit_decimal_places(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_float_with_single_digit_decimal_places_returns_true() {
        let input: f64 = 1.1;
        println!(
            "Value is: {}",
            is_float_with_single_digit_decimal_places(input)
        );
        let result = is_float_with_single_digit_decimal_places(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_left_hand_side_of_float_as_int() {
        let exp = 2;
        let result = floor_of_a_number(2.5);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_get_fractional_number() {
        let exp: i32 = 5;
        let result = get_fractional_number(2.5);
        assert_eq!(result, exp);
    }

    #[test]
    fn test_get_fractional_number_two() {
        let exp: i32 = 9;
        let result = get_fractional_number(2.9);
        assert_eq!(result, exp);
    }
}
