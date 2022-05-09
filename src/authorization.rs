// use std::collections::HashSet;

// use actix_web::Error;
// use actix_web::dev::ServiceRequest;
// use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
// use actix_web_httpauth::extractors::AuthenticationError;

// use jsonwebtoken::errors::ErrorKind;
// use jsonwebtoken::jwk::AlgorithmParameters;
// use jsonwebtoken::{jwk, decode, DecodingKey, Validation, Algorithm, decode_header};
// use serde::{Deserialize, Serialize};

// pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     let config = req
//         .app_data::<Config>()
//         .map(|data| data.clone())
//         .unwrap_or_else(Default::default);
//     match validate_token(credentials.token()) {
//         Ok(_) => {
//             Ok(req)
//         }
//         Err(_) => Err(AuthenticationError::from(config).into()),
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     exp: usize,
// }

// fn validate_token(token: &str) -> Result<(), ()> {
//     let key_url = "https://dev-q1c7r97i.eu.auth0.com/.well-known/jwks.json";
//     let audience = "https://hestia.mladedav.ml/";
//     let issuer = "https://dev-q1c7r97i.eu.auth0.com/";

//     let header = decode_header(token).unwrap();
//     let kid = match header.kid {
//         Some(k) => k,
//         None => panic!("Token doesn't have a `kid` header field")
//     };

//     let key = fetch_jwk(key_url, &kid);

//     let mut iss = HashSet::<String>::new();
//     iss.insert(issuer.to_owned());
//     let mut aud = HashSet::<String>::new();
//     aud.insert(audience.to_owned());

//     Validation::default();

//     let mut validation = Validation::new(Algorithm::RS256);
//     validation.set_issuer(&[issuer]);
//     validation.set_audience(&[audience]);
//     match decode::<Claims>(token, &key, &validation) {
//         Ok(_) => Ok(()),
//         Err(err) => match *err.kind() {
//             ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
//             ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
//             ErrorKind::InvalidAlgorithm => panic!("Algorithm is invalid"),
//             _ => {
//                 panic!("Some other error: {:?}", *err.kind());
//             }
//         },
//     }
// }

// fn fetch_jwk(uri: &str, kid: &str) -> DecodingKey {
//     let jwks = reqwest::blocking::get(uri).unwrap().json::<jwk::JwkSet>().unwrap();

//     if let Some(jwk) = jwks.find(kid) {
//         match jwk.algorithm {
//             AlgorithmParameters::RSA(ref rsa) => {
//                 let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
//                 return decoding_key;
//             }
//             _ => panic!("Key is not RSA")
//         }
//     } else {
//         panic!("No matching JWK found for the given kid");
//     }
// }
