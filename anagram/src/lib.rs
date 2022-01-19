use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    let formated_word = sort_word(&word); 

    for possible_anagram in possible_anagrams.iter() {
        let formated_pa = sort_word(&possible_anagram);

        if formated_word.to_lowercase().eq(&formated_pa.to_lowercase()) && word.to_lowercase().ne(&possible_anagram.to_lowercase()) {
            result.insert(*possible_anagram);
        }
    }

    result
}

fn sort_word(word: &str) -> String {
    let mut sorted_chars: Vec<char> = word.to_lowercase().chars().collect();
    sorted_chars.sort();

    let formated_word: String = sorted_chars.into_iter().collect();

    formated_word
}
