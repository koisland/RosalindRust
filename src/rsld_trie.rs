use std::{
    cell::RefCell,
    rc::{Rc, Weak}, fs,
};

#[derive(Debug)]
pub struct Node {
    pub n: usize,
    pub out_edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct Edge {
    pub val: char,
    pub parent: Weak<RefCell<Node>>,
    pub child: Weak<RefCell<Node>>,
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (Some(parent), Some(child)) = (self.parent.upgrade(), self.child.upgrade()) {
            write!(f, "{} {} {}", parent.borrow().n, child.borrow().n, self.val)
        } else {
            write!(f, "{}", self.val)
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    curr_node: Weak<RefCell<Node>>,
    nodes: Vec<Rc<RefCell<Node>>>
}

impl Trie {
    pub fn new() -> Trie {
        let root = Rc::new(RefCell::new(Node {
            n: 1,
            out_edges: vec![],
        }));
        let curr_node = Rc::downgrade(&root);

        Trie { curr_node, nodes: vec![root]}
    }

    pub fn add_word(&mut self, word: &str) -> &mut Self {
        for letter in word.chars() {
            self.add_char(letter);
        }

        // Set curr node back to root after adding word.
        if let Some(root_node) = self.nodes.first() {
            self.curr_node = Rc::downgrade(&root_node);
        }

        self
    }

    pub fn add_char(&mut self, val: char) -> &mut Self {
        // Get current node.
        let curr_node = self.curr_node.upgrade().unwrap();

        // If the current node has an edge than contains the value, get it.
        if let Some(child_edge) = curr_node.borrow().out_edges.iter().find(|edge| edge.val == val) {
            // And set curr node to edge's child node.
            // No need to add new edge.
            self.curr_node = child_edge.child.clone();
            return self;
        }

        // And add new node.
        let new_node = Rc::new(RefCell::new(Node {
            n: self.nodes.len() + 1,
            out_edges: vec![],
        }));
        // Add new edge from current node to new node.
        curr_node.borrow_mut().out_edges.push(Edge {
            val,
            parent: Rc::downgrade(&curr_node),
            child: Rc::downgrade(&new_node),
        });
        self.curr_node = Rc::downgrade(&new_node);

        self.nodes.push(new_node);

        self
    }

    pub fn print_adj_list(&self) {
        for node in self.nodes.iter() {
            for edge in node.borrow().out_edges.iter() {
                println!("{edge}");
            }
        }
    } 
}

pub fn build_trie(file: &str) {
    let seqs = fs::read_to_string(file).unwrap();

    let mut trie = Trie::new();

    for seq in seqs.lines() {
        trie.add_word(seq);
    }

    trie.print_adj_list();
}
