use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;

use super::util::minimum_edit_distance;

use rayon::prelude::*;

pub fn generate_subsequences(sequence: &str, length: usize) -> Vec<String> {
    let l = sequence.len();
    let mut subsequences = Vec::<String>::with_capacity(l + 1);
    for i in 0..l - length + 1 {
        subsequences.push(sequence[i..i + length].to_string());
    }
    return subsequences;
}

pub fn get_motif_matches(
    filename: PathBuf,
    sequences: Vec<String>,
    distance: usize,
    length: usize,
    indel: usize,
    sub: usize,
) {
    let total_sequences = sequences.len();
    let seq_clone = sequences.clone();
    let mut subsequences_map = HashMap::<String, Vec<String>>::new();

    sequences.iter().for_each(|sequence| {
        let subsequences = generate_subsequences(&sequence, length);
        subsequences_map.insert(sequence.clone(), subsequences);
    });

    sequences.par_iter().for_each(|sequence| {
        let subsequences = subsequences_map.get(sequence).unwrap();

        for s in subsequences {
            let mut count_check = 0;
            for o in seq_clone.iter() {
                if s != o {
                    let mut flag = false;
                    let osubseq = subsequences_map.get(o).unwrap();

                    for s2 in osubseq {
                        if minimum_edit_distance(&s2, &s, indel, sub) <= distance {
                            flag = true;
                            break;
                        }
                    }
                    if flag {
                        count_check += 1;
                    } else {
                        break;
                    }
                }
            }

            if count_check == total_sequences - 1 {
                let mut f = std::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(&filename)
                    .unwrap();
                f.write(s.as_bytes()).unwrap();
                f.write(b"\n").unwrap();
            }
        }
    });
}
