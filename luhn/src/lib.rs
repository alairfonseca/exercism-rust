/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 || code.trim().chars().any(|ch| !ch.is_whitespace() && (ch.is_alphabetic() || !ch.is_digit(10))) {
        return false;
    }

    let mut numbers: Vec<u8> = code.chars()
                                .filter(|ch| !ch.is_whitespace())
                                .map(|ch| ch.to_string().parse().unwrap())
                                .collect();

    check_luhn(&mut numbers)
}

fn check_luhn(numbers: &mut Vec<u8>) -> bool {
    numbers.reverse();
    
    let mut sum = 0;
    
    for i in 0..numbers.len() {
        let mut digit = numbers[i];

        if !(i % 2 == 0) {
            digit = digit * 2;
        }
        
        if digit > 9 {
            digit = digit - 9;
        }

        sum += digit;
    }

    return sum % 10 == 0;
}
