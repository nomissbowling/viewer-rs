//! core
//!

use std::fs;
use std::path::{Path, PathBuf};
use std::{result::Result, error::Error};
use std::time::Duration;
use std::sync::Arc;

use rstk::{self, *};
use mime_guess;
use reqwest::{self, multipart, cookie, header, Client};
use tokio::runtime::Runtime;

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

/// add_cookies (needless to mut cj)
pub fn add_cookies(cj: &Arc<cookie::Jar>, c: &[(&str, &str)], url: &str) ->
  Result<(), Box<dyn Error>> {
  let pu = reqwest::Url::parse(url)?;
  c.iter().for_each(|&(k, v)| {
    cj.add_cookie_str(&format!("{}={}", k, v), &pu);
  });
  Ok(())
}

/// reqwest_test
pub fn reqwest_test(client: &Client, url: &str, q: &[(&str, &str)],
  hdrs: header::HeaderMap, b: &str) -> Result<(), Box<dyn Error>> {
  // let lb = Box::leak(format!("{}", b).into_boxed_str()); // (same as below)
  let lb = b.to_string().leak(); // body requires &'static (into &'static str)
  Runtime::new()?.block_on((async || -> Result<(), Box<dyn Error>> {
    // let resp = reqwest::get(url).await?;
    let resp = client.get(url).query(q).headers(hdrs).body(&*lb).send().await?;
    println!("resp {}: len: {}", resp.status(), resp.text().await?.len());
    Ok(())
  })())
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

/// uploader_catbox_litterbox
/// - This test places a load on the target; use it with caution.
// (see also ________github_memo.txt)
pub fn uploader_catbox_litterbox(src: &str) -> Result<(), Box<dyn Error>> {
  let dst = format!("unknown{}", splitext(src));
  let mt = guess_type(src);
  println!("{} {} {}", dst, src, mt);
  let dat: Vec<u8> = fs::read(src)?;
  let part = multipart::Part::bytes(dat).file_name(dst).mime_str(&mt)?;
/*
  // create *random* hash like https://litter.catbox.moe/xxxxxxxxxxxxxxxx.png
  let mp = multipart::Form::new()
    .text("reqtype", "fileupload")
    .text("time", "24h") // to be expired (1h 12h 24h 72h)
    .text("fileNameLength", "16") // (16 or 6)
    .part("fileToUpload", part);
  let url_post = "https://litterbox.catbox.moe/resources/internals/api.php";
*/
  // create always same hash like https://files.catbox.moe/riqo8l.png (forever)
  let mp = multipart::Form::new()
    .text("reqtype", "fileupload")
    .part("fileToUpload", part);
  let url_post = "https://catbox.moe/user/api.php";
  let cj = Arc::new(cookie::Jar::default()); // needless to mut cj
  // let client = reqwest::Client::new();
  let builder = reqwest::Client::builder().timeout(Duration::from_secs(5))
    .cookie_provider(cj).default_headers(mk_default_headers());
  let client = builder.build()?;
  let h = [("Accept-Language", "ja,en-US;q=0.9,en;q=0.8")];
  let hdrs = mk_headers(&h);
  Runtime::new()?.block_on((async || -> Result<(), Box<dyn Error>> {
    let resp = client.post(url_post).headers(hdrs).multipart(mp).send().await?;
    println!("Status: {}", resp.status()); // 200 412 500
    // println!("{:?}", resp.bytes().await?); // may be compressed
    // println!("{}", resp.text().await?);
    println!("{}", resp.error_for_status()?.text().await?); // proc 412 500
    Ok(())
  })())
}

/// rstk_test
pub fn rstk_test(rt: &TkTopLevel, t: &str) -> Result<(), Box<dyn Error>> {
  let w = rstk::make_toplevel(rt);
  w.title(t);
  // w.geometry(800, 600, 200, 150); // 800x600+200+150
  rstk::tell_wish(&format!("wm geometry {} 800x600", w.id())); // +200+150
  rstk::tell_wish("update idletasks"); // force change geometry to ask_wish
  let lbl = rstk::make_label(&w);
  lbl.text(t);
  lbl.grid().layout();

  // let url = "http://127.0.0.1:8080/"; // test with resource/ua_server.py
  let url = "https://www.google.com/search"; // https OK
  let cj = Arc::new(cookie::Jar::default()); // needless to mut cj
  // add_cookies(&cj, &[("ck0", "foo"), ("ck1", "bar"), ("ck2", "baz")], url)?;
  // let client = reqwest::Client::new();
  let builder = reqwest::Client::builder().timeout(Duration::from_secs(5))
    .cookie_provider(cj).default_headers(mk_default_headers());
  let client = builder.build()?;
  let q = [("q", "reqwest"), ("ie", "UTF-8")];
  let h = [("Custom-Header", "custom header")];
  let b = ""; // set body when post
  assert_eq!(reqwest_test(&client, url, &q, mk_headers(&h), b)?, ());

  // This test places a load on the target; use it with caution.
  // assert_eq!(uploader_catbox_litterbox("resource/ruby_200x200.png")?, ());

  let q = format!("wm geometry {}", w.id());
  // let g = rstk::ask_wish(&q); // not flushed (catch another output)
  let g = rstk::ask_wish(&format!("puts [{}]; flush stdout", q)); // safely
  println!("geom: {} {} {}", g, w.id(), t);
  Ok(())
}

/// lib_test
pub fn lib_test() -> Result<(), Box<dyn Error>> {
  let rt = rstk::start_wish().expect("wish");
  assert_eq!(rstk_test(&rt, "test lib")?, ());
  rstk::mainloop();
  Ok(())
}
