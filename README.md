# Rust library for accessing the Porkbun API

An Porkbun API client written in 🦀.

[Porkbun](https://porkbun.com/) is a 🦀 client library for accessing the Porkbun API.

## Examples

### Synchronous

```rs
use porkbun_rs::{api, api::Query, auth::Auth, endpoints, Porkbun};

fn main() -> eyre::Result<()> {
    let auth = Auth::new("apikey".into(), "apisecret".into());
    let client = Porkbun::new(auth)?;
    let endpoint = endpoints::Ping::builder().build()?;

    api::ignore(endpoint).query(&client)?;

    Ok(())
}
```

### Asynchronous

```rs
use porkbun_rs::{api, api::AsyncQuery, auth::Auth, endpoints, AsyncPorkbun};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let auth = Auth::new("apikey".into(), "apisecret".into());
    let client = AsyncPorkbun::new(auth)?;
    let endpoint = endpoints::Ping::builder().build()?;

    api::ignore(endpoint).query_async(&client).await?;

    Ok(())
}
```

## API Documentation

- [API Docs](https://porkbun.com/api/json/v3/documentation)
