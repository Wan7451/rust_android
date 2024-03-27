use std::collections::HashMap;
use std::error::Error;
use std::thread;
use std::time::Duration;

pub async fn get_request() -> Result<String, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?.text().await;
    println!("{res:#?}");
    res
}

// #[tokio::main]
// pub async fn test_request() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{resp:#?}");
//     Ok(())
// }


pub fn test_request<F>(cd:F) where F: FnOnce(String) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let a = get_request().await;
        match a {
            Ok(a) => {
                cd(a.clone());
                println!("{a}");
            }
            Err(e) => {
                cd(e.to_string());
                println!("{e}");
            }
        }
    })
}


// pub async fn request() -> Result<(), Box<dyn Error>> {
//     // let client = Client::builder(TokioExecutor::new())
//     //     .pool_idle_timeout(Duration::from_secs(30))
//     //     .http2_only(true)
//     //     .build_http();
//     //
//     // // 构建一个GET请求
//     // let uri = "https://httpbin.org/ip";
//     // let request = Request::get(uri).body(())?;
//     //
//     // // 发送请求并获取响应
//     // let response = client.request(request).await?;
//     //
//     // // 打印响应状态码
//     // println!("Status: {}", response.status());
//     //
//     Ok(())
// }

