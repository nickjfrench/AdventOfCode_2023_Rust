fn main() {
    let puzzle_input: String = String::from("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");

    let mut total = 0;

    let input_bytes = puzzle_input.as_bytes();

    let mut first_num: Option<&char> = None;
    let mut last_num: Option<&char> = None;

    for (i, &char) in input_bytes.iter().enumerate() {
        if char == b'\n' {
            // Handle the Some, None logic
            // Combine the two numbers to form one two-digit number = 1 + 2 = 12
            // Add to total.
            // Reset first and last nums
        }
        // Check new_line character
        // If yes, add up line_total and add to total

        // Try converting to number

        // If first_num = None, set as first_num.
        // else set as last_num
    }

    // Do final add up for the last line.
}
