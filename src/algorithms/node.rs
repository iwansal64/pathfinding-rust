use std::f32::INFINITY;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub previous_node: String,
    pub explored: bool,
    pub cost: f32,
    pub connected_nodes_name: HashMap<String, f32>
}

impl Node {
    pub fn new(name:&str, connected_nodes_name:HashMap<&str, f32>) -> Self {
        let mut connected_nodes_name_string:HashMap<String, f32> = HashMap::new();
        for (name, cost) in connected_nodes_name {
            connected_nodes_name_string.insert(name.to_string(), cost);
        }
        
        Self {
            name: name.to_string(),
            previous_node: "".to_string(),
            explored: false,
            cost: INFINITY,
            connected_nodes_name: connected_nodes_name_string
        }
    }

    pub fn clone(&self) -> Node {
        Node {
            connected_nodes_name: self.connected_nodes_name.clone(),
            name: self.name.clone(),
            previous_node: self.previous_node.clone(),
            explored: self.explored.clone(),
            cost: self.cost.clone()
        }
    }
}
