// http://rosettacode.org/wiki/Casting_out_nines

// Takes a single number and generates its digit sum
// Sum will be between 1 and 9
fn co9(mut k: i64) -> i64 {
    // The accumulator
    let mut l: i64 = 0;
    // We reduce k until it's 0
    while k != 0 {
        // Add the 1s digit
        l += k % 10;
        // Reduce
        k /= 10;
    }
    // Remove negative, checksum is always positive
    if l < 0 {
        l = -l;
    }
    // Checksum is single digit, if not continue reducing
    if l > 9 {
        co9(l)
    } else {
        l
    }
}

fn main() {
    println!("Part 1 : co9 function");
    println!("  6395 --> 6 + 3 + 5 + 6 = 23 --> 2 + 3 = [{}]", co9(6395));
    println!("+ 1259 --> 1 + 2 + 5 + 9 = 17 --> 1 + 7 = [{}]", co9(1259));
    println!("------");
    println!("  7654 --> 7 + 6 + 5 + 4 = 22 --> 2 + 2 = [{}]", co9(7654));
    println!("");
    println!("  [{}] + [{}] = [{}]",
             co9(6395),
             co9(1259),
             co9(co9(6395) + co9(1259)));
    println!("\n");

    println!("Part 2 : filter range 1..99 where co9(k) == co9(k*k)");
    for i in 1..100 {
        if co9(i) == co9(i * i) {
            print!("{} ", i);
        }
    }
    println!("\n\n");

    println!("Part 3: efficient algorithm -- (k)%(base-1) == (k^2)%(base-1)");
    for base in &[10, 17] {
        println!("Base {}", base);
        for i in 1..(base * base) {
            if (i % (base - 1)) == ((i * i) % (base - 1)) {
                print!("{} ", i);
            }
        }
        println!("");
    }
}

#[test]
fn co9_test() {
    assert_eq!(co9(6395), 5);
    assert_eq!(co9(1259), 8);
    assert_eq!(co9(7654), 4);
    assert_eq!(co9(5), 5);
    assert_eq!(co9(8), 8);
    assert_eq!(co9(13), 4);
    assert_eq!(co9(8051305), 4);
    assert_eq!(co9(8051647), 4);
    assert_eq!(co9(3942), 9);
    assert_eq!(co9(1581), 6);
    assert_eq!(co9(2361), 3);
}
