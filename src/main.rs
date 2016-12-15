extern crate ssh2;
extern crate rusoto;

use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;
use std::io::{Read, BufReader};
use rusoto::{ChainProvider, Region};
use rusoto::s3::S3Helper;

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
  match sftp.readdir(Path::new("/")) {
    Ok(list) => {
      for (name, _) in list {
        println!("{:?}", name);
      }
    },
    Err(err) => {
      println!("Reddir error: {:?}", err);
      return;
    }
  }
  let fhandler = sftp.open(Path::new("/readme.txt")).unwrap();
  let mut br = BufReader::with_capacity(1024, fhandler);
  let mut buffer = String::new();
  match br.read_to_string(&mut buffer) {
    Ok(_) => println!("{}", buffer),
    Err(_) => (),
  }
  //aws
  let provider = ChainProvider::new();
  let region = Region::UsWest2;
  let s3 = S3Helper::new(provider, region);
  match s3.put_object("purchase.guru", "upload/test.txt", "test".as_bytes()) {
    Ok(_) => println!("S3: OK"),
    Err(err) => println!("S3 error: {}", err)
  }
  //
  println!("Ok, try one more time :)");
}
