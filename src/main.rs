use lib::{Messager, Events};
use std::path::Path;

mod lib;

fn main() {
    let mut im = Messager::new(Path::new("./config.toml"));
    im.bootstrap().ok().expect("Bootstrap failure.");
    im.save().err().and_then(|err| Some(println!("Save profile failure. {}", err)));
    println!("{}: {}", im.core.get_name(), im.core.get_address());

    im.on("connection", Box::new(|tox, args| {
        println!("[connection] {} ConnectionStatus: {:?}", tox.core.get_name(), args.status);
    }));
/*
 * WTF?
 * Why can not I borrow `&mut self`?
    im.on("friend.request", box |tox, args| {
        tox.core.add_friend_norequest(&args.pk.unwrap()).unwrap();
    });
    im.on("group.invite.text", box |tox, args| {
        tox.core.join_groupchat(args.peer.unwrap(), &args.data.unwrap()).unwrap();
    });
    im.on("group.message", box |tox, args| {
        if tox.core.group_peername(args.groupnum.unwrap(), args.peer.unwrap()).unwrap() != tox.core.get_name() {
            tox.core.group_message_send(args.groupnum.unwrap(), &args.message.unwrap()).unwrap();
        };
    });
*/

    im.eloop();
}
