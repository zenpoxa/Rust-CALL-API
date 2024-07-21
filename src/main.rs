#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url : &str = "http://api.open-notify.org/astros.json";
    let res: serde_json::Value = reqwest::get(url).await?.json().await?;

    let n = &res["number"];
    let n = n.as_u64().unwrap();

    println!("{n}");

    Ok(())
}
