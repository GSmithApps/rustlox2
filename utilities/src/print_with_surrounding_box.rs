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
fn get_surrounding_box(lines_of_text: Vec<&str>) -> Vec<String> {
    
    let max_len = lines_of_text.iter().map(|line| line.len()).max().unwrap_or(0);
    
    let horizontal_lines = "─".repeat(max_len);

    std::iter::once(
        format!("┌─{}─┐", horizontal_lines).into()).chain(
        lines_of_text.iter().map(|line| format!("│ {}{} │", line, " ".repeat(max_len - line.len())))).chain(
        std::iter::once(format!("└─{}─┘", horizontal_lines).into())).collect()
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