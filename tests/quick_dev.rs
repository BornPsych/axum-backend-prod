use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8000")?;

    hc.do_get("/hello?name=yogesh").await?.print().await?;
    hc.do_get("/hello2/shahi").await?.print().await?;

    Ok(())
}
