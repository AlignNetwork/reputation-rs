use serde::{Serialize, Deserialize};
use warp::Filter;
use serde_json::to_string_pretty;
use tokio::fs::write as async_write;
use ndarray::Array1;

#[derive(Serialize, Deserialize)]
struct TrustScore {
    node: usize,
    score: f64,
}

// Function to save trust scores array directly to a JSON file
pub async fn save_trust_scores(scores: &Array1<f64>) -> Result<(), Box<dyn std::error::Error>> {
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
