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

use serde_json;
use serde::{Serialize, Deserialize};

/// Act
#[derive(Serialize, Deserialize, Debug)]
pub struct Act {
  /// act
  #[serde(alias = "id")]
  pub act: String,
  /// lst
  #[serde(alias = "list")]
  pub lst: Vec<String>
}

/// disp
pub fn disp(act: &Act) -> Result<(), Box<dyn Error>> {
  println!("{}", serde_json::to_string(act)?);
  let a: Act = serde_json::from_str(r#"{"act": "abc", "lst": ["wx", "yz"]}"#)?;
  println!("{}", serde_json::to_string(&a)?);
  Ok(())
}

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
  // add_cookies(&cj, &[("ck0", "foo"), ("ck1", "bar"), ("ck2", "baz")], url)?;
  // let client = reqwest::Client::new();
  let builder = reqwest::Client::builder().timeout(Duration::from_secs(5))
    .cookie_provider(cj).default_headers(mk_default_headers());
  let client = builder.build()?;
  let q = [("q", "reqwest"), ("ie", "UTF-8")];
  let h = [("Custom-Header", "custom header")];
  let b = ""; // set body when post
  reqwest_get_test(&client, url, &q, mk_headers(&h), b)
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

/// downloader
/// - test with resource/dl_https.tcl
pub fn downloader(w: &impl TkWidget, uri: &str) -> Result<(), Box<dyn Error>> {
  let l = ask_safely(&format!(r#"set_img_uri [idchild {} {}] {} "{}""#,
    w.id(), ".lbl", "$img", uri)); // $img: global
  println!("length: {}", l);
  Ok(())
}

/// rstk_test
pub fn rstk_test(rt: &TkTopLevel, t: &str) -> Result<(), Box<dyn Error>> {
  let w = mk_toplevel_with_id(rt, &mk_id(t));
  w.title(t);
  let ((x, y), z) = ((200, 150), (800, 600));
  // re-position before first geometry
  let (x, y) = if t.ends_with("test") { (x / 2, y / 2) } else { (x, y) };
  w.geometry(z.0, z.1, x, y);
  // re-set size after first geometry
  let z = if t.ends_with("test") { (z.0 / 2, z.1 / 2) } else { z };
/*
  rstk::tell_wish(&format!("wm geometry {} {}x{}", w.id(), z.0, z.1)); // +x+y
  rstk::tell_wish("update idletasks"); // force change geometry to ask_wish
  let g = ask_safely(&format!("wm geometry {}", w.id()));
*/
  let g = ask_safely(&format!("wm_size {} {} {}", w.id(), z.0, z.1));
  println!("geometry: {} {} {}", g, w.id(), t);
  let p = if t.ends_with("lib") { "center" }
    else { if t.ends_with("test") { "lb" } else { "rt" } };
  let btn = mk_button_with_id(&w, ".mybtn");
  btn.text(p);
  btn.grid().layout();
  let lbl = mk_label_with_id(&w, ".mylbl");
  lbl.text(t);
  lbl.grid().layout();

//  let title = t.to_string(); // to keep lifetime (used w finialy before move)
  btn.clone().command(move || { // must clone btn before move
/*
    let g = ask_safely(&format!("wm_loc {} {}", w.id(), btn.cget("text")));
    println!("geometry: {} {} {}", g, w.id(), title);
*/
    let g = ask_safely(&format!("on_btn_clicked {} {}", btn.id(), w.id()));
    println!("geometry: {}", g);
  });

  assert_eq!(reqwest_builder_test()?, ());

  assert_eq!(downloader(rt, "https://files.catbox.moe/riqo8l.png")?, ());

  // This test places a load on the target; use it with caution.
  // assert_eq!(uploader_catbox_litterbox("resource/ruby_200x200.png")?, ());
  Ok(())
}

/// topmost
pub fn topmost(w: &TkTopLevel) -> Result<(), Box<dyn Error>> {
  let c = format!("wm attributes {0} -topmost 1; raise {0}; focus -force {0}",
    w.id());
  rstk::tell_wish(&c);
  Ok(())
}

/// wish_toplevel
pub fn wish_toplevel(w: u64, h: u64) -> Result<TkTopLevel, Box<dyn Error>> {
  rstk::start_wish().map_or(
    Err("Can not start wish. Tcl/Tk is not installed ?".into()),
    |rt| { // place toplevel root window to right bottom (no care multi screen)
/*
      let scw = ask_safely("winfo screenwidth .").parse::<u64>()?;
      let sch = ask_safely("winfo screenheight .").parse::<u64>()?;
      rt.geometry(w, h, (scw - w).try_into()?, (sch - h).try_into()?);
*/
      prepare_funcs();
      let scwh = ask_safely(&format!("wm_sizeloc {} {} {} rb", rt.id(), w, h));
      println!("desktop: {}", scwh);
      setup_toplevel(&rt);
      Ok(rt)
    }
  )
}

/// prepare_funcs
pub fn prepare_funcs() {
  rstk::tell_wish(r#"
set base_dir [file dirname [file normalize [info script]]]
source -encoding utf-8 [file join $base_dir resource prepare_funcs.tcl]

proc on_btn_clicked {bid id} {
  set nb [new_btn $id {my button} [list {on_btn_clicked} $bid $id]]
  return [list [wm_loc $id [$bid cget -text]] $id [wm title $id] $nb]
}

proc new_btn {id t c} {
  global num
  set bid [expr {"$id.mybtn_$num"}]
  ttk::button $bid -text [expr {"${t}_$num"}] -command $c
  # must match append style pack/grid on the parent
  # pack $bid -padx 10 -pady 10
  grid $bid -column 0 -padx 10 -pady 10
  set num [expr {$num + 1}]
  return $bid
}

set num 0
"#);
  rstk::tell_wish(r#"proc dumb_double_quote {} { return '"' }"#);
}

/// setup_toplevel
pub fn setup_toplevel(rt: &TkTopLevel) {
  rstk::tell_wish(r#"
source -encoding utf-8 [file join $base_dir resource setup_toplevel.tcl]
source -encoding utf-8 [file join $base_dir resource dl_https.tcl]
"#);
  rstk::tell_wish(r#"proc dumb_double_quote {} { return '"' }"#); // overwrite
  rstk::tell_wish(&format!("set img [setup_toplevel {}]", rt.id())); // global
}

/// mk_id
pub fn mk_id(t: &str) -> String {
  format!(".{}", t.replace(" ", "_"))
}

/// mk_with_checked_id
pub fn mk_with_checked_id(p: &impl TkWidget, id: &str, w: &str) -> String {
  let wid = if id.starts_with(".") { id }
    else { eprintln!("{} id forget [.]{}", w, id); &format!(".{}", id) };
  ask_safely(&format!("{} [idchild {} {}]", w, p.id(), wid))
}

/// mk_toplevel_with_id
pub fn mk_toplevel_with_id(p: &impl TkWidget, id: &str) -> TkTopLevel {
  TkTopLevel{id: mk_with_checked_id(p, id, "toplevel")}
}

/// mk_frame_with_id
pub fn mk_frame_with_id(p: &impl TkWidget, id: &str) -> TkFrame {
  TkFrame{id: mk_with_checked_id(p, id, "ttk::frame")}
}

/// mk_button_with_id
pub fn mk_button_with_id(p: &impl TkWidget, id: &str) -> TkButton {
  TkButton{id: mk_with_checked_id(p, id, "ttk::button")}
}

/// mk_label_with_id
pub fn mk_label_with_id(p: &impl TkWidget, id: &str) -> TkLabel {
  TkLabel{id: mk_with_checked_id(p, id, "ttk::label")}
}

/// ask_safely
pub fn ask_safely(q: &str) -> String {
  // rstk::ask_wish(&q) // not flushed (catch another output)
  rstk::ask_wish(&format!("puts [{}]; flush stdout", q)) // safely
}

/// lib_test
pub fn lib_test() -> Result<(), Box<dyn Error>> {
  let rt = wish_toplevel(320, 240)?;
  assert_eq!(rstk_test(&rt, "test lib")?, ());
  topmost(&rt)?;
  rstk::mainloop();
  assert_eq!(disp(&Act{act: "xyz".into(), lst: vec!["ab".into(), "cd".into()]})?, ());
  Ok(())
}
