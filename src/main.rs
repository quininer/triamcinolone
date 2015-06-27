use std::collections::HashMap;
use lib::{Messager, Events};
use std::path::Path;

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
    }));
    events.on("group.invite.text", Box::new(|tox, args| {
        tox.core.join_groupchat(args.peer.unwrap(), &args.data.unwrap()).unwrap();
    }));
    events.on("group.message", Box::new(|tox, args| {
        if tox.core.group_peername(args.groupnum.unwrap(), args.peer.unwrap()).unwrap() != tox.core.get_name() {
            tox.core.group_message_send(args.groupnum.unwrap(), &args.message.unwrap()).unwrap();
        };
    }));

    events.eloop(&mut im);
}
