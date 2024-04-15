use std::collections::HashMap;

use crate::algorithms::node::{Nodes, Node};

pub struct World {
    pub nodes: Nodes,
    pub double_check: bool
}

impl World {
    pub fn from_nodes(nodes: Nodes) -> Self {
        Self {
            nodes,
            double_check: true
        }
    }

    /// Format (ignore whitespaces) -> A-B:2,A-C:4,B-D:3,B-C:3,C-E:3,E-F:2,D-F:3
    pub fn from_text(text: &str) -> Self {
        let mut nodes: Nodes = Nodes::new();

        //? "A-B:2,A-C:4" -> ["A-B:2", "A-C:4"]
        for connection_information in text.to_string().split(",").into_iter() {
            
            //? "A-B:2" -> ["A-B", "2"]
            let splitted_information: Vec<&str> = connection_information.split(":").collect();
            
            //? "A-B" -> ["A", "B"]
            let nodes_name: Vec<&str> = splitted_information[0].split("-").collect();
            
            //? "2" -> 2.0
            let cost: f32 = splitted_information[1].to_string().parse().unwrap();

            for index in 0..=1 {
                //? Node(A|B) -> nodes (Add the current node (Will be "A" and "B") to nodes variable)
                
                //? index = 0, anti_index = 1 & index = 1, anti_index = 0
                let anti_index: usize = if index == 0 { 1 } else { 0 };
                if !nodes.contains_key(nodes_name[index]) {
                    let node: Node = Node::new(nodes_name[index], HashMap::from([
                        (nodes_name[anti_index], cost)
                    ]));
                    nodes.insert(nodes_name[index].to_string(), node);
                }
                else {
                    match nodes.get_mut(nodes_name[index]) {
                        Some(node) => {
                            node.connected_nodes_name.insert(nodes_name[anti_index].to_string(), cost);
                        },
                        None => ()
                    }
                }
            }
        }

        Self {
            nodes,
            double_check: false
        }
    }

    /// From Simple HashMap -> {"A-B": 2, "A-C": 4, "B-C": 3, "B-D": 3, "C-D": 4, "C-E": 3, "D-F": 3, "E-F": 2}
    pub fn from_sh(simple_hashmap: HashMap<&str, f32>) -> Self {
        let mut nodes: Nodes = Nodes::new();

        //? {"A-B": 2, "A-C": 4} -> ["A-B", "A-C"]
        for nodes_name_key in simple_hashmap.keys() {
            if let Some(cost) = simple_hashmap.get(nodes_name_key) {
                let nodes_name: Vec<&str> = nodes_name_key.split("-").collect();
                for index in 0..=1 {
                    //? Node(A|B) -> nodes (Add the current node (Will be "A" and "B") to nodes variable)
                    
                    //? index = 0, anti_index = 1 & index = 1, anti_index = 0
                    let anti_index: usize = if index == 0 { 1 } else { 0 };
                    if !nodes.contains_key(nodes_name[index]) {
                        let node: Node = Node::new(nodes_name[index], HashMap::from([
                            (nodes_name[anti_index], cost.clone())
                        ]));
                        nodes.insert(nodes_name[index].to_string(), node);
                    }
                    else {
                        match nodes.get_mut(nodes_name[index]) {
                            Some(node) => {
                                node.connected_nodes_name.insert(nodes_name[anti_index].to_string(), cost.clone());
                            },
                            None => ()
                        }
                    }
                }

            }
        }

        Self {
            nodes,
            double_check: false
        }
    }
}