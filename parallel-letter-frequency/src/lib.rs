use std::thread;
use std::collections::HashMap;
use std::cmp;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut workers = vec![];
    let mut result = HashMap::new();
    let slices_per_worker = cmp::min(1, input.len() / worker_count);

    if input.len() == 0 {
        return HashMap::new();
    }

    if input.len() == 1 {
        return count_chars(input[0]);
    }

    let chunks = input.chunks(slices_per_worker);

    for texts in chunks {
        let txt = texts.join("");

        let worker = thread::spawn(move || count_chars(&txt));

        workers.push(worker);
    }

   for worker in workers {
        let worker_result = worker.join().unwrap();

        for (&char, &count) in worker_result.iter() {
            *result.entry(char).or_insert(0) += count;
        }
    }
    
    result
}

fn count_chars(input: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    for c in input.to_lowercase().chars().filter(|ch| ch.is_alphabetic()) {

        let key = result.entry(c).or_insert(0);
    
        *key += 1;
    }

    result
}
