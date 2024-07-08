use flate2::{write::GzEncoder, Compression};
use rocket::{
  fairing::{Fairing, Info, Kind},
  Request, Response,
};
use std::io::Write;
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
