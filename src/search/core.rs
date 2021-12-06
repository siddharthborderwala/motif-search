use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::util::minimum_edit_distance;

use rayon::prelude::*;

pub fn generate_fixed_subs<'a>(sequence: &'a str, length: usize) -> Vec<&'a str> {
    let size = sequence.len();
    let mut subsequences = Vec::<&'a str>::new();
    for i in 0..size - length + 1 {
        subsequences.push(&sequence[i..i + length]);
    }
    return subsequences;
}

pub fn generate_subsequences<'a>(
    sequence: &'a str,
    length: usize,
    distance: usize,
) -> Vec<&'a str> {
    let size = sequence.len();
    let mut subsequences = Vec::<&'a str>::new();
    for l in length - distance..length + distance + 1 {
        for i in 0..size - l + 1 {
            subsequences.push(&sequence[i..i + l]);
        }
    }
    return subsequences;
}

pub fn get_motif_matches(
    sequences: Vec<String>,
    distance: usize,
    length: usize,
    indel: usize,
    sub: usize,
) -> Vec<String> {
    let total_sequences = sequences.len();
    let mut subsequences_map = HashMap::<usize, Vec<&str>>::new();
    let mut fixed_subs_map = HashMap::<usize, Vec<&str>>::new();
    let results = Arc::new(Mutex::new(Vec::<String>::new()));

    sequences.iter().enumerate().for_each(|(n, sequence)| {
        let subsequences = generate_subsequences(&sequence, length, distance);
        let fix_subs = generate_fixed_subs(&sequence, length);
        subsequences_map.insert(n, subsequences);
        fixed_subs_map.insert(n, fix_subs);
    });

    sequences.par_iter().enumerate().for_each(|(n, _)| {
        let fixed_subsequences = fixed_subs_map.get(&n).unwrap();
        for s in fixed_subsequences {
            let mut count = 0;
            for oi in 0..total_sequences {
                if n == oi {
                    continue;
                }
                let mut flag = false;
                let other_subs = subsequences_map.get(&oi).unwrap();

                for s2 in other_subs {
                    if minimum_edit_distance(s, *s2, indel, sub) <= distance {
                        flag = true;
                        break;
                    }
                }
                if flag {
                    count += 1;
                } else {
                    break;
                }
            }

            if count == total_sequences - 1 {
                let r = Arc::clone(&results);
                r.lock().unwrap().push(s.to_string());
            }
        }
    });

    return results.lock().unwrap().to_vec();
}
