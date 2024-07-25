use flate2::{write::GzEncoder, Compression};
use rocket::{
  fairing::{Fairing, Info, Kind},
  http::ContentType,
  Request, Response,
};
use serde::Serialize;
use nanoid::nanoid;
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
pub struct ApiResponse<T> {
  pub ret_code: u16,
  pub ret_msg: String,
  pub response: Option<T>,
  pub session_id: String,
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
    if response.content_type() != Some(ContentType::JSON) {
      return;
    }
    let session_id = request.query_value("session_id").unwrap_or(Ok(nanoid!())).unwrap();
    let body_string = response.body_mut().to_string().await.unwrap();
    let body_json: Value = serde_json::from_str(&body_string).unwrap();
    let (message, data) = if code == 200 {
      ("success".to_string(), Some(body_json))
    } else {
      (body_json.as_str().unwrap().to_string(), None)
    };
    let ret_code = if code == 200 { 0 } else { code };
    let api_response = ApiResponse {
      ret_code,
      ret_msg:message,
      response: data,
      session_id,
    };
    let api_response_string = to_string(&api_response).unwrap();
    *response = rocket::Response::build()
      .header(ContentType::JSON)
      .sized_body(api_response_string.len(), Cursor::new(api_response_string))
      .finalize();
  }
}
