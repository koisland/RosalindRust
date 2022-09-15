// std::fs (file system) to read files.
use std::fs;

pub struct Seq {
    a: u32,
    t: u32,
    g: u32,
    c: u32,
    u: u32,
    n: u32
}

pub fn cnt_nt(fname: &String) -> Nts {
    // Counts the nucleotides in a given file containing a nucleotide sequence.
    // 
    // # Arguments
    // * `fname` - A filename as a string reference.
    // 
    // # Returns
    // * `nts` - A struct containing the enumerated nucleotides.

    let contents = fs::read_to_string(fname)
        .expect("Unable to read file.");
    
    // Initialize nts struct.
    let mut seq: Seq = Seq {a: 0, t: 0, g: 0, c: 0, u: 0, n: 0};

    // Iterate through all characters in trimmed string.
    for nt in contents.trim().chars() {
        match nt {
            'A' | 'a' => seq.a += 1,
            'T' | 't' => seq.t += 1,
            'G' | 'g' => seq.g += 1,
            'C' | 'c' => seq.c += 1,
            'U' | 'u' => seq.u += 1,
            // Show unclassified characters.
            _ => {
                println!("{}", nt);
                seq.n += 1
            }
        }
    }

    println!("A: {}, T: {}, G: {}, C: {}, U: {}, N: {}", seq.a, seq.t, seq.g, seq.c, seq.u, seq.n);

    return seq;
}