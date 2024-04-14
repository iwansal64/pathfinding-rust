pub mod models;
pub mod algorithms;
pub mod worlds;

use std::collections::HashMap;

use algorithms::djikstra::DjikstraAlgorithm;
use algorithms::node::Node;

fn main() {
    let nodes: HashMap<String, Node> = HashMap::<String, Node>::from([
       (String::from("A"), Node::new("A", HashMap::from([
            ("B", 2 as f32),
            ("C", 4 as f32)
        ]))),
        (String::from("B"), Node::new("B", HashMap::from([
            ("D", 3 as f32),
            ("C", 3 as f32)
        ]))),
        (String::from("C"), Node::new("C", HashMap::from([
            ("D", 4 as f32),
            ("E", 3 as f32)
        ]))),
        (String::from("D"), Node::new("D", HashMap::from([
            ("F", 3 as f32)
        ]))),
        (String::from("E"), Node::new("E", HashMap::from([
            ("F", 2 as f32)
        ]))),
        (String::from("F"), Node::new("F", HashMap::from([])))
    ]);

    /* 

    (A) - 2 - (B) -- 3 -- (D) -- 3 --- +
     \         \                       | 
      \         3                     (F)
       \         \                     |
        4 ------ (C) -- 3 --- (E) - 2 -+
    
    */


    let mut algo: DjikstraAlgorithm = DjikstraAlgorithm::new(nodes);
    match algo.start_exploring("A".to_string()) {
        Ok(msg) => println!("{msg}"),
        Err(msg) => println!("{msg}")
    }
    println!("{:?}", algo.nodes);
}