package require Tk
package require http
package require tls

::http::register https 443 [list ::tls::socket -autoservername true]

proc dl_https {uri} {
  set token [::http::geturl $uri]
  set dat [::http::data $token]
  ::http::cleanup $token
  return $dat
}

proc set_img_uri {lbl img uri} {
  $lbl configure -text $uri
  return [set_img_dat $img [dl_https $uri]]
}

proc set_img_dat {img dat} {
  $img configure -data $dat
  return [string bytelength $dat]
}

proc idchild {id c} {
  # set w [if {$id eq {.}} { expr {""} } else { expr {$id} }]
  set w [string trimright $id {.}]
  return [expr {"${w}${c}"}]
}

proc setup_toplevel {id} {
  set lbl [idchild $id .lbl]
  # set img [image create photo -data $dat]
  set img [image create photo]
  label $lbl -image $img -text {sample} -compound bottom
  place $lbl -relx 0.5 -rely 0.5 -anchor center
  # pack $lbl -expand 1
  # grid $lbl
  # grid $lbl -row 0 -column 0
  # grid rowconfigure $id 0 -weight 1
  # grid columnconfigure $id 0 -weight 1
  # $lbl configure -image $img
  # $lbl configure -image {}
  return $img
}

# namespace eval my_namespace {
#   proc hoge {} {
#     ...
#   }
# }

wm geometry . 800x600
# set img [setup_toplevel .]
toplevel .t
wm geometry .t 800x600
set img [setup_toplevel .t]

puts [set_img_uri [idchild .t .lbl] $img "https://files.catbox.moe/riqo8l.png"]
