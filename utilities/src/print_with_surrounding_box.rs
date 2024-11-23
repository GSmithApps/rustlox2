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

    let mut lines_of_statements_in_response = lines_of_statements_in_response.iter().map(|line| line.to_string()).collect::<Vec<String>>();

    get_surrounding_box(&mut lines_of_statements_in_response);

    for line in lines_of_statements_in_response {println!("{}", line);}

}



/// Get the lines with a surrounding box.
/// 
/// Note that the first line and the last line need to be empty
/// 
/// # Example
/// 
/// ```
/// get_surrounding_box(vec![
///     String::from("This is the first line"),
///     String::from("And the second"),
/// ]);
/// vec![
///     String::from("┌────────────────────────┐"),
///     String::from("│ This is the first line │"),
///     String::from("│ And the second         │"),
///     String::from("└────────────────────────┘"),
/// ]);
/// ```
///
fn get_surrounding_box(lines_of_text: &mut Vec<String>)
{
    let max_len = lines_of_text.iter().map(|line| line.len()).max().unwrap_or(0);
    
    let horizontal_lines = "─".repeat(max_len);

    for line_to_print in lines_of_text.iter_mut() {
        *line_to_print = format!("│ {}{} │", &line_to_print, " ".repeat(max_len - line_to_print.len()))
    }

    lines_of_text.insert(0,format!("┌─{}─┐", horizontal_lines));

    lines_of_text.push(format!("└─{}─┘", horizontal_lines));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_function() {
        // use crate::utilities; // Adjust this based on your crate name

        let mut lines_of_statements_in_response = vec![
            String::from("This is the first line"),
            String::from("And the second"),
        ];
        get_surrounding_box(&mut lines_of_statements_in_response);

        let intended_result = vec![
            String::from("┌────────────────────────┐"),
            String::from("│ This is the first line │"),
            String::from("│ And the second         │"),
            String::from("└────────────────────────┘"),
        ];

        assert_eq!(lines_of_statements_in_response, intended_result);
    }
}