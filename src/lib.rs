//! High-level API package is rstox

extern crate rstox;
extern crate toml;

use self::rstox::core::*;

use std::collections::HashMap;
use self::toml::{Parser, Table};
use std::fs::File;
use std::path::Path;
use std::io::{Write, Read, Error};

/// Tox Messager.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use std::path::Path;
/// use painting::{Messager, Events};
///
/// let mut im = Messager::new(Path::new("examples/config.toml"));
/// let mut events = HashMap::new();
/// im.bootstrap().ok().expect("Bootstrap failure.");
/// im.save().err().and_then(|err| Some(println!("Save profile failure. {}", err)));
///
/// events.on("connection", Box::new(|tox, args| {
///     println!("{} ConnectionStatus: {:?}", tox.core.get_name(), args.status);
/// }));
///
/// //events.eloop(&mut im);
/// ```
pub struct Messager {
    /// Tox Messager core
    pub core: Tox,
    //av: ToxAV,
    /// Bootstrap config
    bootstrap: Table,
    /// Bot config
    pub config: Table,
    /// Bot owner public ey
    pub owner: PublicKey,
}

impl Messager {
    /// New a Messager
    pub fn new(path: &Path) -> Self {
        // load toml config
        let mut data = String::new();
        File::open(path).ok().expect("you need `config.toml`.").read_to_string(&mut data).unwrap();
        let xconfig = Parser::new(&data).parse().unwrap();

        // load Tox profile
        let mut xp: Vec<u8> = Vec::new();
        let xcore = match Tox::new(ToxOptions::new(), match File::open(Path::new(
                xconfig.get("bot").unwrap().as_table().unwrap()
                    .get("profile").and_then(|x| x.as_str()).unwrap_or("./profile.tox")
            )) {
                Ok(mut data) => {
                    data.read_to_end(&mut xp).ok().expect("Error while loading profile.");
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
            bootstrap: xconfig.get("bootstrap").expect("missing bootstrapd config.").as_table().unwrap().clone(),
            config: xconfig.get("bot").expect("minssing bot config.").as_table().unwrap().clone(),
            owner: xconfig.get("bot").unwrap().as_table().unwrap()
                .get("owner").unwrap().as_str().unwrap().parse().unwrap()
        }
    }

    /// Bootstrap Tox network
    pub fn bootstrap(&mut self) -> Result<(), errors::BootstrapError> {
        self.core.set_name(
            self.config.get("name").expect("missing bot name.")
                .as_str().unwrap()
        ).unwrap();
        self.core.set_status_message(
            self.config.get("status").and_then(|x| x.as_str()).unwrap_or("A4.")
        ).unwrap();
        self.core.bootstrap(
            self.bootstrap.get("ip").and_then(|x| x.as_str()).unwrap_or("127.0.0.1"),
            self.bootstrap.get("port").and_then(|x| x.as_integer()).unwrap_or(33445) as u16,
            self.bootstrap.get("key").expect("missing bootstrapd key.")
                .as_str().unwrap().parse().unwrap()
        )
    }

    /// Save Tox profile
    pub fn save(&mut self) -> Result<(), Error> {
        let mut f = try!(File::create(Path::new(
            self.config.get("profile").and_then(|x| x.as_str()).unwrap_or("./profile.tox")
        )));
        try!(f.write(&self.core.save()));
        Ok(())
    }
}

// pub trait Operate {
    // fn send();
    // fn get_nick();
// }

/// Event arguments
///
/// # Examples
///
/// ```
/// let args = painting::Arguments {
///     message: Some("Hello world!".to_string()),
///     ..Default::default()
/// };
///
/// assert_eq!(args.message, Some("Hello world!".to_string()));
/// ```
#[derive(Clone)]
pub struct Arguments {
    ///  Connection Status
    pub status: Option<Connection>,
    ///  Friend num
    pub fnum: Option<u32>,
    ///  Message String
    pub message: Option<String>,
    ///  Public Key
    pub pk: Option<PublicKey>,
    ///  Data
    pub data: Option<Vec<u8>>,
    ///  Group peer num
    pub peer: Option<i32>,
    ///  Group num
    pub groupnum: Option<i32>,
    ///  Friend Group num
    pub fgnum: Option<i32>,
    ///  Group change
    pub change: Option<ChatChange>
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            status: None,
            fnum: None,
            message: None,
            pk: None,
            data: None,
            peer: None,
            groupnum: None,
            fgnum: None,
            change: None
        }
    }
}

/// Events loop
pub trait Events<'e> {
    fn on(&mut self, event: &'e str, foo: Box<Fn(&mut Messager, Arguments)>);
    fn trigger(&mut self, im: &mut Messager, event: &str, arguments: Arguments) -> Result<(), EventError>;
    fn eloop(&mut self, im: &mut Messager);
}

pub enum EventError {
    NotFoundEvent
}

impl<'e> Events<'e> for HashMap<&'e str, Vec<Box<Fn(&mut Messager, Arguments)>>> {
    /// Register event
    fn on(&mut self, event: &'e str, foo: Box<Fn(&mut Messager, Arguments)>) {
        let mut l = self.entry(event).or_insert(Vec::new());
        l.push(foo);
    }

    /// Trigger event
    fn trigger(&mut self, im: &mut Messager, event: &str, arguments: Arguments) -> Result<(), EventError> {
        match self.get(event) {
            Some(l) => {
                for foo in l {
                    foo(im, arguments.clone());
                };
                Ok(())
            },
            None => Err(EventError::NotFoundEvent)
        }
    }

    /// Start events loop
    #[allow(unused_must_use)]
    fn eloop(&mut self, im: &mut Messager) {
        loop {
            for ev in im.core.iter() {
                println!("Event {:?}", ev);
                match ev {
                    ConnectionStatus(status) => {
                        self.trigger(im, "connection", Arguments {
                            status: Some(status),
                            ..Arguments::default()
                        });
                    },
                    FriendRequest(pk, message) => {
                        self.trigger(im, "friend.request", Arguments {
                            pk: Some(pk),
                            message: Some(message),
                            ..Arguments::default()
                        });
                    },
                    FriendMessage(fnum, kind, message) => {
                        self.trigger(im, &format!("friend.{}", match kind {
                            MessageType::Normal => "message",
                            MessageType::Action => "action"
                        }), Arguments {
                            fnum: Some(fnum),
                            message: Some(message),
                            ..Arguments::default()
                        });
                    },
                    LossyPackage(fnum, data) => {
                        self.trigger(im, "package.lossy", Arguments {
                            fnum: Some(fnum),
                            data: Some(data),
                            ..Arguments::default()
                        });
                    },
                    LosslessPackage(fnum, data) => {
                        self.trigger(im, "package.lossless", Arguments {
                            fnum: Some(fnum),
                            data: Some(data),
                            ..Arguments::default()
                        });
                    },
                    GroupInvite(peer, kind, data) => {
                        self.trigger(im, &format!("group.invite.{}", match kind {
                            GroupchatType::Text => "text",
                            GroupchatType::Av => "av"
                        }), Arguments {
                            peer: Some(peer),
                            data: Some(data),
                            ..Arguments::default()
                        });
                    },
                    GroupMessage(groupnum, peer, message) => {
                        self.trigger(im, "group.message", Arguments {
                            groupnum: Some(groupnum),
                            peer: Some(peer),
                            message: Some(message),
                            ..Arguments::default()
                        });
                    },
                    GroupTitle(groupnum, peer, message) => {
                        self.trigger(im, "group.title", Arguments {
                            groupnum: Some(groupnum),
                            peer: Some(peer),
                            message: Some(message),
                            ..Arguments::default()
                        });
                    },
                    GroupNamelistChange(groupnum, peer, change) => {
                        self.trigger(im, "group.change", Arguments {
                            groupnum: Some(groupnum),
                            peer: Some(peer),
                            change: Some(change),
                            ..Arguments::default()
                        });
                    },
                }
            };

            im.core.wait();
        };
    }
}
