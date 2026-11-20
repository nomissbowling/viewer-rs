//! core
//!

use std::{result::Result, error::Error};

use rstk::{self, *};

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
