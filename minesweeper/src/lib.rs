
const MINE: char = '*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    
    for (row_idx, row) in minefield.iter().enumerate() {
        let mut row_result = String::from("");

        for (square_idx, square) in row.as_bytes().iter().enumerate() {
            if *square == MINE as u8 {
                row_result.push(MINE);
            } else {
                let count = &scan(square_idx, minefield, row_idx);

                if *count > 0 {
                    row_result = format!("{}{}", row_result, *count);
                } else {
                    row_result.push(' ');
                }
            }
        }

        result.push(row_result);
    }
    
    result
}

fn scan(square_idx: usize, minefield: &[&str], row_idx: usize) -> u8 {
    let mut count = 0;

    if row_idx > 0 {
        count += count_in_adjacent_row(square_idx, minefield[row_idx - 1]);
    }

    if row_idx + 1 < minefield.len() {
        count += count_in_adjacent_row(square_idx, minefield[row_idx + 1]);
    }

    count += count_sides(square_idx, minefield[row_idx]);

    count
}

fn count_in_adjacent_row(square_idx: usize, row: &str) -> u8 {
    let mut count = 0;
    
    let start = if square_idx > 0 { square_idx - 1 } else { square_idx };
    let end = if square_idx + 1 < row.len() { square_idx + 1 } else { square_idx };
    
    let range = std::ops::RangeInclusive::new(start, end);

    for ch in &row.as_bytes()[range] {
        println!("CASA: {}", *ch);
        if *ch == MINE as u8 {
            count += 1;
        }
    }

    count
}

fn count_sides(square_idx: usize, row: &str) -> u8 {
    let mut count = 0;

    let start = if square_idx > 0 { square_idx - 1 } else { square_idx };
    let end = if square_idx + 1 < row.len() { square_idx + 1 } else { square_idx };
    let range = std::ops::RangeInclusive::new(start, end);
    
    for ch in row.as_bytes()[range].iter() {
        if *ch == MINE as u8 {
            count += 1;
        }
    }
    
    count
}

