use std::io::{Cursor, Read, Write, Stdin};
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::Path;
use std::collections::HashMap;
use suppaftp::FtpStream;
use suppaftp::types::FileType;
use regex::Regex;
use zip::result::ZipError;
use super::types::TableMaster;

// fn main() {
//     let _ = ftp();
// //  std::process::exit(real_main());
// }

pub fn ftp_regex_zip_count(filelist: Vec<String>) -> Vec<TableMaster> {
// pub fn ftp(filelist: Vec<String>) -> std::io::Result<()> {
    let mut tablemaster: Vec<TableMaster> = Vec::new();

    let mut ftp_stream = FtpStream::connect("192.168.100.105:21").unwrap();
    let _ = ftp_stream.login("vuser1", "123456").unwrap();
    assert!(ftp_stream.cwd("/").is_ok());
    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    // 设置传输  二进制  传输
    assert!(ftp_stream.transfer_type(FileType::Binary).is_ok());


    println!("filelist length is {:?}", filelist.len());

    let mut filemap: HashMap<&str, Vec<u8>> = HashMap::new();
        // struct TableMaster < raw_filename,  body>
    //         
    for filename in &filelist {
        println!("{:?}", &filename);
        let mut reader = ftp_stream.retr_as_buffer(&filename).map(|bytes| bytes.into_inner()).unwrap();
        println!("reader length is {:#?}", reader.len());
        filemap.insert(&filename, reader);
    }
    println!("filemap count is {}", filemap.len());

    let TimePrefixFilename = Regex::new(r"(^\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})_([A-Z\.\-]+)_(?<baobiaotime>([0-9a-zA-Z\.\-\_]+))00-(\d{14})_X(?<timestamp>[\d]+)\.zip").unwrap();
    let TimePrefixFilenameCsvname = Regex::new(r"(^\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})_(?<odmbaobiaotime>[0-9a-zA-Z\-\.\_]+)\-(?<stoptime_timestamp>[0-9\_X]+)\.zip").unwrap();
    let NotimePrefixFilename = Regex::new(r"([A-Z\-\.]+)_(?<baobiaotime>[A-Za-z0-9\_]+)00-(\d{14})_XG(\d{4})T(?<timestamp>\d{14})AA(\d{4})merge.zip").unwrap();
    let NotimePrefixFilenameCsvname = Regex::new(r"(?<file_name>[0-9a-zA-Z\-\.\_]+).zip").unwrap();

    //  struct TableMaster < ( raw_filename, csv_name ) , body>
    let mut filenamebody: HashMap< ( &str,  &str ) , Vec<u8>  > = HashMap::new();

    for ( filename, body ) in filemap {
       if TimePrefixFilename.is_match(filename) && TimePrefixFilenameCsvname.is_match(filename) {
           println!("\\                    ");
           println!("\\\\                  ");
           println!("\\\\\\                ");
           println!();
           println!("<<<<<<<filename -> {filename} >>>>>>>");
           let baobiaotime = format!("{}", TimePrefixFilename.replace(filename, "$baobiaotime"));
           let timestamp = format!("{}", TimePrefixFilename.replace(filename, "$timestamp"));

           // 拼接csv文件名

           let odmbaobiaotime = format!("{}", TimePrefixFilenameCsvname.replace(filename, "$odmbaobiaotime"));

           let stoptime_timestamp = format!("{}", TimePrefixFilenameCsvname.replace(filename, "$stoptime_timestamp"));

           let csv_name = format!("{odmbaobiaotime}_{stoptime_timestamp}/{odmbaobiaotime}-{stoptime_timestamp}.csv");

           let mut archive = zip::ZipArchive::new(Cursor::new(body)).unwrap();
           let mut file = archive.by_name(&csv_name).unwrap();

           let mut contents = String::new();
           file.read_to_string(&mut contents).unwrap();

           let mut lines = contents.as_str().lines();
           let mut csv_count = 0;
           while !lines.next().is_none() {
               csv_count += 1;
           }
           println!("{filename} of csv line is {}", csv_count);

           println!("csv_name is {}", csv_name);
           println!("//////                ");
           println!("////                  ");
           println!("//                    ");

          
           let a = TableMaster {
               raw_filename: filename.to_string(),
               baobiaotime: baobiaotime.to_string(),
               timestamp: timestamp.to_string().parse::<u64>().unwrap(),
               csv_name,
               csv_count,
           };
           


       } else if NotimePrefixFilename.is_match(filename) && NotimePrefixFilenameCsvname.is_match(filename) {
           println!("{}", "========== NotimePrefixFilename ==========".repeat(3));
           println!();
           println!("<<<<<<<filename -> {filename} >>>>>>>");
           let baobiaotime = NotimePrefixFilename.replace(filename, "$baobiaotime");
           let timestamp = NotimePrefixFilename.replace(filename, "$timestamp").parse::<u64>().expect("notimeprefixfilename函数，拆分timestamp报错");


           //  new中参数取自外部参数文件，如果从外部文件获取为空则设置成默认
           //
           // 文件名，取自zip压缩包名，不包括扩展名
           // csv_name也用此名，添加扩展名csv
           //
           let csv_name = format!("{}.csv", NotimePrefixFilenameCsvname.replace(filename, "$file_name"));

           let mut archive = zip::ZipArchive::new(Cursor::new(body)).unwrap();
           let mut file = archive.by_name(&csv_name).unwrap();

           let mut contents = String::new();
           file.read_to_string(&mut contents).unwrap();

           let mut lines = contents.as_str().lines();
           let mut csv_count = 0;
           while !lines.next().is_none() {
               csv_count += 1;
           }
           println!("{filename} of csv line is {}", csv_count);
           println!("{}", "========== NotimePrefixFilename ==========".repeat(3));
           println!();

           tablemaster.push(TableMaster {
                raw_filename: filename.to_string(),
                baobiaotime: baobiaotime.to_string(),
                timestamp: timestamp.to_string().parse::<u64>().unwrap(),
                csv_name,
                csv_count,
                }
           );
       }
    }



    // Terminate the connection to the server.
    let _ = ftp_stream.quit();
//  Ok(())

    return tablemaster

}




//  let mut contents = String::new();
//  file.read_to_string(&mut contents).unwrap();
//  println!("{contents}");

//  let mut lines = contents.as_str().lines();
//  let mut count = 0;
//  while !lines.next().is_none() {
//      count += 1;
//  }
//  println!("line is {}", count);


    // Create a connection to an FTP server and authenticate to it.
//  let mut ftp_stream = FtpStream::connect("10.209.180.227:21").unwrap();
//  let _ = ftp_stream.login("hncmtycj01", "Hnutzq#@!@2022").unwrap();

    // 设置下载到的本地路径
//  let root = Path::new("/opt/Gcp/gaof/ftpa");
//  assert!(env::set_current_dir(&root).is_ok());
//  println!("Successfully changed working directory to {}  !", root.display());
//  // Get the current directory that the client will be reading from and writing to.
//  let _ = ftp_stream.cwd("/S01_FTP_HXWXNSJ_0001_S").unwrap();
//  println!("Current directory: {}", ftp_stream.pwd().unwrap());

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



// use regex::Regex;
// 
// fn notimeprefixfilename() {
//     let raw = "ODM-A.WL.PM.FILE_WL_IMS_ZTE_IPSMGW_tpd_ip_smgw_q_20231014023000-20231014023015_XG1019T20231014033000AA8202merge.zip";
// 
//     // baobiaotime 获取
//     // timestamp 获取
//     //
//     let re = Regex::new(r"([A-Z\-\.]+)_(?<baobiaotime>[A-Za-z0-9\_]+)00-(\d{14})_XG(\d{4})T(?<timestamp>\d{14})AA(\d{4})merge.zip").unwrap();
//     println!("{}", re.is_match(raw));
//     let timestamp = re.replace(raw, "$timestamp").parse::<u64>().expect("notimeprefixfilename函数，拆分timestamp报错");
//     println!("timestamp is {}", timestamp);
//     let baobiaotime = re.replace(raw, "$baobiaotime");
//     println!("baobiaotime  is {}", baobiaotime );
//     println!("{}", "#".repeat(99));
// 
// 
//     //  new中参数取自外部参数文件，如果从外部文件获取为空则设置成默认
//     //
//     // 文件名，取自zip压缩包名，不包括扩展名
//     // csv_name也用此名，添加扩展名csv
//     //
//     let re = Regex::new(r"(?<file_name>[0-9a-zA-Z\-\.\_]+).zip").unwrap();
//     println!("{}", re.is_match(raw));
//     let file_name = format!("{}", re.replace(raw, "$file_name"));
//     println!("file_name is {}", file_name );
//     let csv_name = format!("{file_name}.csv");
//     println!("{csv_name}");
//     println!("{}", "---------------".repeat(4));
// 
// }
// 
// fn main() {
//     notimeprefixfilename();
//     timeprefixfilename();
// }
// 
// fn timeprefixfilename() {
//     // 只有new中的正则表达式完全匹配后raw字符串中的每个字符后，用re.is_match(raw)验证是否为true，才能成功打印其中的变量
//     //
//     let raw = "2023-10-14 06:31:43_ODM-A.WL.PM.FILE_WL_4G_HW_IMS_SBC_V4.3.0_PM_842_tpd_dns_q_20231014060000-20231014061500_X1697236205631139.zip";
// 
//     // 匹配 baobiaotime 字符串，并打印
//     //
//     let re = Regex::new(r"(^\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})_([A-Z\.\-]+)_(?<baobiaotime>([0-9a-zA-Z\.\-\_]+))00-(\d{14})_X(?<timestamp>[\d]+)\.zip").unwrap();
//     println!("{}", re.is_match(raw));
//     let baobiaotime = format!("{}", re.replace(raw, "$baobiaotime"));
//     let timestamp = format!("{}", re.replace(raw, "$timestamp"));
//     println!("baobiaotime is {}", baobiaotime);
//     println!("timestamp is {}", timestamp);
// 
//     // 拼接csv文件名
//     //
//     let re = Regex::new(r"(^\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})_(?<odmbaobiaotime>[0-9a-zA-Z\-\.\_]+)\-(?<stoptime_timestamp>[0-9\_X]+)\.zip").unwrap();
//     println!("{}", re.is_match(raw));
//     let odmbaobiaotime = format!("{}", re.replace(raw, "$odmbaobiaotime"));
//     println!("odmbaobiaotime is {}", odmbaobiaotime );
//     let stoptime_timestamp = format!("{}", re.replace(raw, "$stoptime_timestamp"));
//     println!("stoptime_timestamp is {}", stoptime_timestamp);
// 
//     let csv_name = format!("{odmbaobiaotime}_{stoptime_timestamp}/{odmbaobiaotime}-{stoptime_timestamp}.csv");
//     println!("csv_name is {}", csv_name);

    // 写入到磁盘上
//  let mut count = 0;
//  for (filename, buffer) in filemap {
//      let mut file =File::create(filename)?;
//      file.write_all(&buffer)?;
//      count += 1;
//      println!("这是 写入到磁盘上 第{}次", count);
