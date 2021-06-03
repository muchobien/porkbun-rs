use porkbun_rs::{api, api::Query, auth::Auth, endpoints, Porkbun};

fn main() -> eyre::Result<()> {
    let auth = Auth::new("apikey".into(), "apisecret".into());
    let client = Porkbun::new(auth)?;
    let endpoint = endpoints::Ping::builder().build()?;

    api::ignore(endpoint).query(&client)?;

    Ok(())
}
