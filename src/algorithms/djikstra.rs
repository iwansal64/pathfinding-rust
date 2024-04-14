use std::{collections::HashMap, f32::INFINITY};

use crate::algorithms::node::Node;

pub struct DjikstraAlgorithm {
    pub nodes: HashMap<String, Node>,
    current_node: String
}

impl DjikstraAlgorithm {
    pub fn new(nodes: HashMap<String, Node>) -> Self {
        Self {
            nodes,
            current_node: String::new()
        }
    }

    ///
    fn update_cost(&mut self, target_node: &mut Node, connected_node_name: &String, connection_cost: &f32) -> Result<bool, String> {
        let mut changes: bool = false;
        // Check if there's a node that connected to the 'nodes' variable
        if let Some(connected_node) = self.nodes.get_mut(connected_node_name) {
            let cost_result = connection_cost.clone() + target_node.cost;

            //? Check if the result cost is less than the connected node cost of target node.
            if cost_result < connected_node.cost {
                //? If it is, replace the cost as well as the previous node with the current node.
                connected_node.cost = cost_result;
                connected_node.previous_node = target_node.name.clone();
                changes = true;
            }
        }
        else {
            // If there isn't it'll be user mistakes
            return Err(format!("ERROR : connected node name error! Line : {}", line!()));
        }

        Ok(changes)
    }
    
    /// Explore The Nodes That Are Connected To Target Node
    pub fn explore(&mut self, target_node_name: String) -> Result<bool, String> {
        if !self.nodes.contains_key(&target_node_name) {
            return Err(format!("Check you nodes name again.. node name : {}, nodes avaliable : {:?} Line : {}", target_node_name, self.nodes.keys(), line!()));
        }
        
        let mut changes = false;
        let mut node = self.nodes.get_mut(&target_node_name).unwrap().clone();

        //? Get nodes name that are connected to target node.
        for (connected_node_name, connection_cost) in &node.clone().connected_nodes_name {
            //? If the connected node is the previous node, skips.. (because it's useless)
            if connected_node_name == &node.previous_node {
                continue;
            }

            match self.update_cost(&mut node, connected_node_name, connection_cost) {
                Ok(changes_res) => {
                    if !changes {
                        changes = changes_res; 
                    }
                },
                Err(msg) => return Err(msg)
            }
        }
        
        let mut update_nodes_data: Vec<(String, f32)> = Vec::new();
        
        for (node_name, node_) in &self.nodes {
            if *node_name == node.name {
                continue;
            }
            
            if node_.connected_nodes_name.contains_key(&target_node_name) {
                match node_.connected_nodes_name.get(&target_node_name) {
                    Some(connection_cost) => {
                        update_nodes_data.push((node_name.clone(), connection_cost.clone()))
                    },
                    None => ()
                }
            }
        }

        for update_node_data in update_nodes_data {
            match self.update_cost(&mut node, &update_node_data.0, &update_node_data.1) {
                Ok(changes_res) => {
                    if !changes {
                        changes = changes_res; 
                    }
                },
                Err(msg) => return Err(msg)
            }
        }

        Ok(changes)
    }

    /// Go To Another Node (Change Current Node)
    fn goto(&mut self, target_node: String) {
        //? Move from current node to target node
        self.current_node = target_node;
        
        match self.nodes.get_mut(&self.current_node) {
            Some(node) => {
                //? Flags the current node explored
                node.explored = true;
            },
            None => ()
        }
    }

    /// Search For The Lowest Cost Node For The Next Current Node
    fn search(&self) -> String {
        let mut lowest_cost: f32 = INFINITY;
        let mut lowest_node_name: String = String::new();

        for (node_name, node) in &self.nodes {
            if node.explored { 
                continue; 
            }

            if node_name == &self.current_node {
                continue;
            }

            if node.cost < lowest_cost {
                lowest_cost = node.cost;
                lowest_node_name = node_name.clone();
            }
        }

        //? Return lowest cost node
        lowest_node_name
    }

    /// Start The Algorithm! ⭐ . You have to insert where you want to go or where are you at. and return the result
    pub fn start_exploring(&mut self, target: String) -> Result<String, String> {
        self.current_node = target.clone();
        match self.nodes.get_mut(&self.current_node) {
            Some(node) => {
                node.cost = 0 as f32;
                node.explored = true;
            },
            None => {
                return Err("Node {target} is not recognized.".to_string());
            }
        }

        let mut max_iter = 50;
        while max_iter > 0 {
            if let Err(res) = self.explore(self.current_node.clone()) {
                return Err(format!("There's an error when exploring : {}", res));
            }
            let next_node: String = self.search();
            if next_node == "" {
                break;
            }
            self.goto(next_node);
            println!("{}", self.current_node);
            max_iter -= 1;
        }

        Ok("The algorithm is done successfully!".to_string())
    }
}