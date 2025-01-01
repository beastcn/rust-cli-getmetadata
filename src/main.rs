
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};
use serde::Serialize;
use std::sync::Arc;

// Define the structure for JSON output
#[derive(Serialize)]
struct TokenMetadata {
    name: String,
    symbol: String,
    decimals: u8,
}

// Basic ERC20 ABI with just the functions we need for get_metadata
abigen!(
    ERC20,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
    ]"#,
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Check if an address is provided as argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <erc20_address>", args[0]);
        return Err(eyre::eyre!("Invalid number of arguments"));
    }

    // Parse the address
    let address = args[1].parse::<Address>()
        .map_err(|e| eyre::eyre!("Invalid Ethereum address: {}", e))?;

    // Connect to Ethereum mainnet using a public RPC endpoint
    let api_key = std::env::var("ALCHEMY_API_KEY")
        .map_err(|_| eyre::eyre!("ALCHEMY_API_KEY environment variable not set"))?;
    let rpc_url = format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key);
    
    let provider = Provider::<Http>::try_from(rpc_url)
        .map_err(|e| eyre::eyre!("Failed to connect to Ethereum node: {}", e))?;
    let provider = Arc::new(provider);

    // Create contract instance
    let contract = ERC20::new(address, provider.clone());

    // Fetch token metadata
    let name = contract.name().call().await
        .map_err(|e| eyre::eyre!("Failed to fetch token name: {}", e))?;
    let symbol = contract.symbol().call().await
        .map_err(|e| eyre::eyre!("Failed to fetch token symbol: {}", e))?;
    let decimals = contract.decimals().call().await
        .map_err(|e| eyre::eyre!("Failed to fetch token decimals: {}", e))?;

    // Create and serialize the token metadata
    let token_metadata = TokenMetadata {
        name,
        symbol,
        decimals,
    };

    // Print the JSON output
    println!("{}", serde_json::to_string_pretty(&token_metadata)
        .map_err(|e| eyre::eyre!("Failed to serialize metadata: {}", e))?);

    Ok(())
} 