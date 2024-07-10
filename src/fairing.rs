use flate2::{write::GzEncoder, Compression};
use rocket::{
  fairing::{Fairing, Info, Kind},
  http::ContentType,
  Request, Response,
};
use serde::Serialize;
use serde_json::{to_string, Value};
use std::io::{Cursor, Write};
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

#[derive(Serialize)]
struct ApiResponse<T> {
  status: u16,
  message: String,
  data: T,
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
  async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
    if response.content_type() != Some(ContentType::JSON) {
      return;
    }
    let body_string = response.body_mut().to_string().await.unwrap();
    let body_json: Value = serde_json::from_str(&body_string).unwrap();
    let api_response = ApiResponse {
      status: response.status().code,
      message: "Success".to_string(),
      data: body_json,
    };
    let api_response_string = to_string(&api_response).unwrap();
    *response = rocket::Response::build()
      .header(ContentType::JSON)
      .sized_body(api_response_string.len(), Cursor::new(api_response_string))
      .finalize();
  }
}
