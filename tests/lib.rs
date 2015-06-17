// use box syntax, but unstable feature
// #![feature(box_syntax)]

extern crate triamcinolone;

use triamcinolone::{Messager, Events};
use std::path::Path;

#[test]
fn test_messager() {
    let mut im = Messager::new(Path::new("examples/config.toml"));
    assert!(im.bootstrap().is_ok());

    assert_eq!(im.core.get_name(), "triam");
    assert_eq!(im.save().is_ok(), true);
}

#[test]
fn test_event() {
    let mut im = Messager::new(Path::new("examples/config.toml"));
    im.bootstrap().ok().expect("bootstrap failure.");

    im.on("test", Box::new(|tox, arguments| {
        assert_eq!(arguments, "Hello world.".to_string());
        assert_eq!(tox.core.get_name(), "triam");
    }));

    assert!(im.trigger("test", "Hello world.".to_string()).is_ok());
    assert!(match im.trigger("not", "Event".to_string()) {
        Ok(_) => false,
        Err(_) => true
    });
}
