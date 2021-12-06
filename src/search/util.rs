use rand::prelude::*;
use std::cmp::min;

pub fn generate_sequence(size: usize) -> String {
    // generate a random ATGC sequence
    let mut rng = thread_rng();
    let mut sequence = String::with_capacity(size);
    for _ in 0..size {
        let rand_char = rng.gen_range(0..4);
        match rand_char {
            0 => sequence.push('A'),
            1 => sequence.push('T'),
            2 => sequence.push('G'),
            3 => sequence.push('C'),
            _ => panic!("Invalid random number"),
        }
    }
    return sequence;
}

pub fn minimum_edit_distance(a: &str, b: &str, indel: usize, sub: usize) -> usize {
    // calculate the minimum edit distance between a and b
    // indel is the cost of insertion or delete
    // sub is the cost of substitution
    let a_len = a.len();
    let b_len = b.len();
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
    for i in 0..a_len + 1 {
        for j in 0..b_len + 1 {
            if i == 0 {
                matrix[i][j] = j * indel;
            } else if j == 0 {
                matrix[i][j] = i * indel;
            } else if a[i - 1..i] == b[j - 1..j] {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                matrix[i][j] = min(
                    matrix[i - 1][j - 1] + sub,
                    min(matrix[i - 1][j] + indel, matrix[i][j - 1] + indel),
                );
            }
        }
    }
    return matrix[a_len][b_len];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_edit_distance_works() {
        assert_eq!(minimum_edit_distance("siddharth", "sidhdhant", 1, 2), 4);
    }

    #[test]
    fn generate_sequence_works() {
        let seq = generate_sequence(100);
        assert_eq!(seq.len(), 100);
        assert!(seq
            .chars()
            .all(|c| c == 'A' || c == 'T' || c == 'G' || c == 'C'));
    }
}
