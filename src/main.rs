use std::io::{self, Write};

const MIN_ALLOWED_VALUE : i16 = (std::i16::MIN + 1);
const MAX_ALLOWED_VALUE : i16 = std::i16::MAX;

// Number of bytes under which `p` should be represented.
// Equal to: number of bytes for multiplicand + number of bytes for multiplier + 1
const P_NUMBER_BYTES : u8 = 16 + 16 + 1;

fn main() {
    let (number1, number2) = read_two_i16s();
    println!("Result: {}", booth_mult(number1, number2));
}

/// Try to read to i16 from stdin and returns them.
/// Just print to stderr and exit if an error arised.
fn read_two_i16s() -> (i16, i16) {
    let mut number1_input = String::new();
    print!("Integer 1 (between {} and {}): ", std::i16::MIN + 1, std::i16::MAX);
    io::stdout().flush().unwrap();
    if let Err(e) = io::stdin().read_line(&mut number1_input) {
        eprintln!("Error: Could not read `number 1` value: {}.", e);
        std::process::exit(1);
    };
    let number1 = parse_i16(number1_input);

    let mut number2_input = String::new();
    print!("Integer 2: ");
    io::stdout().flush().unwrap();
    if let Err(e) = io::stdin().read_line(&mut number2_input) {
        eprintln!("Error: Could not read `number 2` value: {}.", e);
        std::process::exit(1);
    };
    let number2 = parse_i16(number2_input);
    (number1, number2)
}

fn parse_i16(number_str : String) -> i16 {
    match number_str.trim().parse::<i64>() {
        Ok(x) if x < (MIN_ALLOWED_VALUE as i64) || x > (MAX_ALLOWED_VALUE as i64) => {
            eprintln!("Error: Please enter a value between {} and {}.",
                MIN_ALLOWED_VALUE, MAX_ALLOWED_VALUE);
            std::process::exit(1);
        },
        Err(_) => {
            eprintln!("Error: Unrecognized number (too big?).");
            std::process::exit(1);
        },
        Ok(n) => n as i16
    }
}

/// Simple implementation of Booth's multiplication algorithm.
fn booth_mult(number1 : i16, number2 : i16) -> i16 {
    if number1 == 0 || number2 == 0 {
        return 0;
    }

    // Cast those to as u64s by just adding 48 `0` in front of it.
    // We first cast it to an u16 as we don't want to expand's the two's
    // complement here, we just want to keep the least significant bits as is
    // and fill the rest with 0.
    let number1_u64 = number1 as u16 as u64;
    let number2_u64 = number2 as u16 as u64;

    // I felt that just doing -number1 was cheating here as we relied to much on
    // Rust there, so I calculate the opposite of number1 in a much more
    // rudimentary manner
    let minus_number1_u64 = !((number1 as u16) - 1) as u64;

    let a = number1_u64 << (16 + 1);
    let s = minus_number1_u64 << (16 + 1);
    let mut p = number2_u64 << 1;

    for _ in 0..16 {
        let val = match p & 0b11 {
            0b01 => (p + a) & ((1 << P_NUMBER_BYTES) - 1),
            0b10 => (p + s) & ((1 << P_NUMBER_BYTES) - 1),
            _ => p,
        };
        p = val >> 1;
    }
    (p >> 1) as i16
}

#[test]
fn test_mult() {
    for i in MIN_ALLOWED_VALUE ..=MAX_ALLOWED_VALUE {
        for j in MIN_ALLOWED_VALUE ..=MIN_ALLOWED_VALUE {
            let result : i32 = (i as i32) * (j as i32);
            if result > (std::i16::MAX as i32) || result < (std::i16::MIN as i32) {
                break;
            }
            assert_eq!(result as i16, booth_mult(i, j));
        }
    }
}
