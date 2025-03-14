use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8000")?;

    hc.do_get("/hello?name=yogesh").await?.print().await?;
    
    let req_login = hc.do_post("/api/login", json!({
        "username":"demo1",
        "pwd":"welcome"
    }));
    
    req_login.await?.print().await?;
    
    hc.do_get("/hello2/shahi").await?.print().await?;
    Ok(())
}
