use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day12").unwrap();

    let mut graph_map: HashMap<String, Node> = HashMap::new();

    for line in input.trim().split("\n"){
        let node_strs: Vec<&str> = line.trim().split("-").collect();

        if graph_map.contains_key(node_strs[0]){
            graph_map.get_mut(node_strs[0]).unwrap().children.insert(0, node_strs[1].to_string());
        } else {
            graph_map.insert(node_strs[0].to_string(), Node {name: node_strs[0].to_string(), children: vec![node_strs[1].to_string()]});
        }

        if graph_map.contains_key(node_strs[1]){
            graph_map.get_mut(node_strs[1]).unwrap().children.insert(0, node_strs[0].to_string());
        } else {
            graph_map.insert(node_strs[1].to_string(), Node {name: node_strs[1].to_string(), children: vec![node_strs[0].to_string()]});
        }
    }

    //println!("{:?}", graph_map.keys().map(|k| format!("{} -> {:?}", graph_map[k].name, graph_map[k].children)).collect::<Vec<String>>());

    let ref start = graph_map["start"];
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert(&start.name);

    let paths = visit(start, visited, &graph_map, false);
    println!("path count: {}", paths);
}

fn visit(start: &Node, visited: HashSet<&str>, graph: &HashMap<String, Node>, double_dipped: bool) -> i32{
    if start.name == "end"{
        println!("visited: {:?}", visited);
        return 1;
    }

    let mut paths = 0;
    let mut visited_new = HashSet::from(visited);
    visited_new.insert(&start.name);

    for child in start.children.iter(){
        let child_node = &graph[child];
        if child == "start"{
            continue;
        }
        if visited_new.contains(&*child_node.name) && !child_node.IsBig(){
            if !double_dipped {
                paths += visit(child_node, visited_new.clone(), graph, true);
            } else {
                continue;
            }
        } else {
            paths += visit(child_node, visited_new.clone(), graph, double_dipped);
        }
    }

    return paths;
}

struct Node{
    name: String,
    children: Vec<String>,
}

impl Node {
    fn IsBig(&self) -> bool{
        return self.name.chars().collect::<Vec<char>>()[0].is_uppercase();
    }
}
