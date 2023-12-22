use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;
use std::convert::Infallible;

pub struct XHR;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for XHR {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if request.headers().contains("X-Requested-With") && 
           request.headers().get("X-Requested-With").any(|x| x == "XMLHttpRequest") {
            Outcome::Success(XHR)
        } else {
            Outcome::Forward(Status::BadRequest)
        }
    }
}
