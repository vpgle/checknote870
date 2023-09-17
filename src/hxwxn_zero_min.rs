use std::{fs::File, env};
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use super::arg;
use super::ope_line;

// 从check_note模板 获得核心网小时粒度所有文件
pub fn hxwxn_zero() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let checknote = &args[1];
    let path = format!("{}", &args[2]);

    let zzz = arg::arg(path);

    let mut hxwxn_zero: Vec<String> = Vec::new();
    let file = File::open(checknote).unwrap();
    let fin = BufReader::new(file);

    for line in fin.lines() {
        let new_line = ope_line(&line.unwrap());
        if new_line.contains(&zzz) && new_line.contains("\t0\t") {
            let v: Vec<_> = new_line.split_terminator('\t').collect();
            let a = format!("{}|{}|{}|{}", v[0],  v[3], v[4], v[5] );
//            println!("{}", a);
            let v: Vec<&str> = a.split('|').collect();
            let a = v[2];
            hxwxn_zero.push(a.to_string());
        }
    }
//  println!("hxwxn_zero length is {:?}", hxwxn_zero.len());
//  println!("hxwxn_zero is {:?}", hxwxn_zero);
    return hxwxn_zero

}


// 从check_note模板 获得核心网小时粒度所有文件
pub fn hxwxn_zero_diff() ->  HashMap<String, u32> {
    let args: Vec<String> = env::args().collect();
    let checknote = &args[1];
    let path = format!("{}", &args[2]);

    let zzz = arg::arg(path);

    let mut hxwxn_zero_diff: HashMap<String, u32> = HashMap::new();
    let file = File::open(checknote).unwrap();
    let fin = BufReader::new(file);

    for line in fin.lines() {
        let new_line = ope_line(&line.unwrap());
        if new_line.contains(&zzz) && new_line.contains("\t0\t") {
            let v: Vec<_> = new_line.split_terminator('\t').collect();
            let a = format!("{}|{}|{}|{}|{}", v[0],  v[3], v[4], v[5] , v[6] );
            let v: Vec<&str> = a.split('|').collect();
            let a = v[2];
            let b: u32 = v[4].parse::<u32>().expect("hxwxn_zero_min.rs -> hxwxn_min_diff -> diff_value_min is not number!");
            hxwxn_zero_diff.insert(a.to_string(), b);
        }
    }
//  println!("hxwxn_zero length is {:?}", hxwxn_zero.len());
//  println!("hxwxn_zero is {:?}", hxwxn_zero);
    return hxwxn_zero_diff

}


// 从check_note模板 获得分钟粒度所有文件
pub fn hxwxn_min() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let checknote = &args[1];
    let path = format!("{}", &args[2]);
    let nul: Vec<String> = Vec::new();

    let zzz = arg::arg(path);
    if zzz == "S01_FTP_5GC_0002_S" {
        return nul
    }

    let mut hxwxn_min: Vec<String> = Vec::new();
    let file = File::open(checknote).unwrap();
    let fin = BufReader::new(file);

    for line in fin.lines() {
        let new_line = ope_line(&line.unwrap());
        if new_line.contains(&zzz) && new_line.contains("\t-1\t") {
            let v: Vec<_> = new_line.split_terminator('\t').collect();
            let a = format!("{}|{}|{}|{}", v[0],  v[3], v[4], v[5] );
            let v: Vec<&str> = a.split('|').collect();
            let a = v[2];
            hxwxn_min.push(a.to_string());
        }
    }
    return hxwxn_min
}


// 从check_note模板 获得分钟粒度所有文件
pub fn hxwxn_min_diff() ->  HashMap<String, u32> {
    let args: Vec<String> = env::args().collect();
    let checknote = &args[1];
    let path = format!("{}", &args[2]);
    let nul:  HashMap<String, u32> = HashMap::new();

    let zzz = arg::arg(path);
    if zzz == "S01_FTP_5GC_0002_S" {
        return nul
    }

    let mut hxwxn_min_diff: HashMap<String, u32> = HashMap::new();
    let file = File::open(checknote).unwrap();
    let fin = BufReader::new(file);

    for line in fin.lines() {
        let new_line = ope_line(&line.unwrap());
        if new_line.contains(&zzz) && new_line.contains("\t-1\t") {
            let v: Vec<_> = new_line.split_terminator('\t').collect();
            let a = format!("{}|{}|{}|{}|{}", v[0],  v[3], v[4], v[5] , v[6] );
            let v: Vec<&str> = a.split('|').collect();
            let a = v[2];
            let b: u32 = v[4].parse::<u32>().expect("hxwxn_zero_min.rs -> hxwxn_min_diff -> diff_value_min is not number!");
            hxwxn_min_diff.insert(a.to_string(), b);
        }
    }
    return hxwxn_min_diff
}

// 从check_note模板 获得 5gc2 分钟粒度所有文件
pub fn hxwxn_min_5gc2() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let checknote = &args[1];
    let path = format!("{}", &args[2]);

    let zzz = arg::arg(path);

    let mut hxwxn_min_5gc2: Vec<String> = Vec::new();
    let file = File::open(checknote).unwrap();
    let fin = BufReader::new(file);

    for line in fin.lines() {
        let new_line = ope_line(&line.unwrap());
        if new_line.contains(&zzz) && new_line.contains("\t-1\t") {
            let v: Vec<_> = new_line.split_terminator('\t').collect();
            let a = format!("{}|{}|{}|{}", v[0],  v[3], v[4], v[5] );
            let v: Vec<&str> = a.split('|').collect();
            let a = v[2];
            hxwxn_min_5gc2.push(a.to_string());
        }
    }
    return hxwxn_min_5gc2

}

// 从check_note模板 获得 5gc2 分钟粒度所有文件
pub fn hxwxn_min_5gc2_diff() -> HashMap<String, u32> {
    let args: Vec<String> = env::args().collect();
    let checknote = &args[1];
    let path = format!("{}", &args[2]);

    let zzz = arg::arg(path);

    let mut hxwxn_min_5gc2_diff: HashMap<String, u32> = HashMap::new();
    let file = File::open(checknote).unwrap();
    let fin = BufReader::new(file);

    for line in fin.lines() {
        let new_line = ope_line(&line.unwrap());
        if new_line.contains(&zzz) && new_line.contains("\t-1\t") {
            let v: Vec<_> = new_line.split_terminator('\t').collect();
            let a = format!("{}|{}|{}|{}|{}", v[0],  v[3], v[4], v[5] , v[6] );
            let v: Vec<&str> = a.split('|').collect();
            let a = v[2];
            let b: u32 = v[4].parse::<u32>().expect("hxwxn_zero_min.rs -> hxwxn_min_5gc2_diff -> diff_value_min_5gc2 is not number!");
            hxwxn_min_5gc2_diff.insert(a.to_string(), b);
        }
    }
    return hxwxn_min_5gc2_diff

}

