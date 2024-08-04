
/// Print the lines with a surrounding box.
/// 
/// The call is something like
/// 
/// ```
/// let lines_of_statements_in_response = vec![
///     "We received too many parameters (the limit is 1).  We received:",
///     "but it should be something like:",
/// ];
/// utilities::print_with_surrounding_box(lines_of_statements_in_response);
/// ```
/// 
/// it will print:
/// 
/// ```text
///  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
///  @@                                                     @@
///  @@  We received too many parameters (the limit is 1).  @@
///  @@  but it should be something like:                   @@
///  @@                                                     @@
///  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
/// ```
/// 
pub fn print_with_surrounding_box(lines_of_statements_in_response: Vec<&str>) -> () {
    // map or iterate through the vector of lines and return the greatest length
    let max_len = lines_of_statements_in_response.iter().map(|line| line.len()).max().unwrap();

    // make a vector of filler spaces to add to the end of each line
    let filler_spaces_for_statements = lines_of_statements_in_response.iter().map(|line| std::iter::repeat(' ').take(max_len - line.len()).collect::<String>()).collect::<Vec<String>>();

    let at_signs = std::iter::repeat('@').take(max_len).collect::<String>();
    let spaces = std::iter::repeat(' ').take(max_len).collect::<String>();
    println!();
    println!(" @@@@{}@@@@", at_signs);
    println!(" @@  {}  @@", spaces);
    for (line, filler) in lines_of_statements_in_response.iter().zip(filler_spaces_for_statements.iter()) {
        println!(" @@  {}{}  @@", line, filler);
    }
    println!(" @@  {}  @@", spaces);
    println!(" @@@@{}@@@@", at_signs);
    println!();
}
