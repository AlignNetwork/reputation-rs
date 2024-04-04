//use ethers::prelude::*;
use ndarray::Array2;
//use std::sync::Arc;

pub async fn fetch_trust_matrix() -> Result<Array2<f64>, Box<dyn std::error::Error>> {
    // Set up the provider with the Ethereum node URL
    //let provider = Provider::<Http>::try_from("https://v2.rpc.align.network")?;
    //let provider = Arc::new(provider);

    //let attestation_station = "0x66792E53CC67DD64c5a8f63085775DCFb79Aada9".parse::<Address>()?;
    //let contract_abi = serde_json::from_str::<ethers::abi::Abi>(include_str!("../abi.json"))?;

    // Creating the contract object with the deserialized ABI
    //let contract = Contract::new(attestation_station, contract_abi, provider.clone());
    // get the attestations you wan to build trust matrix for
    /* let trust_matrix: Vec<Vec<f64>> = contract
        .method::<(), Vec<Vec<f64>>>("getAttestations", ())?
        .call()
        .await?; */

        let trust_matrix = Array2::from_shape_vec((5, 5), vec![
          0.0, 0.2, 0.3, 0.5, 0.0,
          0.1, 0.0, 0.1, 0.1, 0.7,
          0.4, 0.1, 0.0, 0.2, 0.3,
          0.1, 0.1, 0.7, 0.0, 0.1,
          0.3, 0.1, 0.4, 0.2, 0.0,
      ]).unwrap();

    Ok(trust_matrix)
}
