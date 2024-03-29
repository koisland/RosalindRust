pub mod rs_utils {
    use csv::Reader;
    use itertools::Itertools;
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn hamm_dst(seq_1: &str, seq_2: &str) -> usize {
        let mut h_dst: usize = 0;

        for (char_1, char_2) in seq_1.chars().zip_eq(seq_2.chars()) {
            if char_1 != char_2 {
                h_dst += 1
            }
        }
        h_dst
    }

    pub fn rev_comp(seq: &str) -> String {
        let rev_seq: String = seq
            .replace("A", "t")
            .replace("T", "a")
            .replace("G", "c")
            .replace("C", "g")
            .to_uppercase()
            .chars()
            .rev()
            .collect();
        rev_seq
    }

    pub fn rna_codons() -> HashMap<String, String> {
        let file = File::open("data/rna_tbl.csv").expect("Unable to read file.");

        let mut rna_tbl = HashMap::new();

        let mut rdr = Reader::from_reader(file);

        for res in rdr.records() {
            let rec = res.expect("Invalid record.");
            // First is codon, second is protein
            rna_tbl.insert(rec[0].to_string(), rec[1].to_string());
        }

        return rna_tbl;
    }

    pub fn unpack_fasta(fname: &str) -> HashMap<String, String> {
        let seqs_file = File::open(&fname).expect("Unable to read seqs file.");
        let seq_lines = BufReader::new(seqs_file).lines();

        let mut seqs = HashMap::new();

        let mut curr_seq_id = "".to_string();

        for (i, ln) in seq_lines.enumerate() {
            let err_msg = format!("Line {} cannot be read", i.to_string());
            let ln = ln.expect(&err_msg);

            // check if first char of line is >
            if ln.chars().nth(0).unwrap() == '>' {
                // get seq id and set as main seq
                curr_seq_id = ln[1..].trim().to_string();

                // init curr_seq_id with empty string if not exists.
                seqs.entry(curr_seq_id.to_string())
                    .or_insert("".to_string());
            } else {
                // get current seq copy
                let mut curr_seq = seqs.get(&curr_seq_id).expect("Seq not found.").to_string();
                // add ln and reinsert over previous seq in curr_seq_id
                curr_seq.push_str(&ln);
                seqs.insert(curr_seq_id.to_string(), curr_seq.to_string());
            }
        }

        return seqs;
    }
}
