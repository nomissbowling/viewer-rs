//! ut
//!

use std::path::{Path, PathBuf};
use std::{result::Result, error::Error};
use std::sync::Arc;

use mime_guess;
use reqwest::{cookie, header, Url};

/// mk_agent
pub fn mk_agent() -> String {
  let mo = "Mozilla/5.0";
  let os = "Rust"; // "Windows NT 10.0";
  let cpu = "Viewer"; // "Win64; x64";
  let rv = "160.0";
  let si = format!("{}; {}; rv:{}", os, cpu, rv);
  let pf = "Gecko/20100101";
  let br = "Firefox";
  format!("{} ({}) {} {}/{}", mo, si, pf, br, rv)
}

/// mk_default_headers (contains Accept: */*)
pub fn mk_default_headers() -> header::HeaderMap {
  let agent = mk_agent();
  // println!("{}", agent);
  let mut default_headers = header::HeaderMap::new();
  default_headers.insert(header::ACCEPT_ENCODING,
    header::HeaderValue::from_static("gzip, deflate, br")); // need features
  default_headers.insert(header::USER_AGENT,
    header::HeaderValue::try_from(&agent).expect("user-agent"));
  default_headers
}

/// mk_headers
pub fn mk_headers(h: &[(&str, &str)]) -> header::HeaderMap {
  let mut headers = header::HeaderMap::new();
  h.iter().for_each(|&(k, v)| {
    headers.insert(header::HeaderName::try_from(k).expect("header key"),
      header::HeaderValue::try_from(v).expect("header value"));
  });
  headers
}

/// cookies (needless to mut cj)
pub fn cookies(cj: &Arc<cookie::Jar>, c: &[(&str, &str)], url: &str) ->
  Result<(), Box<dyn Error>> {
  let pu = Url::parse(url)?;
  c.iter().for_each(|&(k, v)| {
    cj.add_cookie_str(&format!("{}={}", k, v), &pu);
  });
  Ok(())
}

/// guess_type
pub fn guess_type(src: &str) -> String {
  mime_guess::from_path(Path::new(src)).first_or_octet_stream().as_ref().into()
}

/// splitext
pub fn splitext(src: &str) -> String {
  let p = PathBuf::from(src); // .extension via Option<&OsStr> via Option<&str>
  let ext = p.extension().map_or("", |osstr| osstr.to_str().map_or("", |e| e));
  format!("{}{}", if ext.len() > 0 { "." } else { "" }, ext)
}
