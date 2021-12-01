use super::util::minimum_edit_distance;

pub fn get_motif_matches(
    sequences: Vec<String>,
    distance: usize,
    length: usize,
    indel: usize,
    sub: usize,
) -> Vec<String> {
    // for each string in sequences
    // for each substring M in string S, where length(M) = length
    // if a neighbor M' of M occur in the rest of strings in the sequences vector
    // then put M' in the results vector.
    // M' is a neighbor of M if minimum_edit_distance(M, M') <= distance
    let mut results = vec![];
    for string in sequences {
        for i in 0..(string.len() - length) {
            let substring = &string[i..(i + length)];
            for j in 0..(string.len() - length) {
                let neighbor = &string[j..(j + length)];
                if minimum_edit_distance(substring, neighbor, indel, sub) <= distance {
                    results.push(neighbor.to_string());
                }
            }
        }
    }
    results
}

// let mut motif_matches = Vec::<String>::new();
// for sequence in sequences {
//     for i in 0..sequence.len() - length {
//         let motif = &sequence[i..i + length];
//         for j in 0..sequence.len() - length {
//             let other_motif = &sequence[j..j + length];
//             if minimum_edit_distance(motif, other_motif, indel, sub) <= distance {
//                 motif_matches.push(motif.to_string());
//             }
//         }
//     }
// }
// return motif_matches;
