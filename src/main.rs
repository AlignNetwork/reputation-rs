
mod eth;
mod server;
use warp::Filter;


use tokio;
use ndarray::{Array2, Array1};


#[tokio::main]
async fn main() {
  match eth::fetch_trust_matrix().await {
    Ok(trust_matrix) => {
        println!("Trust Matrix: {:?}", trust_matrix);
        // Further processing...
    
    

    let n = trust_matrix.shape()[0]
    let mut global_trust_scores = Array1::from_vec(vec![1000.0, 2000.0, 500.0, 300.0, 200.0]);

    // Normalize local trust scores row-wise
    for i in 0..n {
        let row_sum: f64 = trust_matrix.row(i).sum();
        if row_sum > 0.0 {
            trust_matrix.row_mut(i).mapv_inplace(|x| x / row_sum);
        }
    }

    // Iteratively compute global trust scores
    for _ in 0..100 {
        let new_global_trust_scores = trust_matrix.t().dot(&global_trust_scores);
        let sum: f64 = new_global_trust_scores.sum();
        let normalized_global_trust_scores = new_global_trust_scores / sum;

        if (&global_trust_scores - &normalized_global_trust_scores).mapv(|a| a.abs()).sum() < 1e-6 {
            break;
        }
        global_trust_scores = normalized_global_trust_scores;
    }

    // Output final trust scores
    // TODO: save them onchain or on the server?
    for (i, &score) in global_trust_scores.iter().enumerate() {
        println!("Node {}: Global Trust Score = {}", i, score);

    }
    let scores = vec![ /* initialize your scores here */ ];
    if let Err(e) = server::save_trust_scores(&scores).await {
        eprintln!("Failed to save trust scores: {}", e);
        return;
    }

    // Set up a route to serve the trust scores JSON
    let trust_scores = warp::path("trust_scores")
        .and(warp::fs::file("trust_scores.json"));
  },
  Err(e) => eprintln!("Error: {}", e),
  }
}
