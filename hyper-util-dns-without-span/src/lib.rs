#![allow(dead_code)]
#![allow(unused_imports)]

use demo::tokio as tokio;
use demo::tracing as tracing;

async fn get_google() -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"));
    client
        .get("https://google.com/")
        .headers(headers.clone())
        .send()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    /// Main worker thread - current
    async fn test_main_current() -> Result<(), Box<dyn std::error::Error>> {
    
        demo::init_global_logger("global.log", tracing::Level::TRACE);
        
        let (dispatch, _guard) = demo::build_logger("local.log", tracing::Level::TRACE);
        let _default_guard = tracing::dispatcher::set_default(&dispatch);
        
        match get_google().await {
            Ok(res) => println!("No panic! And status: {}", res.status()),
            Err(e) => eprintln!("No panic! But error: {}", e),
        };
    
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    /// Main worker thread - multi
    async fn test_main_multi() -> Result<(), Box<dyn std::error::Error>> {
    
        demo::init_global_logger("global.log", tracing::Level::TRACE);
        
        let (dispatch, _guard) = demo::build_logger("local.log", tracing::Level::TRACE);
        let _default_guard = tracing::dispatcher::set_default(&dispatch);
        
        match get_google().await {
            Ok(res) => println!("No panic! And status: {}", res.status()),
            Err(e) => eprintln!("No panic! But error: {}", e),
        };
    
        Ok(())
    }
    
    #[tokio::test(flavor = "current_thread")]
    /// Spawn separate thread - current
    async fn test_spawn_current() -> Result<(), Box<dyn std::error::Error>> {
    
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
                match get_google().await {
                    Ok(res) => println!("No panic! And status: {}", res.status()),
                    Err(e) => eprintln!("No panic! But error: {}", e),
                };
            })
        });
    
        thread.join().unwrap();
    
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    /// Spawn separate thread - multi
    async fn test_spawn_multi() -> Result<(), Box<dyn std::error::Error>> {
    
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
                match get_google().await {
                    Ok(res) => println!("No panic! And status: {}", res.status()),
                    Err(e) => eprintln!("No panic! But error: {}", e),
                };
            })
        });
    
        thread.join().unwrap();
    
        Ok(())
    }
}