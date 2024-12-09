use std::collections::{HashMap, HashSet};
use rand::Rng;

pub fn most_neighbors(node_map: &HashMap<String, HashSet<String>>) -> String{
    let mut max_node = None;
    let mut max_edges = 0;

    for (node, edges) in node_map {
        if edges.len() > max_edges {
            max_node = Some(node.clone());
            max_edges = edges.len();
        }
    }
    max_node.expect("Unable to find max node")
}

fn random_key(map: &HashMap<String, Vec<String>>) -> &String
//For page rank function
{
    let mut rng = rand::thread_rng();
    let keys: Vec<&String> = map.keys().collect();
    let index = rng.gen_range(0..keys.len());
    keys[index]
}

fn step(vertex: String, map: &HashMap<String, Vec<String>>, rng: &mut rand::rngs::ThreadRng) -> String {
    if let Some(edges) = map.get(&vertex) {
        let random_number = rng.gen_range(1..=10);
        if random_number <= 9 && edges.len() > 1{
            let index = rng.gen_range(0..edges.len());
            edges[index].clone() // Pick Random Neighbor
        } else {
            random_key(map).clone() //Jump to random Vertex
        }
    } else {
        random_key(map).clone() // Jump to a random vertex
    }
}

pub fn page_rank(map: &HashMap<String, Vec<String>>, n: usize) -> HashMap<String, f64> {
    let mut rng = rand::thread_rng();
    let mut ranking_map: HashMap<String, f64> = HashMap::new();

    for _ in 0..n {
        let start_vertex = random_key(map).clone(); // Pick a random start node
        let mut vertex = start_vertex.clone();
        for _ in 0..80 {
            vertex = step(vertex, map, &mut rng);

            let count = ranking_map.entry(vertex.clone()).or_insert(0.0);
            *count += 1.0;
        }
    }
    let total_walks = (n * 80) as f64;
    for value in ranking_map.values_mut() {
        *value /= total_walks;
    }
    
    ranking_map
}
