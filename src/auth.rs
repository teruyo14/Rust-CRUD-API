use base64::engine::general_purpose;
use base64::Engine;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_auth_header(header: &str) -> Option<BasicAuth> {
        let input = header.split_whitespace().collect::<Vec<_>>();
        if input.len() != 2 {
            return None;
        }

        if input[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(input[1])
    }

    fn from_base64_encoded(base_str: &str) -> Option<BasicAuth> {
        let decoded = general_purpose::STANDARD.decode(base_str).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(':').collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        Some(BasicAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if let Some(auth) = Self::from_auth_header(auth_header) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
