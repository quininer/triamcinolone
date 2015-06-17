use lib::{Messager, Events};
use std::path::Path;

mod lib;

fn main() {
    let mut im = Messager::new(Path::new("./config.toml"));
    im.bootstrap();
    im.save().unwrap();

    println!("{}: {}", im.core.get_name(), im.core.get_address());

    im.eloop();
}
