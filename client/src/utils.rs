use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use yew::prelude::*;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("{0}")]
    JsError(String),
    /// Error returned by `serde` during deserialization.
    #[error("{0}")]
    SerdeError(String),
    /// Error returned by this crate
    #[error("{0}")]
    GlooError(String),
    #[error("Error: {0}")]
    ServerError(String),
}

impl From<gloo_net::Error> for Error {
    fn from(error: gloo_net::Error) -> Self {
        match error {
            gloo_net::Error::JsError(error) => Error::JsError(error.to_string()),
            gloo_net::Error::SerdeError(error) => Error::SerdeError(error.to_string()),
            gloo_net::Error::GlooError(error) => Error::GlooError(error),
        }
    }
}

// Wait for async trait to impl
// pub trait RequestResult {
//     async fn result<R>(&self) -> Result<R, Error>
//     where
//         R: DeserializeOwned;
// }
//
// impl RequestResult for gloo_net::http::Request {
//     async fn result<R>(&self) -> Result<R, Error>
//     where
//         R: DeserializeOwned,
//     {
//         let response = self.send().await?;
//         if response.ok() {
//             Ok(response.json().await?)
//         } else {
//             Err(Error::ServerError(response.text().await?))
//         }
//     }
// }

pub async fn post<D, R>(url: &str, data: &D) -> Result<R, Error>
where
    D: Serialize + ?Sized,
    R: DeserializeOwned,
{
    let response = Request::post(url).json(data)?.send().await?;
    if response.ok() {
        Ok(response.json().await?)
    } else {
        Err(Error::ServerError(response.text().await?))
    }
}

pub async fn put<D, R>(url: &str, data: &D) -> Result<R, Error>
where
    D: Serialize + ?Sized,
    R: DeserializeOwned,
{
    let response = Request::put(url).json(data)?.send().await?;
    if response.ok() {
        Ok(response.json().await?)
    } else {
        Err(Error::ServerError(response.text().await?))
    }
}

pub async fn get_with_query<'a, T, V, R>(url: &str, query: T) -> Result<R, Error>
where
    R: DeserializeOwned,
    T: IntoIterator<Item = (&'a str, V)>,
    V: AsRef<str>,
{
    let response = Request::get(url).query(query).send().await?;
    if response.ok() {
        Ok(response.json().await?)
    } else {
        Err(Error::ServerError(response.text().await?))
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
