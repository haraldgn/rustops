async fn handle(_req: tide::Request<()>) -> tide::Result<String> {
    Ok("Hello, world!".to_string())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(handle);
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
