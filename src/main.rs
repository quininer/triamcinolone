extern crate rstox;

use std::collections::HashMap;
use lib::{Messager, Events, Operate, GroupOperate};
use std::path::Path;
use rstox::core::*;

mod lib;

fn main() {
    let mut im = Messager::new(Path::new("./config.toml"));
    let mut events = HashMap::new();
    im.bootstrap().ok().expect("Bootstrap failure.");
    im.save().err().and_then(|err| Some(println!("Save profile failure. {}", err)));
    println!("{}: {}", im.core.get_name(), im.core.get_address());

    events.on("connection", Box::new(|tox, args| {
        println!("[connection] {} ConnectionStatus: {:?}", tox.core.get_name(), args.status);
    }));

    events.on("friend.request", Box::new(|tox, args| {
        tox.core.add_friend_norequest(&args.pk.unwrap()).unwrap();
        tox.save().err().and_then(|err| Some(println!("Save profile failure. {}", err)));
    }));
    events.on("group.invite.text", Box::new(|tox, args| {
        tox.join(GroupchatType::Text, args.peer, args.data).err().and_then(|err| {
            Some(println!("Join Group Error. {:?}", err))
        });
    }));
    events.on("group.message", Box::new(|tox, args| {
        if tox.group(args.groupnum).get_nick(args.peer) != tox.get_nick(None) {
            tox.group(args.groupnum).send(None, MessageType::Normal, args.message.unwrap_or(String::new())).err().and_then(|err| {
                Some(println!("Group Message Send Error. {:?}", err))
            });
        };
    }));

    events.eloop(&mut im);
}
