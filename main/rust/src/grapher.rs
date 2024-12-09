//converts graph from a person to a hashmap

use std::collections::{HashMap, HashSet};
use std::fs::File;
use csv::ReaderBuilder;

pub fn commenter_and_channels(path: &str) -> HashMap<String, HashSet<String>>{
    let file = File::open(path).expect("File not found");
    let mut reader = csv::Reader::from_reader(file);

    // HashMap to store author_name -> set of channel_name
    let mut author_map: HashMap<String, HashSet<String>> = HashMap::new();

    // Iterate over each row in the CSV
    for result in reader.records() {
        let record = result.unwrap();
        let author_name = record.get(5).unwrap_or("example").to_string(); // 5th column: author_name
        let channel_name = record.get(0).unwrap_or("").to_string(); // 0th column: channel_name

        if author_name.trim().is_empty() {
            continue;
        }

        // Insert into the HashMap
        author_map.entry(author_name)
            .or_insert_with(HashSet::new)
            .insert(channel_name);
    }

    author_map
}

pub fn channel_and_commenters(file_path: &str) -> HashMap<String, HashSet<String>> {
    let file = File::open(file_path).expect("File not found");
    let mut reader = csv::Reader::from_reader(file);

    // Map to store channel_name -> set of author_names
    let mut channel_to_authors: HashMap<String, HashSet<String>> = HashMap::new();

    for result in reader.records() {
        let record = result.unwrap();
        let channel_name = record.get(0).unwrap_or("").to_string(); // 0th column: channel_name
        let author_name = record.get(5).unwrap_or("").to_string(); // 5th column: author_name

        if !channel_name.is_empty() && !author_name.is_empty() {
            channel_to_authors
                .entry(channel_name)
                .or_insert_with(HashSet::new)
                .insert(author_name);
        }
    }

    channel_to_authors
}

// https://doc.rust-lang.org/std/keyword.where.html, Wanted to make a simple clean function to convert hashmap to vector
pub fn hashmap_to_vec<K, V>(map: HashMap<K, HashSet<V>>) -> HashMap<K, Vec<V>>
where
    K: Eq + std::hash::Hash,
    V: Eq + std::hash::Hash + Clone,
{
    map.into_iter()
        .map(|(key, set)| (key, set.into_iter().collect()))
        .collect()
} 

// Implement graph that is of comment threads
pub fn comment_threads_neighbors(file_path: &str) -> HashMap<String, Vec<String>> {
    let file = File::open(file_path).expect("File not found");
    let mut reader = csv::Reader::from_reader(file);

    // Map to store -> set of author_names
    let mut threads: HashMap<String, HashSet<String>> = HashMap::new();

    for result in reader.records() {
        let record = result.unwrap();

        let thread_id = record.get(3).unwrap_or("").to_string(); // 5th column: author_name
        let author_name = record.get(5).unwrap_or("").to_string(); // 5th column: author_name

        if !author_name.is_empty() && !thread_id.is_empty() {
            threads
                .entry(thread_id)
                .or_insert_with(HashSet::new)
                .insert(author_name);
        }

    }
    let mut neighbor_map: HashMap<String, Vec<String>> = HashMap::new();
    for commenters in threads.values(){
        for (i, commenter) in commenters.iter().enumerate() {
            let mut neighbors = Vec::with_capacity(commenters.len() - 1);
            neighbors.extend(
                commenters.iter()
                    .take(i) // All commenters before the current one
                    .chain(commenters.iter().skip(i + 1)) // All commenters after the current one
                    .map(|c| c.to_string()),
            );
    
            neighbor_map.insert(commenter.to_string(), neighbors);
        }
    
    }
    neighbor_map

}