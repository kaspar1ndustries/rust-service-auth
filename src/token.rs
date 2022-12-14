// #[macro_use]
use jwt_simple::prelude::*;
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

// use config
use crate::key::Key;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
}

impl Token {
    pub fn create_for_user(uid: i64) -> Self {
        let subject = format!("{}", uid);
        let access_token = Token::create_token(&subject, "access");
        let refresh_token = Token::create_token(&subject, "refresh");
        Token {
            access_token,
            refresh_token,
        }
    }
    pub fn create_token(subject: &str, token_type: &str) -> String {
        let key = Key::read();
        let duration_seconds;
        if token_type == "refresh" {
            duration_seconds = 3600 * 24 * 365;
        } else {
            duration_seconds = 30;
        }
        // claims with token type and user id as a subject
        let claims = Claims::create(Duration::from_secs(duration_seconds))
            .with_jwt_id(token_type)
            .with_subject(subject);

        key.authenticate(claims).unwrap()
    }

    fn create_from_header<'a>(header: &'a str) -> Token {
        let token = header.trim_start_matches("Bearer ");
        Token {
            access_token: String::from(token),
            refresh_token: String::from(token),
        }
    }
}

#[derive(Debug)]
pub enum ApiAuthError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiAuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if header contains valid auth token
        fn is_valid(header: &str) -> bool {
            let token = header.trim_start_matches("Bearer ");
            let key = Key::read();
            let claims = key.verify_token::<NoCustomClaims>(&token, None);
            println!("Claims: {:?}", claims);
            match claims {
                Ok(claims) => {
                    println!("Claims: {:?}", claims);
                    true
                }
                Err(error) => {
                    println!("Token error: {:?}", error);
                    false
                }
            }
        }

        match req.headers().get_one("Authorization") {
            // if not found
            None => Outcome::Failure((Status::BadRequest, ApiAuthError::Missing)),
            // if found check if valid
            Some(val) if is_valid(val) => Outcome::Success(Token::create_from_header(val)),
            // if not valid
            Some(_) => Outcome::Failure((Status::BadRequest, ApiAuthError::Invalid)),
        }
    }
}
