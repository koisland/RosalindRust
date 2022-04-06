use std::fs;

pub fn hamm_dst(fname: &String) -> usize {
    let contents = fs::read_to_string(fname)
        .expect("Unable to read file.");

    
    let split_contents: Vec<&str> = contents
        .trim()
        .split("\n")
        .collect();
    
    // needs 2nd trim or length of strings are uneven.
    let seq_1_len = split_contents[0].trim().len();   

    let mut h_dst: usize = 0;
    for i in 0..seq_1_len {
        // Strings in rust are utf-8 and are of variable length. Need to traverse thru entire
        // string to get length. O(N)
        if split_contents[0].chars().nth(i).unwrap() != split_contents[1].chars().nth(i).unwrap() {
            h_dst += 1;
        }
    }
    
    println!("{}", h_dst);
    return h_dst;
}