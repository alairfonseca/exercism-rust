use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let size_are_equal = first_list.len().cmp(&second_list.len());

    match size_are_equal {
        Ordering::Less => {
            if is_sublist(first_list, second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        },
        Ordering::Equal => {
            if is_sublist(first_list, second_list) {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        },
        Ordering::Greater => {
            if is_sublist(second_list, first_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn is_sublist<T: PartialEq>(smaller_slice: &[T], bigger_slice: &[T]) -> bool {
    if !smaller_slice.is_empty() {
        bigger_slice.windows(smaller_slice.len()).any(|w| w == smaller_slice)
    } else {
        true
    }
}
