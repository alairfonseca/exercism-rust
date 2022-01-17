// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter
    .enumerate()
    .filter(|&(idx, _)| ((idx == 0) || ((idx > 0) && (idx % 2) == 0)))
    .map(|(_, element)| element).into_iter()
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        i16::abs(self.0) + i16::abs(self.1)
    }
}
