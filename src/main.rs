extern crate ssh2;
extern crate reqwest;

use ssh2::Session;
use std::net::TcpStream;
use reqwest::Client;
use std::path::Path;

fn main() {
  let ftp_host = "test.rebex.net:22";
  let username = "demo";
  let password = "password";
  println!("Open TCP");
  let tcp = TcpStream::connect(ftp_host).unwrap();
  println!("Create session");
  let mut sess = Session::new().unwrap();
  println!("Handshake");
  let _ = match sess.handshake(&tcp) {
    Ok(x) => x,
    Err(err) => {
      println!("Handshake error: {:?}", err);
      return;
    }
  };
  println!("Auth");
  let _ = sess.userauth_password(username, password);
  println!("SFTP");
  let sftp = sess.sftp().unwrap();
  println!("Connected to SFTP");
  let _ = sftp.readdir(Path::new("/"));
  let client = Client::new().unwrap();
  let _ = client.post("https://aws.amazon.com/").body("Hello").send();
  println!("Ok, try one more time :)");
}
