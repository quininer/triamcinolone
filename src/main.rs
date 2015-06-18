use lib::{Messager, Events};
use std::path::Path;

mod lib;

fn main() {
    let mut im = Messager::new(Path::new("./config.toml"));
    im.bootstrap().ok().expect("Bootstrap failure.");
    im.save().err().and_then(|err| Some(println!("Save profile failure. {}", err)));
    println!("{}: {}", im.core.get_name(), im.core.get_address());

    im.eloop();
}
