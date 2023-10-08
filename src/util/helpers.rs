use std::io::{self, BufRead, Read};
use std::fs::{self, File};
use crate::util::config::get_db_file_path;
use colored::Colorize;
use crate::util::enums::StatusColorType;

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
    file.read_to_string(&mut json_string).unwrap();
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
        _ => StatusColorType::Info
    }
}

/**
* helper fn to retrieve tags from a given sentence
* @param {String} text
* @returns tuple {(String, usize, usize)} - word, start_index, end_index
* Using 2 pointers
*/
pub fn get_tag_annotation_from_string(text: &String) -> (String, usize, usize) {
    // guard check
    if !text.contains("@") {
        return (String::from(""), 0, 0);
    }
    let start = text.find("@").unwrap();
    let mut end = start+1;
    while end < text.len() && !text.chars().nth(end).unwrap().is_whitespace() {
        end += 1;
    }
    (text[start..end].to_string(), start, end)
}

/**
* helper fn to highlight text with annotations
* @param {String} input text
* @returns {String} highlighted text
*/
pub fn highlight_text(text: &String) -> String {
    let (tag, i, j) = get_tag_annotation_from_string(&text);
    // edge case - if no tag annotation
    if tag == "" { return text.clone(); }
    // FIXME: small cost of cloning - tags are not long sentences
    let mut result = String::new();
    let tag_clone = tag.clone();
    let text_highlight: String = get_status_type(tag).highlight_color(tag_clone);
    result.push_str(&text[..i]);
    result.push_str(text_highlight.as_str());
    result.push_str(&text[j..]);
    result
}


// ******************* FOR INTERNAL TESTING PURPOSE *************

/**
* TESTING: Using BufReader for reading large files
* FOR TESTING PURPOSE
*/
fn read_file_with_lines() -> Result<Vec<String>, String> {
    let source_json = "jotdown-db.json";
    let file:File = File::open(source_json).unwrap();
    // another approach - using io::BufReader for large files
    let reader = io::BufReader::new(file);
    let result: Vec<String> = reader.lines().into_iter()
                                .map(|item| item.unwrap()).collect();
    Ok(result)
}

/**
 * TESTING: Using Vectors fith file reads
 * FOR TESTING PURPOSE
 */
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
fn colored_crate_features() {
    "this is blue".blue();
    "this is red".red();
    "this is red on blue".red().on_blue();
    "this is also red on blue".on_blue().red();
    "you can use truecolor values too!".truecolor(0, 255, 136);
    "background truecolor also works :)".on_truecolor(135, 28, 167);
    "you can also make bold comments".bold();
    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    "or change advice. This is red".yellow().blue().red();
    "or clear things up. This is default color and style".red().bold().clear();
    "purple and magenta are the same".purple().magenta();
    "bright colors are also allowed".bright_blue().on_bright_white();
    "you can specify color by string".color("blue").on_color("red");
    "and so are normal and clear".normal().clear();
    String::from("this also works!").green().bold();
    let result: String = format!("{}", "format works as expected. This will be padded".on_truecolor(135, 28, 167));
    let output: String = format!("{}", "and this will be green but truncated to 3 chars".red().bold().clear());
    println!("{}", result);
    println!("{}", output);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_contains_default_tags() {
        let result: String = read_file_from_path();
        assert_eq!(true, result.as_str().contains("tags"));
        assert_eq!(true, result.as_str().contains("important"));
        assert_eq!(true, result.as_str().contains("today"));
        assert_eq!(true, result.as_str().contains("week"));
    }

    #[test]
    fn test_read_file_into_strings_list() {
        let result: Vec<String> = read_file_with_lines().unwrap();
        assert_eq!(true, result.len() > 0);
    }

    #[test]
    fn test_return_lines_only_if_contains_string() {
        let result: Vec<String> = return_lines_only_if_contains_string("important").unwrap();
        // println!("result is: {:?}", &result);
        assert_eq!(true, result.len() > 0);
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
    fn test_tag_when_no_annotation() {
        let input: String = String::from("text with no annotation");
        let result = get_tag_annotation_from_string(&input);
        assert_eq!(result.0, "");
    }

    #[test]
    fn test_tag_when_annotation_contains_week() {
        let input: String = String::from("text with annotation @week");
        let result = get_tag_annotation_from_string(&input);
        assert_eq!(result.0, "@week");
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
}