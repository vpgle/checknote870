// use std::str;
#[warn(unused_imports)]
use std::io::{Cursor, Read, Write, Stdin};
use std::io::prelude::*;
use std::fs::File;
use suppaftp::FtpStream;
use suppaftp::types::FileType;
// use zip::read::ZipFile;
use std::env;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

// fn main() {
//     let _ = ftp();
// //  std::process::exit(real_main());
// }

pub fn ftp(filelist: Vec<String>) -> std::io::Result<()> {
    // Create a connection to an FTP server and authenticate to it.
//  let mut ftp_stream = FtpStream::connect("10.209.180.227:21").unwrap();
//  let _ = ftp_stream.login("hncmtycj01", "Hnutzq#@!@2022").unwrap();

//  let root = Path::new("/opt/Gcp/gaof/ftpa");
//  assert!(env::set_current_dir(&root).is_ok());
//  println!("Successfully changed working directory to {}  !", root.display());
//  // Get the current directory that the client will be reading from and writing to.
//  let _ = ftp_stream.cwd("/S01_FTP_HXWXNSJ_0001_S").unwrap();
//  println!("Current directory: {}", ftp_stream.pwd().unwrap());

    let mut ftp_stream = FtpStream::connect("192.168.100.105:21").unwrap();
    let _ = ftp_stream.login("vuser1", "123456").unwrap();
    assert!(ftp_stream.cwd("/").is_ok());
    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    // 设置传输  二进制  传输
    assert!(ftp_stream.transfer_type(FileType::Binary).is_ok());

    // 设置下载到的本地路径
//  let root = Path::new("/opt/Gcp/gaof/ftpa");
//  assert!(env::set_current_dir(&root).is_ok());
//  println!("Successfully changed working directory to {}  !", root.display());
    // Get the current directory that the client will be reading from and writing to.
//  let _ = ftp_stream.cwd("/S01_FTP_HXWXNSJ_0001_S").unwrap();
//  println!("Current directory: {}", ftp_stream.pwd().unwrap());

    println!("filelist length is {:?}", filelist.len());

    let mut filemap: HashMap<&str, Vec<u8>> = HashMap::new();
    for filename in &filelist {
//      println!("!{}!", filename);
//      let filename = format!("\"{}\"", filename);
//      println!("|{}|", &filename);
//      let mut reader = ftp_stream.retr_as_stream(&filename).unwrap();
        println!("{:?}", &filename);
        let mut reader = ftp_stream.retr_as_buffer(&filename).map(|bytes| bytes.into_inner()).unwrap();
        println!("reader length is {:#?}", reader.len());
        filemap.insert(&filename, reader);
    }
    println!("filemap count is {}", filemap.len());

    let mut filenamebody: HashMap< &str, ( &str, Vec<u8> ) > = HashMap::new();
    for ( filename, body ) in filemap {
            
        }
    }


    // 写入到磁盘上
    let mut count = 0;
    for (filename, buffer) in filemap {
        let mut file =File::create(filename)?;
        file.write_all(&buffer)?;
        count += 1;
        println!("这是 写入到磁盘上 第{}次", count);
    }


    // Terminate the connection to the server.
    let _ = ftp_stream.quit();
    Ok(())

//  let mut contents = String::new();
//  file.read_to_string(&mut contents).unwrap();
//  println!("{contents}");

//  let mut lines = contents.as_str().lines();
//  let mut count = 0;
//  while !lines.next().is_none() {
//      count += 1;
//  }
//  println!("line is {}", count);



    // Retrieve (GET) a file from the FTP server in the current working directory.
//  let data = ftp_stream.retr_as_buffer("greeting.txt").unwrap();
//  println!("Read file with contents\n{}\n", str::from_utf8(&data.into_inner()).unwrap());

    // 创建随机文件夹成功
//  let tempdir: String = "smfdksl".to_string();
//  assert!(ftp_stream.mkdir(tempdir.as_str()).is_ok());
    // 切换目录成功
//  assert!(ftp_stream.cwd(tempdir.as_str()).is_ok());
//  println!("Current directory: {}", ftp_stream.pwd().unwrap());
//  assert!(ftp_stream.rm("greeting.txt").is_ok());
//  assert!(ftp_stream.cwd("/").is_ok());
//  println!("Current directory: {}", ftp_stream.pwd().unwrap());
//  assert!(ftp_stream.rmdir(tempdir.as_str()).is_ok());
//  assert!(ftp_stream.cwd(tempdir.as_str()).is_err());

//  let data = ftp_stream.retr_as_stream("greeting.txt").unwrap();
//  let f =    ftp_stream.finalize_retr_stream(Box::new(data)).is_ok();
//  println!("data is {:?}", data);
//  println!("f is {:?}", f);

    // Store (PUT) a file from the client to the current working directory of the server.
//  let mut reader = Cursor::new("Hello from the Rust \"ftp\" crate!".as_bytes());
//  let _ = ftp_stream.put_file("greeting.txt", &mut reader);
//  println!("Successfully wrote greeting.txt");
    // 删除是成功的
//  assert!(ftp_stream.rm("greeting.txt").is_ok());

    // Change into a new directory, relative to the one we are currently in.
//  let _ = ftp_stream.cwd("/n3").unwrap();
//  println!("Current directory: {}", ftp_stream.pwd().unwrap());


}


