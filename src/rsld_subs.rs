use std::fs;

pub fn find_motif(fname: &String) -> String{
    let contents = fs::read_to_string(fname)
        .expect("Unable to read file.");
    
    // split by line
    let seqs: Vec<&str> = contents.lines().collect();

    // get new copy (not ptr) of seq and substring
    let seq = seqs.get(0).expect("No sequence.").to_owned();
    let substr = seqs.get(1).expect("No substring.").to_owned();

    // allocate mem for vector of u32
    let mut substr_indices: Vec<usize> = Vec::new(); 

    for (i, _) in seq.chars().enumerate() {
        // sequence slice with same num of chars as substring
        let seq_slice_end = i+&substr.len();

        // avoid overindexing
        if seq_slice_end < seq.len() {
            let seq_slice = &seq[i .. seq_slice_end];
            
            // check if match and add index to vector if true
            if seq_slice == substr {
                substr_indices.push(i + 1)
            };
        }
        
    }
    
    //https://users.rust-lang.org/t/how-do-i-convert-vec-of-i32-to-string/18669/8
    let str_substr_indices: Vec<String> = substr_indices
        .into_iter() // convert to iterator
        .map(|d| d.to_string())// apply to_string to each digit converting digit to string representation
        .collect();  // return to iterator

    // join the itr of strings by a space and return
    return str_substr_indices.join(" ");
}