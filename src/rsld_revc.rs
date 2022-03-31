pub mod rs_revc {
    use std::fs;

    pub fn rev_comp(fname: &String) -> String {
        let contents = fs::read_to_string(fname)
            .expect("Unable to read file.");
        
        let rev_contents = contents
            .replace("A", "t")
            .replace("T", "a")
            .replace("G", "c")
            .replace("C", "g")
            .to_uppercase()
            .chars()
            .rev()
            .collect();

        println!("{}", rev_contents);

        return rev_contents;
    }
}