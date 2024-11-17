//! Contains a function to print a string with a surrounding box.

/// Print the lines with a surrounding box.
/// 
/// This is usually used for human-readable messages in the console.
/// 
/// # Example
/// 
/// ```
/// print_with_surrounding_box(vec![
///     "This is the first line",
///     "And the second",
/// ]);
/// ```
/// 
/// prints
/// 
/// ```text
/// ┌────────────────────────┐
/// │ This is the first line │
/// │ And the second         │
/// └────────────────────────┘
/// ```
///
pub fn print_with_surrounding_box(lines_of_statements_in_response: Vec<&str>) {

    let lines_with_surrounding_box = get_surrounding_box(lines_of_statements_in_response);

    for line in lines_with_surrounding_box {println!("{}", line);}

}



/// Get the lines with a surrounding box.
/// 
/// # Example
/// 
/// ```
/// get_surrounding_box(vec![
///     "This is the first line",
///     "And the second",
/// ]);
/// vec![
///     "┌────────────────────────┐",
///     "│ This is the first line │",
///     "│ And the second         │",
///     "└────────────────────────┘",
/// ]);
/// ```
///
fn get_surrounding_box(lines_of_statements_in_response: Vec<&str>) -> Vec<String> {

    // make a vector of strings that we can push onto
    let mut lines_with_surrounding_box = Vec::<String>::new();

    // map or iterate through the vector of lines and return the greatest length
    let max_of_lengths = lines_of_statements_in_response.iter().map(|line| line.len()).max();

    // handle the option such that if there is no max length, we can set it to 0,
    // otherwise, we can use the value
    let max_len = max_of_lengths.unwrap_or(0);

    // make a vector of filler spaces to add to the end of each line
    let filler_spaces_for_statements = lines_of_statements_in_response.iter().map(
        |line| " ".repeat(max_len - line.len())
    ).collect::<Vec<String>>();

    let horizontal_lines = "─".repeat(max_len);

    // push the first item onto the vector
    lines_with_surrounding_box.push(format!("┌─{}─┐", horizontal_lines));

    for (line, filler) in lines_of_statements_in_response.iter().zip(filler_spaces_for_statements.iter()) {
        lines_with_surrounding_box.push(format!("│ {}{} │",line, filler));
    }
    lines_with_surrounding_box.push(format!("└─{}─┘", horizontal_lines));

    lines_with_surrounding_box
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_function() {
        // use crate::utilities; // Adjust this based on your crate name

        let lines_of_statements_in_response = vec![
            "This is the first line",
            "And the second",
        ];
        let result = get_surrounding_box(lines_of_statements_in_response);

        assert_eq!(result, vec![
            "┌────────────────────────┐",
            "│ This is the first line │",
            "│ And the second         │",
            "└────────────────────────┘",
        ]);
    }
}