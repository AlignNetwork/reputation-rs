use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::{self};
use reqwest::Client;

use std::{error::Error, fmt};

#[derive(Debug)]
enum MyError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    // You can add more error variants as needed
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> MyError {
        MyError::Reqwest(err)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        MyError::Serde(err)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::Reqwest(err) => write!(f, "HTTP request error: {}", err),
            MyError::Serde(err) => write!(f, "Serde JSON error: {}", err),
            // Handle other cases
        }
    }
}

impl Error for MyError {}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    // Add more fields as per your JSON structure
}

#[derive(Serialize)]
struct PageParam {
    pageParam: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    data: String, // The data is initially a JSON string
    nextPage: Option<Value>, // Using Value to handle any type, as "nextPage" can be null
}

#[derive(Serialize, Deserialize, Debug)]
struct DataItem {
    id: i32,
    chain_id: i32,
    caller: String,
    transaction_hash: String,
    event_type: String,
    data: DataContent,
    l2_block_number: i32,
    l3_block_number: i32,
    value: String,
    timestamp: i64,
    to: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataContent {
    claim: String,
    claimProof: String,
}




async fn get_attested() -> Result<String, MyError> {
  let page_param = PageParam { pageParam: "1".to_string() };
    // Supabase endpoint URL
    let url = "http://127.0.0.1:54321/functions/v1/get-attested";
    // Supabase API Key (Be cautious to keep it secure and not expose it unnecessarily)
    let api_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InNjeGFibmd4d25oaWhkc2JpamlwIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDc0MTgwODgsImV4cCI6MjAyMjk5NDA4OH0.PWo592WM_amL2An3NzngJ3JOx9y4c-T4gFd8E5bXgzU";
    
    let client = Client::new();
    let response = client
    .post(url)
    .bearer_auth(api_key)
    .json(&page_param)
    .header("Content-Type", "application/json")
    .send()
    .await?
    .text() // Extract the text body directly instead of .json::<Value>()
    .await
    .map_err(MyError::from)?; // Also converts reqwest::Error to MyError
  
  // Print out the fetched users
    println!("error {:#?}", response);
    
    Ok(response)
  }

  async fn get_claim_types() -> Result<(), MyError> {
      // Supabase endpoint URL
      let url = "http://127.0.0.1:54321/functions/v1/get-claim-types";
      // Supabase API Key (Be cautious to keep it secure and not expose it unnecessarily)
      let api_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InNjeGFibmd4d25oaWhkc2JpamlwIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDc0MTgwODgsImV4cCI6MjAyMjk5NDA4OH0.PWo592WM_amL2An3NzngJ3JOx9y4c-T4gFd8E5bXgzU";
      
      let client = reqwest::Client::new();
      let response = client
      .post(url)
      .bearer_auth(api_key)
      .header("Content-Type", "application/json")
      .send()
      .await?
      .json::<Value>() // Assuming the response is an array of users
      .await?;
    
    // Print out the fetched users
      println!("{:#?}", response);
      
      Ok(())
    }
  
    


  

    async fn parse_response(json_str: &str) -> Result<(), MyError> {
      // First, deserialize the JSON string into `ApiResponse`
      let api_response: ApiResponse = serde_json::from_str(json_str)?;
      println!("{:#?}", api_response);
      // `api_response.data` is a String containing JSON, so parse it separately
      let data_items: Vec<DataItem> = serde_json::from_str(&api_response.data)?;
  
      // Now `data_items` is a Vec<DataItem> you can work with
      println!("{:#?}", data_items);
  
      Ok(())
  }
    #[tokio::main]
    async fn main() {
      match get_attested().await {
        Ok(response_body) => {
            // `response_body` is now a `String` that you can use directly.
            // If you want to parse it as JSON, you can do so here.
            match parse_response(&response_body).await {
                Ok(_) => println!("Successfully parsed the response."),
                Err(e) => eprintln!("Error parsing JSON: {}", e),
            }
        },
        Err(e) => eprintln!("Error fetching attested data: {}", e),
    }
    }
