// http://rosettacode.org/wiki/Digital_root

// Returns the tuple (digital root, persistance)
fn digital_root(base: i64, mut k: i64) -> (i64, i64) {
    // Digital root sum
    let mut l: i64 = 0;
    // Persistance
    let mut p: i64 = 1;
    // Remove the negative, not needed
    if k < 0 {
        k = -k;
    }
    // Loop forever until break
    loop {
        // Loop through the digits
        while k != 0 {
            l += k % base;
            k /= base;
        }
        // If sum is a single digit, break out of loop
        if l < base {
            break;
        }
        // We're going to new summation, persistance +1
        p += 1;
        // Reset
        k = l;
        l = 0;
    }
    // Return tuple (sum, persistance)
    (l, p)
}

fn main() {
    for base in &[2i64, 3, 8, 10, 16, 36] {
        for n in &[5i64, 627615, 39390, 588225, 393900588225] {
            let (r, p) = digital_root(*base, *n);
            println!("{} (using base{}) has additive persistance {} and digital root of {}",
                     n,
                     base,
                     p,
                     r);
        }
        println!("");
    }
}
