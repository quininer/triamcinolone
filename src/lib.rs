extern crate rstox;
extern crate toml;

use self::rstox::core::*;

use std::collections::HashMap;
use self::toml::{Parser, Table};
use std::fs::File;
use std::path::Path;
use std::io::{Write, Read, Error};

// pub struct Args {
    // //
// }

pub struct Messager<'e> {
    pub core: Tox,
    //av: ToxAV,
    config: Table,
    owner: PublicKey,
    events: HashMap<&'e str, Vec<Box<Fn(&Messager, String)>>>
}

impl<'e> Messager<'e> {
    pub fn new(path: &Path) -> Messager {
        // load toml config
        let mut data = String::new();
        File::open(path).unwrap().read_to_string(&mut data).unwrap();
        let xconfig = Parser::new(&data).parse().unwrap();

        // load Tox Profile
        let mut xp: Vec<u8> = Vec::new();
        let xcore = match Tox::new(ToxOptions::new(), match File::open(Path::new(
                xconfig.get("bot").unwrap().as_table().unwrap()
                    .get("profile").and_then(|x| x.as_str()).unwrap_or("./profile.tox")
            )) {
                Ok(mut data) => {
                    data.read_to_end(&mut xp).ok().expect("Open Profile Error.");
                    Some(&xp)
                },
                Err(_) => None
            }
        ) {
            Ok(core) => core,
            Err(err) => panic!("Tox InitError: {:?}", err)
        };

        Messager {
            core: xcore,
            config: xconfig.clone(),
            owner: xconfig.get("bot").unwrap().as_table().unwrap()
                .get("owner").unwrap().as_str().unwrap().parse().unwrap(),
            events: HashMap::new()
        }
    }

    pub fn bootstrap(&mut self) -> Result<(), errors::BootstrapError> {
        self.core.set_name(
            self.config.get("bot").unwrap().as_table().unwrap()
                .get("name").unwrap().as_str().unwrap()
        ).unwrap();
        self.core.set_status_message(
            self.config.get("bot").unwrap().as_table().unwrap()
                .get("status").and_then(|x| x.as_str()).unwrap_or("A4.")
        ).unwrap();
        self.core.bootstrap(
            self.config.get("bootstrap").unwrap().as_table().unwrap()
                .get("ip").unwrap().as_str().unwrap(),
            self.config.get("bootstrap").unwrap().as_table().unwrap()
                .get("port").unwrap().as_integer().unwrap() as u16,
            self.config.get("bootstrap").unwrap().as_table().unwrap()
                .get("key").unwrap().as_str().unwrap().parse().unwrap()
        )
    }

    pub fn save(&mut self) -> Result<(), Error> {
        let mut f = try!(File::create(Path::new(
            self.config.get("bot").unwrap().as_table().unwrap()
                .get("profile").and_then(|x| x.as_str()).unwrap_or("./profile.tox")
        )));
        try!(f.write(&self.core.save()));
        Ok(())
    }
}

pub trait Events<'e> {
    fn eloop(&mut self);
    fn on(&mut self, event: &'e str, f: Box<Fn(&Messager, String)>);
    fn trigger(&mut self, event: &str, arguments: String) -> Result<(), EventError>;
}

pub enum EventError {
    None = 0,
    NotFoundEvent
}

impl<'e> Events<'e> for Messager<'e> {
    fn on(&mut self, event: &'e str, foo: Box<Fn(&Messager, String)>) {
        let mut l = self.events.entry(event).or_insert(Vec::new());
        l.push(foo);
    }
    fn trigger(&mut self, event: &str, arguments: String) -> Result<(), EventError> {
        match self.events.get(event) {
            Some(l) => {
                for foo in l {
                    foo(self, arguments.clone());
                };
                Ok(())
            },
            None => Err(EventError::NotFoundEvent)
        }
    }

    fn eloop(&mut self) {
        loop {
            for ev in self.core.iter() {
                match ev {
                    FriendRequest(cid, _) => {
                        self.core.add_friend_norequest(&cid).unwrap();
                    },
                    GroupInvite(fid, kind, data) => {
                        match kind {
                            GroupchatType::Text => { self.core.join_groupchat(fid, &data).unwrap(); },
                            _ => {},
                        }
                    },
                    GroupMessage(group, peer, msg) => if self.core.group_peername(group, peer).unwrap() != self.core.get_name() {
                        self.core.group_message_send(group, &msg).unwrap();
                    },
                    ev => println!("Tox event: {:?}", ev),
                }
            };

            self.core.wait();
        };
    }
}
