// http://rosettacode.org/wiki/Floyd's_triangle

fn floyd(rows: usize) {
    // 0 rows = 0 output
    if rows == 0 {
        return;
    }
    // last row starts with (n^2 - n + 2) / 2
    // we'll convert that number to a string and get the length
    let last_row_starting_number = ((rows * rows) - rows + 2) / 2;
    // Current number we're displaying, easier to count
    let mut cur = 1;
    // Start with row 0 because .. operator acts as:
    // inclusive..exclusive, so 1..rows would actually be 1,...,rows-1
    for r in 0..rows {
        // Since we're a right triangle, the current row is also how many
        // columns we'll have, we do +1 to account for the ..exclusive
        for c in 0..r + 1 {
            // Length of the number as a string
            let width = (last_row_starting_number + c).to_string().len();
            // Print the number, using 0$ to use width in place
            print!("{num:>0$} ", width, num = cur);
            // Increment accumulator
            cur += 1;
        }
        // New line for new row
        println!("");
    }
}

fn main() {
    floyd(5);
    floyd(14);
}
