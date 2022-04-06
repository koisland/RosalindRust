pub mod rs_utils {
    use std::{fs::File, collections::HashMap};
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

    pub fn unpack_fasta() {
        
    }
}