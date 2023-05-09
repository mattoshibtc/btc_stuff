use std::env;
use std::io::Read;
use std::net::TcpStream;
use ssh2::Session;

pub(crate) fn get_session() -> Session {
    let host = env::var("BITCOIN_NODE_ADDRESS").unwrap();
    let username = env::var("BITCOIN_NODE_USERNAME").unwrap();
    let password = env::var("BITCOIN_NODE_PASSWORD").unwrap();

    let tcp = TcpStream::connect(format!("{}:22", host)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password(&username, &password).unwrap();
    assert!(sess.authenticated());
    sess
}

pub(crate) fn execute_command(sess: &Session, command: &str) -> String {
    let mut channel = sess.channel_session().unwrap();
    channel.exec(command).unwrap();
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    output
}