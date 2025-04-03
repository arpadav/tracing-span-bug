#![allow(unused_imports)]
use demo::tokio as tokio;
use demo::tracing as tracing;

#[tokio::test(flavor = "multi_thread")]
async fn test() -> Result<(), Box<dyn std::error::Error>> {

    demo::init_global_logger("global.log", tracing::Level::TRACE);

    let thread = std::thread::spawn(move || {

        let (dispatch, _guard) = demo::build_logger("local.log", tracing::Level::TRACE);
        let _default_guard = tracing::dispatcher::set_default(&dispatch);
        let rt = tokio::runtime::Builder::new_current_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap_or_else(|_| panic!("Unable to create async runtime"));

        rt.block_on(async move {

            let client = reqwest::Client::new();
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"));

            let response = client
                .get("https://google.com/")
                .headers(headers.clone())
                .send()
                .await;

            match response {
                Ok(res) => println!("No panic! And status: {}", res.status()),
                Err(e) => eprintln!("No panic! But error: {}", e),
            };
        
        })
    });

    thread.join().unwrap();

    Ok(())
}