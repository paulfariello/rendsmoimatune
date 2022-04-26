use diesel;
use rocket::{
    http::ContentType,
    request::Request,
    response::{self, Responder, Response},
};
use std::io::Cursor;

#[derive(Debug)]
pub enum Error {
    DbError(diesel::result::Error),
    IdError,
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::DbError(err)
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let resp = format!("{:?}", self);
        Response::build()
            .header(ContentType::Plain)
            .sized_body(resp.len(), Cursor::new(resp))
            .ok()
    }
}
