use porkbun_rs::{api, api::AsyncQuery, auth::Auth, endpoints, AsyncPorkbun};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let auth = Auth::new("apikey".into(), "apisecret".into());
    let client = AsyncPorkbun::new(auth)?;
    let endpoint = endpoints::Ping::builder().build()?;

    api::ignore(endpoint).query_async(&client).await?;

    Ok(())
}
