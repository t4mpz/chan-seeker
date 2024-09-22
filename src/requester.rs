pub mod requester {
  use fantoccini::wd::Capabilities;

  #[tokio::main]
  pub async fn get_url(url: String) -> Result<String, Box<dyn std::error::Error>>{
    let cap: Capabilities = serde_json::from_str(r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,).unwrap();
    let client = fantoccini::ClientBuilder::native()
    .capabilities(cap)
    .connect("http://localhost:38073").await.expect("No webdriver?"); // make sure chromedriver is running >o<

    client.goto(&url).await?;
    let binding = client.current_url().await?;
    let client_url = binding.as_str();
    assert_eq!(url, client_url); // testing if the url loaded in the headless, cuz headless

    let body = client.source().await?;

    client.close().await?;

    Ok(body)
  }

  #[tokio::main]
  pub async fn show_image(url: String) -> Result<String, Box<dyn std::error::Error>>{
    let cap: Capabilities = serde_json::from_str(r#"{"browserName":"chrome","goog:chromeOptions":{}}"#,).unwrap();
    let client = fantoccini::ClientBuilder::native()
    .capabilities(cap)
    .connect("http://localhost:38073").await.expect("No webdriver?"); // make sure chromedriver is running >o<
    // i'll have toi check if two webbrowsers can run at the same time, they probably can, but who knows?

    client.goto(&url).await?;
    let binding = client.current_url().await?;
    let client_url = binding.as_str();
    assert_eq!(url, client_url); // testing if the url loaded in the headless, cuz headless

    let body = client.source().await?;

    client.close().await?;

    Ok(body)
  }

}
