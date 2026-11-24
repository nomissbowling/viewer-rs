# (main) dl_image.tcl

set base_dir [file dirname [file normalize [info script]]]
source -encoding utf-8 [file join $base_dir prepare_funcs.tcl]
source -encoding utf-8 [file join $base_dir setup_toplevel.tcl]
source -encoding utf-8 [file join $base_dir dl_https.tcl]

# namespace eval my_namespace {
#   proc hoge {} {
#     ...
#   }
# }

wm geometry . 800x600
setup_toplevel . im0

toplevel .t
wm geometry .t 800x600
setup_toplevel .t im1

set uri "https://files.catbox.moe/riqo8l.png"
puts [set_img_uri [idchild . .lbl] im0 $uri]
puts [set_img_uri [idchild .t .lbl] im1 $uri]
