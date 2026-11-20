//! core
//!

use std::{result::Result, error::Error};

use rstk::{self, *};

/// rstk_test
pub fn rstk_test(rt: &TkTopLevel, t: &str) -> Result<(), Box<dyn Error>> {
  let w = rstk::make_toplevel(rt);
  let lbl = rstk::make_label(&w);
  lbl.text(t);
  lbl.grid().layout();
  Ok(())
}

/// lib_test
pub fn lib_test() -> Result<(), Box<dyn Error>> {
  let rt = rstk::start_wish().expect("wish");
  assert_eq!(rstk_test(&rt, "test lib")?, ());
  rstk::mainloop();
  Ok(())
}
