use std::collections::{HashMap, HashSet};
use rand::Rng;
use std::collections::VecDeque;

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


// Page Rank functions
fn random_key(map: &HashMap<String, Vec<String>>) -> &String
{
    let mut rng = rand::thread_rng();
    let keys: Vec<&String> = map.keys().collect();
    let index = rng.gen_range(0..keys.len());
    keys[index]
}

fn step(vertex: String, map: &HashMap<String, Vec<String>>, rng: &mut rand::rngs::ThreadRng) -> String {
// For Page rank function
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

#[test]
fn test_pagerank_sum() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("John".to_string(), vec!["Steve".to_string(), "Bob".to_string()]);
    graph.insert("Bob".to_string(), vec!["John".to_string()]);
    graph.insert("Bill".to_string(), vec!["Steve".to_string(), "Joe".to_string()]);
    graph.insert("Joe".to_string(), vec!["Bob".to_string(), "Bill".to_string(), "John".to_string()]);

    let n = graph.len();
    let ranking_map = page_rank(&graph, n);
    let sum: f64 = ranking_map.values().sum();
    let tolerance = 1e-6;
    assert!((sum - 1.0).abs() < tolerance, "Sum of PageRank values is not close to 1.0");
}

#[test]
fn test_pagerank_ranking() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("Mr.Popular".to_string(), vec![]);
    graph.insert("Bob".to_string(), vec!["Mr.Popular".to_string()]);
    graph.insert("Bill".to_string(), vec!["Mr.Popular".to_string()]);
    graph.insert("Joe".to_string(), vec!["Mr.Popular".to_string(), "John".to_string()]);
    graph.insert("Steve".to_string(), vec!["Mr.Popular".to_string()]);
    graph.insert("Al".to_string(), vec!["Mr.Popular".to_string()]);
    graph.insert("Frank".to_string(), vec!["Mr.Popular".to_string()]);
    graph.insert("Hank".to_string(), vec!["Mr.Popular".to_string()]);


    let ranking_map = page_rank(&graph, 100);

    let mut ranking_vec: Vec<(&String, &f64)> = ranking_map.iter().collect();
    ranking_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("{:?}", ranking_vec);

    assert!(ranking_vec[0].0 == "Mr.Popular", "Mr.Popular is not the most popular with page rank.");
}

//BFS implementation
pub fn bfs(start: &str, graph: &HashMap<String, Vec<String>>) -> HashMap<String, u32>{
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start.to_string(), 0);
    queue.push_back(start.to_string());

    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];

        if let Some(neighbors) = graph.get(&current) { // Check if neighbors exist
            for neighbor in neighbors {
                if !distances.contains_key(neighbor) {
                    distances.insert(neighbor.clone(), current_distance + 1);
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    distances
}

#[test]
fn test_bfs(){
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    graph.insert("John".to_string(), vec!["Steve".to_string(), "Bob".to_string()]);
    graph.insert("Steve".to_string(), vec!["Bill".to_string()]);
    let test_bfs = bfs("John", &graph);
    let mut correct_bfs: HashMap<String, u32> = HashMap::new();
    correct_bfs.insert("John".to_string(), 0);
    correct_bfs.insert("Steve".to_string(), 1);
    correct_bfs.insert("Bob".to_string(), 1);
    correct_bfs.insert("Bill".to_string(), 2);


    println!("{:?}", test_bfs);
    assert!(test_bfs == correct_bfs, "BFS distances calculated incorrectly.");

}

struct ChannelCounter{
    sum: f32,
    count: u32
}

impl ChannelCounter {
    fn average(&self) -> f32{
        self.sum / self.count as f32
    }
    fn update_values(&mut self, distance: &u32){
        self.sum += *distance as f32;
        self.count += 1;
    }
}

pub fn channel_commentor_speration(bfs_vec: &Vec<HashMap<String, u32>>, channel_map: &HashMap<String, HashSet<String>>){
    
    let mut channel_distances: HashMap<String, ChannelCounter> = HashMap::new();

    for bfs in bfs_vec {
        for (key, distance) in bfs.iter(){
            let channels = channel_map.get(key).unwrap();
            for channel in channels.iter(){
                channel_distances.entry(channel.clone())
                .or_insert_with(|| ChannelCounter {
                    sum: 0.0,
                    count: 0,
                })
                .update_values(distance);
            }
        }
    }
    let mut distance_vec = Vec::new();
    
    for (channel, distance) in channel_distances.iter(){
        let average_distance = distance.average();
        distance_vec.push((channel, average_distance));
    }
    distance_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("In Descending Order, the average seperation for commentors on that channel is:");
    for (channel, distance) in distance_vec{
        println!("{:?}: {}", channel, distance);
    }
}
