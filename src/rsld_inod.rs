use std::fs;

pub fn inner_nodes(fname: &str) -> usize {
    let contents = fs::read_to_string(fname).expect("No file.");
    let n_leaves: usize = contents.trim().parse().unwrap();
    // lmao -2
    n_leaves - 2

    // let mut inodes: Vec<usize> = vec![];
    // let mut starting_leaves = n_leaves;
    // loop {
    //     if starting_leaves <= 2 {
    //         break;
    //     } else {
    //         starting_leaves = starting_leaves / 2;
    //         let remainder =  n_leaves % 2;
    //         println!("{starting_leaves} - {remainder}");
    //         inodes.push(starting_leaves);
    //         inodes.push(remainder);
    //     }
    // }
    // inodes.iter().sum()
}
