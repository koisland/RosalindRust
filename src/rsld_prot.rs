pub mod rs_prot {
    use std::{fs::read_to_string, fs::File, collections::{HashMap}};
    use csv::Reader;

    pub fn rna_codons() -> HashMap<String, String> {
        let file = File::open("data/rna_tbl.csv")
            .expect("Unable to read file.");

        let mut rna_tbl = HashMap::new();

        let mut rdr = Reader::from_reader(file);

        for res in rdr.records() {
            let rec = res.expect("Invalid record.");
            // First is codon, second is protein
            rna_tbl.insert(rec[0].to_string(), rec[1].to_string());
        }

        return rna_tbl;
    }

    pub fn translate(fname: &String) -> String {
        let contents = read_to_string(fname)
            .expect("Unable to read rsld prompt.");

        let contents = contents.trim();

        // hashmap of codon string: protein or stop
        let res_translation = rna_codons();

        let mut prot_contents = String::new();

        let contents_len = contents.chars().count();

        for (i, _) in contents.chars().enumerate() {
            // Skip any nt that is not the 3rd nt.
            if i % 3 != 0 {
                continue;
            }
            // break from loop after last nt in contents
            if i + 3 > contents_len {
                break;
            }
            let codon = &contents[i..i+3];

            let protein = res_translation
                .get(&codon.to_string())
                .expect("Invalid codon.");
            
            // need to match string slice "Stop"
            match protein.as_str() {
                "Stop" => break,
                // append first character of protein to prot_contents
                _ => prot_contents.push(protein.chars()
                        .nth(0)
                        .expect("Unexpected character."))
            }
        }
        
        // println!("{}", prot_contents);
        return prot_contents;
        

    }
}