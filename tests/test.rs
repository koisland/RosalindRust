#[cfg(test)]
mod tests {
    use rosalind_rust::rs_utils::{rna_codons, unpack_fasta};

    #[test]
    fn test_unpack() {
        let test_seqs_path = "data/test_seqs.txt".to_string();
        let seqs = unpack_fasta(&test_seqs_path);

        assert_eq!(seqs.keys().len(), 3 as usize);
    }

    #[test]
    fn test_rna_codons() {
        let codons = rna_codons();

        assert_eq!(codons.keys().len(), 64 as usize);
    }
}
