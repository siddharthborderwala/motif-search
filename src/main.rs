use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

use search::core::get_motif_matches;
use search::util::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Motif Search",
    about = "Motif search based on Minimum Edit Distance."
)]
struct Opt {
    // Input file's path
    // default value is input.txt in the current directory
    #[structopt(short, long, parse(from_os_str), default_value = "./input.txt")]
    input: PathBuf,

    // Output file's path
    // default value is output.txt in the current directory
    #[structopt(short, long, parse(from_os_str), default_value = "./output.txt")]
    output: PathBuf,

    // The minimum edit distance
    #[structopt(short, long = "distance", name = "minimum-edit-distance")]
    d: usize,

    // The length of the motif
    #[structopt(short, long = "length", name = "motif-length")]
    l: usize,

    // The indel cost
    #[structopt(short, long = "indel", name = "indel-cost")]
    indel: usize,

    // The substitution cost
    #[structopt(short, long = "sub", name = "substitution-cost")]
    sub: usize,
}

fn write_vec_to_file(path: PathBuf, input: &Vec<String>) {
    let mut file = File::create(path).unwrap();
    for line in input {
        writeln!(file, "{}", line).expect("Failed to write to file");
    }
}

const SIZE: usize = 600;

fn main() {
    // get the arguments from the command line
    let opt = Opt::from_args();
    // minimum edit distance
    let distance = opt.d;
    // substring length
    let length = opt.l;

    // initialize the array to store the sequences
    let mut sequences = Vec::<String>::with_capacity(20);
    for _ in 0..20 {
        sequences.push(generate_sequence(SIZE));
    }
    // write the sequences generated to the input file
    write_vec_to_file(opt.input, &sequences);

    // get the matches
    let result = get_motif_matches(sequences, distance, length, opt.indel, opt.sub);

    // write the matches to the output file
    write_vec_to_file(opt.output, &result);
}
