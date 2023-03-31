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

#[cfg(test)]
mod tests {
    use super::handle;
    use tide_testing::TideTestingExt;

    #[async_std::test]
    async fn hello_world_works() -> tide::Result<()> {
        let mut app = tide::new();
        app.at("/").get(handle);

        assert_eq!(app.get("/").recv_string().await?, "Hello, world!");

        Ok(())
    }
}
