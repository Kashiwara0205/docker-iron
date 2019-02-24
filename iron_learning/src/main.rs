extern crate iron;
extern crate staticfile;
extern crate mount;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use std::path::Path;
fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("web/test/index.html")));
    Iron::new(mount).http("0000:8080").unwrap();;
}