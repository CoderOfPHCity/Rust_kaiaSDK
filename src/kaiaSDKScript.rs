

use kaiascan_sdk::{KaiaScan, Address};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // STEP 1: Initialize client
    let client = KaiaScan::new()?;
    
    // STEP 2: Create test address
    let address = Address::new("0x1234567890abcdef");
    
    // STEP 3: Make API call
    println!("Making API call...");
    match client.get_fungible_token(address).await {
        Ok(response) => {
            // STEP 4: Process successful response
            println!("Success! Token name: {}", response.data.name);
            println!("Full response: {:#?}", response);
        },
        Err(e) => {
            // STEP 5: Handle error
            println!("Error occurred: {}", e);
        }
    }

    Ok(())
}