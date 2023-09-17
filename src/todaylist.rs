use std::io::BufReader;
use std::io::BufRead;
use super::ope_line;
use std::env;
use std::fs::File;
use super::hxwxn_zero_min;
use super::arg;
// use super::time;



// 获得今天的所有   小时粒度 文件
pub fn todayhourlist(riqi: &str) -> Vec<String> {
    println!("todaylist.rs > todayhourlist > riqi is {}", riqi);
    //  hxwxn_zero  来源 mecheck_note 模板 中 的 小时 粒度
    let hxwxn_zero: Vec<String> = hxwxn_zero_min::hxwxn_zero();
    println!("todaylist.rs > todayhourlist > hxwxn_zero length is {}", &hxwxn_zero.len());
    let args: Vec<String> = env::args().collect();
    let path = &args[2];
//    println!("args[2] is {}", path);
    let file = File::open(path).unwrap();
    let fin = BufReader::new(file);
    let mut s: Vec<String>  = Vec::new();
    let riqi: String = format!("{}{}", "_", riqi);
    for line in fin.lines() {
        let mut new_line = ope_line(&line.unwrap());
        for j in &hxwxn_zero {

            if new_line.contains(j) && new_line.contains(&riqi) && new_line.contains("zip") {
                let v: Vec<_> = new_line.match_indices("WL_").collect();
                let y = v[0].0;
                let mut zhi = new_line.split_off(y);
//                  println!("first zhi is {}", zhi);
                if zhi.len() != 0 {
                    let w: Vec<_> = zhi.match_indices("0000-").collect();
//                      println!("index w is {:?}", w);
                    if w.len() != 0 {
                        let z = w[0].0;
                        let _zhinew = zhi.split_off(z);
                        if zhi.len() != 0 {
                            s.push(zhi);
                        }
                    }
                }
            }

        }



    }
    println!("s length is {}", s.len());
    println!("//////////////////////////////////////////");
    return s
}

// 获得今天的所有   分钟粒度 文件
pub fn todaymin_list(riqi: &str) -> Vec<String> {
    println!("todaylist.rs > todaymin_list > riqi is {}", riqi);
    let hxwxn_min: Vec<String> = hxwxn_zero_min::hxwxn_min();
    println!("todaylist.rs > todaymin_list > hxwxn_min length is {}", &hxwxn_min.len());
    let args: Vec<String> = env::args().collect();
    let path = &args[2];
//    println!("args[2] is {}", path);
    let file = File::open(path).unwrap();
    let fin = BufReader::new(file);
    let mut s: Vec<String>  = Vec::new();
    let riqi: String = format!("{}{}", "_", riqi);
    for line in fin.lines() {
        let mut new_line = ope_line(&line.unwrap());
        for j in &hxwxn_min {

            if new_line.contains(j) && new_line.contains(&riqi) && new_line.contains("zip") {
                let v: Vec<_> = new_line.match_indices("WL_").collect();
                let y = v[0].0;
                let mut zhi = new_line.split_off(y);
                if zhi.len() != 0 {
                    let w: Vec<_> = zhi.match_indices("00-20").collect();
                    if w.len() != 0 {
                        let z = w[0].0;
                        let _zhinew = zhi.split_off(z);
                        if zhi.len() != 0 {
                            s.push(zhi);
                        }
                    }
                }
            }
        }
    }
//    println!("s length is {}", s.len());
//    println!("//////////////////////////////////////////");
    return s
}


// todaymin_5gc2_list(time::min_result( &hm, riqi.to_string()).1)
// 获得今天的所有 5gc2  分钟粒度 文件
pub fn todaymin_5gc2_list(riqi: &str) -> Vec<String> {
    println!("todaylist.rs > todaymin_5gc2_list > riqi is {}", riqi);
    let args: Vec<String> = env::args().collect();
    let hxwxn_min_5gc2: Vec<String> = hxwxn_zero_min::hxwxn_min_5gc2();
//    let riqi: String = time::houresult(_hm, riqi.to_string()).1;
    println!("hxwxn_min_5gc2 length is {}", &hxwxn_min_5gc2.len());

    let mut s: Vec<String>  = Vec::new();
    let path = &args[2];
    let filepath = arg::arg(path.to_string());
    if filepath.to_string() == String::from("S01_FTP_5GC_0002_S") {

        let file = File::open(path).unwrap();
        let fin = BufReader::new(file);
        let riqi: String = format!("{}{}", "_", riqi);
        println!("todaylist.rs > todaymin_5gc2_list > riqi > if inner is {}", riqi);
        for line in fin.lines() {
            let mut new_line = ope_line(&line.unwrap());
            for j in &hxwxn_min_5gc2 {
    
                if new_line.contains(j) && new_line.contains(&riqi) && new_line.contains("zip") {
                    let v: Vec<_> = new_line.match_indices("WL_").collect();
                    let y = v[0].0;
                    let mut zhi = new_line.split_off(y);
                    if zhi.len() != 0 {
                        let w: Vec<_> = zhi.match_indices("00-20").collect();
                        if w.len() != 0 {
                            let z = w[0].0;
                            let _zhinew = zhi.split_off(z);
                            if zhi.len() != 0 {
                                s.push(zhi);
                            }
                        }
                    }
                }
            }
        }

    }
//    println!("s length is {}", s.len());
//    println!("//////////////////////////////////////////");
    return s
}
