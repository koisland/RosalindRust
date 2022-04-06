use std::fs;

pub fn replace_u(fname: &String) -> String {
    let contents = fs::read_to_string(fname)
        .expect("Unable to read file.");
    
    let contents_rna = contents.replace("T", "U"); 
    println!("{}", contents_rna);

    return contents_rna;
}