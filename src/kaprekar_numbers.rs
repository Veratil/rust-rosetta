// http://rosettacode.org/wiki/Kaprekar_numbers

fn is_kaprekar(k: i64) -> bool {
    // Only positive numbers are Kaprekar numbers
    if k < 1 {
        return false;
    }
    // 1 is a Kaprekar number
    if k == 1 {
        return true;
    }
    // Square the input
    let sq = k * k;
    // 10^18 is the largest 10^x for i64 where you can split a number
    let mut split = 10i64.pow(18);
    // The left number of the split
    let mut left = sq / split;
    // Until we find a split where the left != 0
    while left == 0 {
        // Reduce the split location
        split /= 10;
        // New left number
        left = sq / split;
    }
    // Get the right number
    let mut right = sq % split;

    // Continue until the split won't split anything
    while split != 1 {
        // if right == 0, splitting further down will always be 0
        // 0 is not considered positive, and is invalid
        if right == 0 {
            return false;
        }
        // If the left + right == input, it's a Kaprekar number
        if left + right == k {
            return true;
        }
        // Reduce the split location
        split /= 10;
        // New left number
        left = sq / split;
        // New right number
        right = sq % split;
    }

    // If we reach here, we do not have a Kaprekar number
    false
}

fn is_kaprekar_base(base: i64, k: i64) -> bool {
    let sq = k * k;
    let mut tens = 1;
    if (sq - k) % (base - 1) != 0 {
        return false;
    }
    while tens < k {
        tens *= base;
    }
    if k == tens {
        return 1 == k;
    }
    let mut r = sq % tens;
    while r < k {
        if sq / tens + r == k {
            return true;
        }
        tens *= base;
        r = sq % tens;
    }
    false
}


// str_is_kaprekar takes a string and converts it to i64,
// returning the result from is_kaprekar
#[allow(dead_code)]
fn str_is_kaprekar(sk: String) -> bool {
    use std::str::FromStr;
    let k: i64 = match i64::from_str(sk.as_str()) {
        Ok(i) => i,
        Err(_) => 0,
    };
    is_kaprekar(k)
}

fn main() {
    println!("Kaprekar numbers between 1..10000");
    for i in 1..10_000 {
        if is_kaprekar(i) {
            print!("{} ", i);
        }
    }
    println!("");

    println!("Extra credit: Count Kaprekar less than 1,000,000");
    let mut count = 0;
    for i in 1..1_000_000 {
        if is_kaprekar(i) {
            count += 1;
        }
    }
    println!("There are {} Kaprekar numbers less than 1,000,000", count);
    println!("");

    println!("Extra extra credit: base 17 between 1 and 1,000,000");
    count = 0;
    for i in 1..1_000_000 {
        if is_kaprekar_base(17, i) {
            count += 1;
            print!("{} ", i);
        }
    }
    println!("");
}

#[test]
fn kaprekar_numeric() {
    assert_eq!(is_kaprekar(-1), false);
    assert_eq!(is_kaprekar(1), true);
    assert_eq!(is_kaprekar(9), true);
    assert_eq!(is_kaprekar(10), false);
    assert_eq!(is_kaprekar(45), true);
    assert_eq!(is_kaprekar(100), false);
}

#[test]
fn kaprekar_string() {
    assert_eq!(str_is_kaprekar(String::from("-1")), false);
    assert_eq!(str_is_kaprekar(String::from("1")), true);
    assert_eq!(str_is_kaprekar(String::from("9")), true);
    assert_eq!(str_is_kaprekar(String::from("10")), false);
    assert_eq!(str_is_kaprekar(String::from("45")), true);
    assert_eq!(str_is_kaprekar(String::from("100")), false);
}

#[test]
fn kaprekar_base() {
    assert_eq!(is_kaprekar_base(10, -1), false);
    assert_eq!(is_kaprekar_base(17, -1), false);
    assert_eq!(is_kaprekar_base(10, 1), true);
    assert_eq!(is_kaprekar_base(17, 1), true);
    assert_eq!(is_kaprekar_base(10, 9), true);
    assert_eq!(is_kaprekar_base(17, 9), false);
    assert_eq!(is_kaprekar_base(10, 16), false);
    assert_eq!(is_kaprekar_base(17, 16), true);
    assert_eq!(is_kaprekar_base(10, 45), true);
    assert_eq!(is_kaprekar_base(17, 45), false);
    assert_eq!(is_kaprekar_base(10, 100), false);
    assert_eq!(is_kaprekar_base(17, 100), false);
}
