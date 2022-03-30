
pub mod rs_dna {
    /// std::fs (file system) to read files.
    use std::fs;

    pub fn cnt_nt(fname: &String) {
        let contents = fs::read_to_string(fname)
            .expect("Unable to read file.");

        println!("{}", contents);
    }
}
