// use box syntax, but unstable feature
// #![feature(box_syntax)]

extern crate painting;
extern crate rstox;

use std::collections::HashMap;
use painting::{Messager, Events, Arguments, Operate, GroupOperate};
use std::path::Path;
use rstox::core::*;

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

    let mut events = HashMap::new();

    events.on("test", Box::new(|tox, args| {
        assert_eq!(args.message, Some("Hello world.".to_string()));
        assert_eq!(tox.core.get_name(), "triam");
    }));

    assert!(events.trigger(&mut im, "test", Arguments {
        message: Some("Hello world.".to_string()),
        ..Arguments::default()
    }).is_ok());
    assert!(events.trigger(&mut im, "not", Arguments {
        message: Some("Event binding.".to_string()),
        ..Arguments::default()
    }).is_err());
}

#[test]
#[should_panic]
fn test_group() {
    let mut im = Messager::new(Path::new("examples/config.toml"));
    let mut events = HashMap::new();
    im.bootstrap().ok().expect("Bootstrap failure.");

    events.on("connection", Box::new(|tox, args| {
        tox.join(GroupchatType::Text, None, None).ok().expect("Group create fail.")
            .send(None, MessageType::Normal, "Hello world.".to_string()).ok().expect("Group Message send fail.");
    }));

    events.on("group.message", Box::new(|tox, args| {
        assert_eq!(args.message, Some("Hello world.".to_string()));
        assert_eq!(tox.group(args.groupnum).get_nick(args.peer), tox.get_nick(None));

        // XXX 应该给eloop加一个 break 的方法
        assert!(false);
    }));

    events.eloop(&mut im);
}
