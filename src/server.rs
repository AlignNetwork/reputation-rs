use serde::{Serialize, Deserialize};
use serde_json::to_string_pretty;
use tokio::fs::write as async_write;
use warp::Filter;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct TrustScore {
    node: usize,
    score: f64,
}

// Function to save trust scores to a JSON file
pub async fn save_trust_scores(scores: &Vec<TrustScore>) -> Result<(), Box<dyn std::error::Error>> {
    let json = to_string_pretty(scores)?;
    async_write("trust_scores.json", json).await?;
    Ok(())
}

pub async fn run_server() {
  // Set up a route to serve the trust scores JSON
  let trust_scores_route = warp::path("trust_scores")
      .and(warp::fs::file("trust_scores.json"));

  warp::serve(trust_scores_route)
      .run(([127, 0, 0, 1], 3030))
      .await;
}
