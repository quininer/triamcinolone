// use box syntax, but unstable feature
// #![feature(box_syntax)]

extern crate painting;

use painting::{Messager, Events, Arguments};
use std::path::Path;

#[test]
fn test_messager() {
    let mut im = Messager::new(Path::new("examples/config.toml"));
    assert!(im.bootstrap().is_ok());

    assert_eq!(im.core.get_name(), "triam");
    assert!(im.save().is_ok());
}

#[test]
fn test_event() {
    let mut im = Messager::new(Path::new("examples/config.toml"));
    im.bootstrap().ok().expect("bootstrap failure.");

    im.on("test", Box::new(|tox, args| {
        assert_eq!(args.message, Some("Hello world.".to_string()));
        assert_eq!(tox.core.get_name(), "triam");
    }));

    assert!(im.trigger("test", Arguments {
        message: Some("Hello world.".to_string()),
        ..Default::default()
    }).is_ok());
    assert!(im.trigger("not", Arguments {
        message: Some("Event binding.".to_string()),
        ..Default::default()
    }).is_err());
}
