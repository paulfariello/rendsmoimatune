use anyhow;
use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};
use yew::prelude::*;

#[derive(Error, Display, Debug)]
enum Error {
    #[error("Network error: {0}")]
    NetworkError(#[from] gloo_net::Error),
    #[error("Error: {0}")]
    ServerError(String),
}

// Wait for async trait to impl
// pub trait RequestResult {
//     async fn result<R>(&self) -> anyhow::Result<R>
//     where
//         R: DeserializeOwned;
// }
//
// impl RequestResult for gloo_net::http::Request {
//     async fn result<R>(&self) -> anyhow::Result<R>
//     where
//         R: DeserializeOwned,
//     {
//         let response = self.send().await?;
//         if response.ok() {
//             match response.json().await {
//                 Ok(ret) => Ok(ret),
//                 Err(err) => Err(anyhow::Error::new(err)),
//             }
//         } else {
//             match response.text().await {
//                 Ok(text) => Err(anyhow::anyhow!(text)),
//                 Err(err) => Err(anyhow::Error::new(err)),
//             }
//         }
//     }
// }

pub async fn post<D, R>(url: &str, data: &D) -> anyhow::Result<R>
where
    D: Serialize + ?Sized,
    R: DeserializeOwned,
{
    let response = Request::post(url).json(data)?.send().await?;
    if response.ok() {
        match response.json().await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    } else {
        match response.text().await {
            Ok(text) => Err(anyhow::anyhow!(text)),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }
}

pub async fn put<D, R>(url: &str, data: &D) -> anyhow::Result<R>
where
    D: Serialize + ?Sized,
    R: DeserializeOwned,
{
    let response = Request::put(url).json(data)?.send().await?;
    if response.ok() {
        match response.json().await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    } else {
        match response.text().await {
            Ok(text) => Err(anyhow::anyhow!(text)),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }
}

pub async fn get_with_query<'a, T, V, R>(url: &str, query: T) -> anyhow::Result<R>
where
    R: DeserializeOwned,
    T: IntoIterator<Item = (&'a str, V)>,
    V: AsRef<str>,
{
    let response = Request::get(url).query(query).send().await?;
    if response.ok() {
        match response.json().await {
            Ok(ret) => Ok(ret),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    } else {
        match response.text().await {
            Ok(text) => Err(anyhow::anyhow!(text)),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }
}

pub async fn get<R>(url: &str) -> Result<R, Error>
where
    R: DeserializeOwned,
{
    let response = Request::get(url).send().await?;
    if response.ok() {
        Ok(response.json().await?)
    } else {
        Err(Error::ServerError(response.text().await?))
    }
}

pub fn loading() -> Html {
    html! {
        <div>{ "Loading…" }</div>
    }
}
