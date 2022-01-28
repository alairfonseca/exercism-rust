pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum: u32 = 0;

    let digits = number_to_vec(num);
    let power = digits.len() as u32;

    for i in digits {
        sum += u32::pow(i, power);
    }

    sum == num
}

fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
