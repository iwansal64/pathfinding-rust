pub mod models;
pub mod algorithms;
pub mod worlds;

use std::collections::HashMap;

use algorithms::djikstra::DjikstraAlgorithm;
use worlds::World;

fn main() {
    // let nodes: Nodes = HashMap::from([
    //    (String::from("A"), Node::new("A", HashMap::from([
    //         ("B", 2 as f32),
    //         ("C", 4 as f32)
    //     ]))),
    //     (String::from("B"), Node::new("B", HashMap::from([
    //         ("D", 3 as f32),
    //         ("C", 3 as f32)
    //     ]))),
    //     (String::from("C"), Node::new("C", HashMap::from([
    //         ("D", 4 as f32),
    //         ("E", 3 as f32)
    //     ]))),
    //     (String::from("D"), Node::new("D", HashMap::from([
    //         ("F", 3 as f32)
    //     ]))),
    //     (String::from("E"), Node::new("E", HashMap::from([
    //         ("F", 2 as f32)
    //     ]))),
    //     (String::from("F"), Node::new("F", HashMap::from([])))
    // ]);

    // let world: World = World::from_text("A-B:2,A-C:4,B-D:3,B-C:3,C-E:3,E-F:2,D-F:3");
    let world: World = World::from_sh(HashMap::from([
            ("A-B", 2 as f32),
            ("A-C", 4 as f32),
            ("B-C", 3 as f32),
            ("B-D", 3 as f32),
            ("C-D", 4 as f32),
            ("C-E", 3 as f32),
            ("D-F", 3 as f32),
            ("E-F", 2 as f32)
    ]));

    /* 

    (A) - 2 - (B) -- 3 -- (D) -- 3 --- +
     \         \                       | 
      \         3                     (F)
       \         \                     |
        4 ------ (C) -- 3 --- (E) - 2 -+
    
    */


    let mut algo: DjikstraAlgorithm = DjikstraAlgorithm::new(world);
    println!();
    match algo.start_exploring("A".to_string()) {
        Ok(msg) => println!("{msg}"),
        Err(msg) => println!("{msg}")
    }
    let path_traces: Vec<String> = algo.find_path("F".to_string()).unwrap();
    let path_cost: f32 = algo.get_cost("F".to_string()).unwrap();

    if let Some(last_trace) = path_traces.last() {
        let last_trace = last_trace.clone();
        for trace in path_traces {
            print!("{trace}");
            if trace != last_trace {
                print!(" <- ");
            }
            else {
                println!();
            }
        }
        println!("\nPath Cost : {path_cost}");
    }
}