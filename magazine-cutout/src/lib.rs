use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut can_construct = true;

    let note_iter = note.iter();

    let mut magazine_counted_words = count_words(magazine);

    for word in note_iter {
        let key = String::from(*word);

        if magazine_counted_words.contains_key(&key) && magazine_counted_words.get(&key).unwrap() > &0 {
            let counter = magazine_counted_words.entry(key).or_insert(1);
            *counter -= 1;
        } else {
            can_construct = false;
        }
    }

    can_construct
}

fn count_words(text: &[&str]) -> HashMap<String, usize> {
    let text_iter = text.iter();
    let mut result = HashMap::new();

    for word in text_iter {
        let count = result.entry(String::from(*word)).or_insert(0);
        *count += 1;
    }

    result
}
