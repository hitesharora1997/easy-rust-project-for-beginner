use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;


#[derive(Deserialize,Debug)]
struct User{
    login: String,
    id: u32
};


#[tokio::main]
async fn main() -> Result<(), Error>{
    
}
