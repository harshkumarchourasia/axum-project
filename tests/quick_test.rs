use anyhow::Result;
use httpc_test;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hello?name=Harsh").await?.print().await;
    hc.do_get("/bye/Harsh").await?.print().await;

    hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        }),
    )
    .await?
    .print()
    .await;
    Ok(())
}
