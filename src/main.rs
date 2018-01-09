extern crate ssh2;
extern crate clap;

use std::net::TcpStream;
use std::error::Error;
use std::io::prelude::*;
use std::thread;
use ssh2::{Session, Identities, Agent, PublicKey};
use clap::{Arg, App};

mod errors;
mod types;

use types::Server;
use errors::AuthenticationError;

fn create_tcp_stream(server: &Server) -> TcpStream {
    let host_string = server.host.to_string() + &":" + &server.port.to_string();
    TcpStream::connect(host_string).unwrap()
}

fn authenticate(username: &str, server: &Server, session: &Session) -> Result<bool, AuthenticationError> {
    let mut agent = session.agent().unwrap();

    agent.connect().unwrap();
    agent.list_identities().unwrap();

    let mut authenticated = false;

    for identity in agent.identities() {
        agent.userauth(username, &identity.unwrap());

        authenticated = session.authenticated();

        if authenticated {
            break;
        }
    }

    if authenticated {
        Ok(true)
    } else {
        Err(AuthenticationError { host: (*server.host).to_string() })
    }
}

fn main() {
    let app = App::new("Marionette")
        .version("0.1.0")
        .author("Dave Benvenuti <davebenvenuti@gmail.com>")
        .about("Execute commands on one or more servers via ssh")
        .arg(Arg::with_name("host")
             .short("h")
             .long("host")
             .value_name("HOST")
             .help("One or more hosts, comma delimited, eg: server1.com:22,server2.com:22")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("username")
             .short("u")
             .long("username")
             .value_name("USERNAME")
             .help("Username to ssh as")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("command")
             .short("c")
             .long("command")
             .value_name("COMMAND")
             .help("Command to issue")
             .required(true)
             .takes_value(true));

    let matches = app
        .get_matches();

    let host_string: String = matches.value_of("host").unwrap().to_string();

    let create_server = |host: &str| {
        let server = Server::from_str(host);

        match server {
            Ok(server) => server,
            Err(e) => {
                eprintln!("{}", e.description());
                eprintln!("{}", matches.usage());

                ::std::process::exit(1);
            }
        }
    };

    let servers = host_string.split(",").map(create_server);

    let threads = servers.map(|server| {
        let username = String::from(matches.value_of("username").unwrap());
        let command = String::from(matches.value_of("command").unwrap());

        thread::spawn(move || {
            let mut session = Session::new().unwrap();

            let tcp = create_tcp_stream(&server);
            session.handshake(&tcp).unwrap();

            match authenticate(&username, &server, &session) {
                Ok(_) => {
                    let mut channel = session.channel_session().unwrap();
                    channel.exec(&command).unwrap();
                    let mut s = String::new();

                    channel.read_to_string(&mut s).unwrap();

                    println!("************[{}] ************\n{}", server.host, s);
                    channel.wait_close();
                    println!("    exit status: {}", channel.exit_status().unwrap());
                },
                Err(err) => {
                    eprintln!("{}", err.description());
                    ::std::process::exit(1);
                }
            }
        })
    });

    for thread in threads {
        thread.join();
    }
}
