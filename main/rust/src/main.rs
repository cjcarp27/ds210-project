mod grapher;
mod analyser;

fn main() {
    println!("====Youtube Comment Analysis====");
    println!("Processing Graphs\n\n");
    let commenter_and_channels = grapher::commenter_and_channels("comments.csv");
    let channel_comments = grapher::channel_and_commenters("comments.csv");
    let comment_thread_neighbors = grapher::comment_threads_neighbors("comments.csv");

    println!("========Simple Analysis========");

    let commenter_channel_top_node = analyser::most_neighbors(&commenter_and_channels);
    let commenter_channel_top_neighbors = commenter_and_channels.get(&commenter_channel_top_node).unwrap();
    println!("Commentor with the most comments is: {}\nChannels commented on are: {:?}\n", commenter_channel_top_node, commenter_channel_top_neighbors);

    let channel_top_node = analyser::most_neighbors(&channel_comments);
    let channel_num_of_top_neighbors = channel_comments.get(&channel_top_node).expect("Channel ID not inputted").len();
    println!("Channel with the most unique comments is: {}\nNumber of Unique Commentors are: {}\n\n\n", channel_top_node, channel_num_of_top_neighbors);

    let _channel_comments: u8 = 1; //Saves space, this map is never used again. Willing to allow a tiny leak : )
    println!("=============BFS===============");
    //BFS section of project
    let mut bfs_vector = Vec::new();
    let mut sum = 0.0;
    let mut count = 0.0;
    for n in comment_thread_neighbors.keys(){
        let bfs = analyser::bfs(n.as_str(), &comment_thread_neighbors);

        //Getting the average distance
        count += bfs.len() as f32;
        for distance in bfs.values(){
            sum += *distance as f32;
        }

        //Push at the end to avoid performance cost
        bfs_vector.push(bfs);
    }
    let average = sum / count;
    println!("The Average distance between all comment threads is: {:?}", average);

    analyser::channel_commentor_speration(&bfs_vector, &commenter_and_channels);

    println!("\n\n\n==========Page Rank============");

    let ranking_map = analyser::page_rank(&comment_thread_neighbors, 10000);

    let mut ranking_vec: Vec<(&String, &f64)> = ranking_map.iter().collect();
    ranking_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    let top_25: Vec<_> = ranking_vec.into_iter().take(25).collect();

    println!("Top 25 Commenfluencers via the Page Rank Algorithm");
    let mut count = 0;
    for (vertex, rank) in top_25 {
        count += 1;
        println!("{}. {}'s page rank: {}",count , vertex, rank);
    }
}
