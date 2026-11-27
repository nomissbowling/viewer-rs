//! core
//!

use crate::{ut, act::*};

use std::fs;
use std::{result::Result, error::Error};
use std::time::Duration;
use std::sync::Arc;

use wktk::{core::*, wk};
use rstk::{self, *};
use reqwest::{self, multipart, cookie, header, Client};
use tokio::runtime::Runtime;

/// reqwest_get_test
pub fn reqwest_get_test(client: &Client, url: &str, q: &[(&str, &str)],
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

/// reqwest_builder_test
pub fn reqwest_builder_test() -> Result<(), Box<dyn Error>> {
  // let url = "http://127.0.0.1:8080/"; // test with resource/ua_server.py
  let url = "https://www.google.com/search"; // https OK
  let cj = Arc::new(cookie::Jar::default()); // needless to mut cj
  // ut::cookies(&cj, &[("ck0", "foo"), ("ck1", "bar"), ("ck2", "baz")], url)?;
  // let client = reqwest::Client::new();
  let builder = reqwest::Client::builder().timeout(Duration::from_secs(5))
    .cookie_provider(cj).default_headers(ut::mk_default_headers());
  let client = builder.build()?;
  let q = [("q", "reqwest"), ("ie", "UTF-8")];
  let h = [("Custom-Header", "custom header")];
  let b = ""; // set body when post
  reqwest_get_test(&client, url, &q, ut::mk_headers(&h), b)
}

/// uploader_catbox_litterbox
/// - This test places a load on the target; use it with caution.
// (see also ________github_memo.txt)
pub fn uploader_catbox_litterbox(src: &str) -> Result<(), Box<dyn Error>> {
  let dst = format!("unknown{}", ut::splitext(src));
  let mt = ut::guess_type(src);
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
    .cookie_provider(cj).default_headers(ut::mk_default_headers());
  let client = builder.build()?;
  let h = [("Accept-Language", "ja,en-US;q=0.9,en;q=0.8")];
  let hdrs = ut::mk_headers(&h);
  Runtime::new()?.block_on((async || -> Result<(), Box<dyn Error>> {
    let resp = client.post(url_post).headers(hdrs).multipart(mp).send().await?;
    println!("Status: {}", resp.status()); // 200 412 500
    // println!("{:?}", resp.bytes().await?); // may be compressed
    // println!("{}", resp.text().await?);
    println!("{}", resp.error_for_status()?.text().await?); // proc 412 500
    Ok(())
  })())
}

/// uploader_test
pub fn uploader_test(_rt: &TkTopLevel) -> Result<(), Box<dyn Error>> {
  assert_eq!(reqwest_builder_test()?, ());
/*
  // This test places a load on the target; use it with caution.
  let res = "resource/ruby_200x200.png";
  let wd = std::env::var("TCL_WKTK").map_or_else(|_err| {
    ".".into()
  }, |tcl_wktk| tcl_wktk);
  let p = format!("{}/{}", wd, res);
  assert_eq!(uploader_catbox_litterbox(&p)?, ());
*/
  Ok(())
}

/// lib_test
pub fn lib_test() -> Result<(), Box<dyn Error>> {
  let rt = wish_toplevel(320, 240)?;
  assert_eq!(rstk_test(&rt, "test lib")?, ());
  assert_eq!(uploader_test(&rt)?, ());
  wk::topmost(&rt)?;
  rstk::mainloop();
  let act: Act = serde_json::from_str(r#"{"id":"abc","list":["wx","yz"]}"#)?;
  assert_eq!(format!("{}", &act), r#"{"act":"abc","lst":["wx","yz"]}"#);
  Ok(())
}
