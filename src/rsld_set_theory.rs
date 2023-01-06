use std::{fs, error::Error, collections::HashSet};

use itertools::Itertools;



pub fn set_theory(fname: &str) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(fname)?;

    let (n, set_a, set_b) = contents.lines().collect_tuple().unwrap();

    let set_n: HashSet<usize> = (1..=n.parse::<usize>().unwrap()).collect();

    let set_a: HashSet<usize> = set_a
        .trim_matches(|c| c == '{' || c == '}' || c == '\n')
        .split(',')
        .map(|val| val.trim().parse::<usize>().unwrap())
        .collect();
    
    let set_b: HashSet<usize> = set_b
        .trim_matches(|c| c == '{' || c == '}' || c == '\n')
        .split(',')
        .map(|val| val.trim().parse::<usize>().unwrap())
        .collect();

    // A∪B, 
    let a_b_union: HashSet<&usize> = set_a.union(&set_b).collect();
    
    // A∩B
    let a_b_intersection: HashSet<&usize> = set_a.intersection(&set_b).collect();
    
    // A−B 
    let a_diff_b: HashSet<&usize> = set_a.difference(&set_b).collect();

    // B−A
    let b_diff_a: HashSet<&usize> = set_b.difference(&set_a).collect();

    // Ac with respect to n
    let a_c: HashSet<&usize> = set_n.difference(&set_a).collect();
    
    // Bc with respect to n
    let b_c: HashSet<&usize> = set_n.difference(&set_b).collect();

    println!("{:?}", a_b_union);
    println!("{:?}", a_b_intersection);
    println!("{:?}", a_diff_b);
    println!("{:?}", b_diff_a);
    println!("{:?}", a_c);
    println!("{:?}", b_c);

    Ok(())
}