use flate2::{write::GzEncoder, Compression};
use nanoid::nanoid;
use rocket::{
  fairing::{Fairing, Info, Kind},
  http::ContentType,
  Data, Request, Response,
};
use serde_json::to_string;
use std::io::{Cursor, Write};

use crate::responder::ApiResponse;
pub struct Gzip;

#[rocket::async_trait]
impl Fairing for Gzip {
  fn info(&self) -> Info {
    Info {
      name: "Gzip compression",
      kind: Kind::Response,
    }
  }
  async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
    if request
      .headers()
      .get("Accept-Encoding")
      .any(|e| e.to_lowercase().contains("gzip"))
    {
      let body_bytes = response.body_mut().to_bytes().await.unwrap();
      let mut buf = Vec::with_capacity(body_bytes.len());
      let mut encoder = GzEncoder::new(&mut buf, Compression::fast());
      encoder.write_all(&body_bytes).unwrap();
      encoder.finish().unwrap();
      response.set_sized_body(buf.len(), std::io::Cursor::new(buf));
      response.set_raw_header("Content-Encoding", "gzip");
    }
  }
}
pub struct JsonResponse;

#[rocket::async_trait]
impl Fairing for JsonResponse {
  fn info(&self) -> Info {
    Info {
      name: "Json Response",
      kind: Kind::Response,
    }
  }
  async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
    let code = response.status().code;
    if code == 200 || code == 401 {
      return;
    }
    let session_id = request
      .query_value("session_id")
      .unwrap_or(Ok(nanoid!()))
      .unwrap();
    let body_string = response.body_mut().to_string().await.unwrap();
    let api_response: ApiResponse<String> = ApiResponse {
      ret_code: code,
      ret_msg: body_string,
      response: None,
      session_id,
    };
    let api_response_string = to_string(&api_response).unwrap();
    *response = Response::build()
      .header(ContentType::JSON)
      .sized_body(api_response_string.len(), Cursor::new(api_response_string))
      .finalize();
  }
}

pub struct Log;

#[rocket::async_trait]
impl Fairing for Log {
  fn info(&self) -> Info {
    Info {
      name: "Logging Fairing",
      kind: Kind::Response | Kind::Request,
    }
  }
  async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
    println!("Incoming request: {:?}", request);
  }

  async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
    println!("Response to {}: {:?}", request, response);
  }
}
