// use hyper::Client;
//
// #[tokio::main]
// pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     // This is where we will setup our HTTP client requests.
//
//     // Previously...
//     let client = Client::new();
//     let uri = "http://httpbin.org/ip".parse()?;
//     let mut resp = client.get(uri).await?;
//     println!("Response: {}", resp.status());
//
//     // And now...
//     while let Some(chunk) = resp.body_mut().data().await {
//         stdout().write_all(&chunk?).await?;
//     }
//
//     Ok(())
// }

pub fn test(){
    println!("aaaa")
}