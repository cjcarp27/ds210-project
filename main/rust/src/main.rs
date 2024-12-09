mod grapher;
mod analyser;

fn main() {

//Select the wanted analysis
// 1-> relationship between channels using breadth first search on the person_graph
//.    output the person who visited the most channels, output neighbor information: the person with the most neighbors, output the farthest neigbor
//     output top 5 page rank
//
// Open the CSV file
println!("=========DS210 PROJECT=========");
// let commenter_and_channels = grapher::commenter_and_channels("comments.csv");
// let channel_comments = grapher::channel_and_commenters("comments.csv");
let neighbors = grapher::comment_threads_neighbors("comments.csv");

let ranking_map = analyser::page_rank(&neighbors, 10);

let mut ranking_vec: Vec<(&String, &f64)> = ranking_map.iter().collect();
ranking_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

let top_30: Vec<_> = ranking_vec.into_iter().take(30).collect();

println!("Using Page Rank Algorithm to find the most influential Commentor");
for (vertex, rank) in top_30 {
    println!("Author: {}, Approximate Page Rank: {}", vertex, rank);
}


// println!("=========Commentor Analysis=========");

// let commenter_channel_top_node = analyser::most_neighbors(&commenter_and_channels);
// let commenter_channel_top_neighbors = commenter_and_channels.get(&commenter_channel_top_node).unwrap();
// println!("Commentor with the most comments is: {}\nChannels commented on are: {:?}\n", commenter_channel_top_node, commenter_channel_top_neighbors);

// println!("=========Channel Analysis=========");

// let channel_top_node = analyser::most_neighbors(&channel_comments);
// let channel_num_of_top_neighbors = channel_comments.get(&channel_top_node).expect("Channel ID not inputted").len();
// println!("Channel with the most unique comments is: {}\nNumber of Unique Commentors are: {}", channel_top_node, channel_num_of_top_neighbors);

// let channel_comments_vec = grapher::hashmap_to_vec(channel_comments);


// let ranking_map = analyser::page_rank(&channel_comments_vec, 10);

// let mut ranking_vec: Vec<(&String, &f64)> = ranking_map.iter().collect();
// ranking_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

// let top_30: Vec<_> = ranking_vec.into_iter().take(30).collect();

// println!("Using Page Rank Algorithm to find the most influential ");
// for (vertex, rank) in top_30 {
//     println!("Author: {}, Approximate Page Rank: {}", vertex, rank);
// }

}
