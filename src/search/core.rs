use super::util::minimum_edit_distance;

pub fn get_motif_matches(
    sequences: Vec<String>,
    distance: usize,
    length: usize,
    indel: usize,
    sub: usize,
) -> Vec<String> {
    // TODO: implement a better version
    let mut motif_matches = Vec::new();
    for sequence in sequences {
        let mut sequence_matches = Vec::new();
        for i in 0..sequence.len() - length {
            let motif = &sequence[i..i + length];
            let mut motif_count = 0;
            for j in 0..sequence.len() - length {
                let other_motif = &sequence[j..j + length];
                if minimum_edit_distance(motif, other_motif, indel, sub) <= distance {
                    motif_count += 1;
                }
            }
            sequence_matches.push(motif_count);
        }
        motif_matches.push(sequence_matches);
    }
    return vec!["hey".to_string()];
}
