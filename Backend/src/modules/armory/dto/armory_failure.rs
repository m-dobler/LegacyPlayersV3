use okapi::openapi3::Responses;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponder;
use rocket_okapi::util::add_schema_response;
use schemars::JsonSchema;
use std::io::Cursor;

#[derive(Debug, JsonSchema)]
pub enum ArmoryFailure {
  InvalidInput,
  Database(String),
  ImplausibleInput
}

impl Responder<'static> for ArmoryFailure {
  fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
    let body;
    let status = match self {
      ArmoryFailure::InvalidInput => {
        body = "Invalid input!".to_owned();
        Status::new(534, "InvalidInput")
      },
      ArmoryFailure::Database(hint) => {
        body = hint.clone();
        Status::new(535, "Database")
      },
      ArmoryFailure::ImplausibleInput => {
        body = "Implausible input!".to_owned();
        Status::new(536, "ImplausibleInput")
      }
    };
    Response::build()
      .status(status)
      .sized_body(Cursor::new(body))
      .ok()
  }
}

impl OpenApiResponder<'static> for ArmoryFailure {
  fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
    let mut responses = Responses::default();
    let schema = gen.json_schema::<String>();
    add_schema_response(&mut responses, 534, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 535, "text/plain", schema.clone())?;
    add_schema_response(&mut responses, 536, "text/plain", schema.clone())?;
    Ok(responses)
  }
}