use std::io::{self, Write};

// Number of bytes under which `p` should be represented.
// Equal to: number of bytes for multiplicand + number of bytes for multiplier + 1
const P_NUMBER_BYTES : u8 = 16 + 16 + 1;

/// Simple implementation of Booth's multiplication algorithm.
fn main() {
    let (number1, number2) = read_two_i16s();

    // Cast those to as u64s by just adding 48 `0` in front of it.
    // We first cast it to an u16 as we don't want to expand's the two's
    // complement here, we just want to keep the least significant bits as is
    // and fill the rest with 0.
    let number1_u64 = number1 as u16 as u64;
    let minus_number1_u64 = (-number1) as u16 as u64;
    let number2_u64 = number2 as u16 as u64;

    let a = number1_u64 << (16 + 1);
    let s = minus_number1_u64 << (16 + 1);
    let mut p = number2_u64 << 1;

    for _ in 0..16 {
        let val = match p & 0b11 {
            0b00 | 0b11 => p,
            0b01 => (p + a) & ((1 << P_NUMBER_BYTES) - 1),
            0b10 => (p + s) & ((1 << P_NUMBER_BYTES) - 1),
            _ => p,
        };
        p = val >> 1;
    }
    p = p >> 1;
    println!("Result: {}", p as i16);
}

/// Try to read to i16 from stdin and returns them.
/// Just print to stderr and exit if an error arised.
fn read_two_i16s() -> (i16, i16) {
    let mut number1_input = String::new();
    print!("Integer 1: ");
    io::stdout().flush().unwrap();
    if let Err(e) = io::stdin().read_line(&mut number1_input) {
        eprintln!("Error: Could not read `number 1` value: {}.", e);
        std::process::exit(1);
    };
    let number1 : i16 = match number1_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Expected a valid signed 8 bit integer as first argument.");
            std::process::exit(1);
        }
    };

    let mut number2_input = String::new();
    print!("Integer 2: ");
    io::stdout().flush().unwrap();
    if let Err(e) = io::stdin().read_line(&mut number2_input) {
        eprintln!("Error: Could not read `number 2` value: {}.", e);
        std::process::exit(1);
    };
    let number2 : i16 = match number2_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Expected a valid signed 8 bit integer as second argument.");
            std::process::exit(1);
        }
    };
    (number1, number2)
}
