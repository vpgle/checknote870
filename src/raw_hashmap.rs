use super::hxwxn_zero_min;
use super::ope_line;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fs::File;
use std::collections::HashMap;
// use super::todaylist;
// use super::time;
// use super::arg;
// use super::todaylist;
// use chrono::prelude::*;


//  输出最新一条的包表时间，时间戳，文件大小

pub fn hashmap_size_filename(houresult: (String , String), sep2_size_parameters: &str , sep2_1_size_parameters: &str ) 
                         -> HashMap< String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>) >    {
    println!("raw_hashmap.rs -> houresult.0 is {}", &houresult.0);
    println!("raw_hashmap.rs -> houresult.1 is {}", &houresult.1);
    println!("raw_hashmap.rs -> hashmap_size_filename > sep2_size_parameters is |{}|", sep2_size_parameters);
    println!("raw_hashmap.rs -> hashmap_size_filename > sep2_1_size_parameters is |{}|", sep2_1_size_parameters);
    //  hxwxn_zero  来源 mecheck_note 模板 中 的 分钟 粒度
    let hxwxn_zero: Vec<String> = hxwxn_zero_min::hxwxn_zero();
    println!("raw_hashmap.rs -> hxwxn_zero length is {}", &hxwxn_zero.len());

    let mut j_clone: Vec<String> = Vec::new();

    let mut hour_string: HashMap< String, Vec< String > > = HashMap::new();

    let mut _key_string: String = String::new();

    let mut s: Vec<String>  = Vec::new();
    let hh24: u8 = houresult.0.parse().expect("not a number!");
    let riqi: String = format!("{}", houresult.1);
    for j in &hxwxn_zero {
        let args: Vec<String> = env::args().collect();
        let path = &args[2];
        let file = File::open(path).unwrap();
        let fin = BufReader::new(file);

        for line in fin.lines() {
            let mut new_line = ope_line(&line.unwrap());
//          let key_string = j.clone();
//          let j_bak = j.clone();
            let mut riqi_bak = &riqi.clone();
            // riqi_bak.remove(0);
            let h = format!("{}{}", &j, &riqi_bak);
//          println!("h is {}", &h);
    
            match new_line.contains(&h) {
                true => {
                    j_clone.insert(0, new_line.clone());
                },
                false => { continue; }
            }

            let j_clone_bak = vec_string(j_clone.to_vec());
            hour_string.insert(h.clone(), j_clone_bak);
        }
        j_clone.clear();


    }


    let mut miss_baotime: Vec<String> = Vec::new();
    let mut timestamp_count: HashMap<String, String> = HashMap::new();
    let mut timestamp_more_count: Vec<String> = Vec::new();
    let mut hash_tuple: HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>) > = HashMap::new();

//  最新时间戳，如果同个时间戳有多条数据按异常算，需补采
//  判断最新时间点是否有多条，有多条需补采
//  输出最新一条的包表时间，时间戳，文件大小
//  for (key , value ) in &hour_string {
//      println!("{:?} <> is ", key );
//      for a in value {
//          println!(" -every value- is {}", a);
//      }
//  }


//  println!("is tnirp!");
    for l in &hxwxn_zero {
        let mut vec_timestamp: Vec<String> = Vec::new();
        let mut _vec_key: Vec<String> = Vec::new();
        let mut _hash_timestamp: HashMap<String, Vec<String>> = HashMap::new();
        let mut count_cache: Vec<String> = Vec::new();
        let j = format!("{}{}", l,  riqi );
//      println!("j is {:?}", j.to_string());
        for (key , value ) in &hour_string {
            match key.clone() == j  {
                true => {
                    for hour  in 0..=hh24 {
                        let j_hour = format!("{}{:<02}{}", j, hour, "00");
//                      println!("j + hour = {:?}", j_hour );
                        for v in value {
                            if v.contains(&j_hour) {
                                count_cache.insert(0,v.to_string());
                            }
                        }
//                      println!("\tcount_cache sis {}", count_cache.len() );
                        match count_cache.len()  {
                            0 => {
                                let j_hour_miss_baotime = j_hour.clone();
                                miss_baotime.insert(0, j_hour_miss_baotime );
                            },
                            _ => {
                                for b in &count_cache {
                                    let mut z = b.clone();
                                    let d: Vec<_> = z.match_indices("00_X").collect();
                                    if d.len() == 1 {
                                        let a = d[0].0 + 3;
                                        let mut y = z.split_off(a);
//                                      println!("00_X timestamp g is {:?}", &y);
                                        let e: Vec<_> = y.match_indices(".zip").collect();
                                        if e.len() == 1 {
                                            let f = e[0].0;
                                            let _g = y.split_off(f);
//                                          println!("timestamp g is {:?}", &y);
                                            vec_timestamp.insert(0, y);
                                        }
                                    }
                                }
                                // 对时间戳从小到大排序
                                vec_timestamp.sort();
                                // 对时间戳去重
                                vec_timestamp.dedup();


                                match vec_timestamp.len() {
                                    0 => { println!("vec_timestamp count is 0!"); },
                                    _ => {
                                        let x: String = vec_timestamp.pop().unwrap();
//                                      println!("");
                                        // 对去重后的进行搜索，确定每个时间戳到底有多少个行数, 每个时间戳
                                        // 现在是同包同表同时间点同一个时间戳有成倍的重复数据，
                                        // 所以判断最新的时间戳有多于一个文件的需要重新补采
                                        let mut count: u128 = 0;
                                        let mut count_cache_pop_one: Vec<String> = Vec::new();
                                        for strline in &count_cache {
                                            if strline.contains(&x) {
//                                              println!("value in strline is {:?}", strline);
                                                count += 1;
                                                count_cache_pop_one.insert(0,strline.to_string());
                                            }
                                        }
                                        count_cache.clear();
        
                                        match count {
                                            0 => { println!("vec_timestamp.len() is 0!"); },
                                            1 => {
                                                let (a,b) = out_size(count_cache_pop_one.to_vec(), sep2_size_parameters, sep2_1_size_parameters);
                                                // timestamp_count.insert(x.parse::<u128>().unwrap(), a);
                                                timestamp_count.insert(a, b);
//                                              println!("timestamp_count is {:?}", timestamp_count);
                                            },
                                            _ => {
                                                let j_hour_bak = j_hour.clone();
                                                timestamp_more_count.insert(0, j_hour_bak );
                                                // timestamp_more_count.insert(0, key.to_string());
//                                              println!("timestamp_more_count is {:?}",timestamp_more_count );
                                            }
                                        }
        
                                        count_cache_pop_one.clear();

                                    }
                                }


                            }
                        }



                    }

                vec_timestamp.clear();
//              println!("asd");
//              println!("miss_baotime is {:?}", miss_baotime);
                let miss_baotime_bak = vec_string(miss_baotime.to_vec());
                let timestamp_count_bak = vec_u128(&timestamp_count);
                let timestamp_more_count_bak = vec_string(timestamp_more_count.to_vec());
                let j_hour_vec_key = l.clone();
//              println!("timestamp_more_count_bak is {:?}",timestamp_more_count_bak );

                miss_baotime.clear();
                timestamp_count.clear();
                timestamp_more_count.clear();
                
                hash_tuple.insert(j_hour_vec_key , ( miss_baotime_bak,  timestamp_count_bak,  timestamp_more_count_bak) );

                },
                false => { continue }
            }
        }

    }
//  println!("hash_tuple si {:?}", hash_tuple );

    return hash_tuple

}

fn out_size(s_string: Vec<String>, sep2_size_parameters: &str , sep2_1_size_parameters: &str ) -> (String, String) {
//  println!("sep2_size_parameters is {:?}", sep2_size_parameters);
    let mut wl_time: String = String::new();
    let mut vec_str = s_string;
    //  999_123_456_789_0
    let mut size: u128 = 9991234567890;
    if vec_str.len()  == 1  {
        let mut vstr = &mut vec_str[0];

        if vstr.contains(sep2_size_parameters) {
            let w: Vec<_> = vstr.match_indices(sep2_size_parameters).collect();
            if w.len() == 1 {
                let x = w[0].0;
    //          println!("x_is {}", x);
                let mut size_raw = vstr.split_off(x);
    //          println!("vstr is {:?}", &vstr);
    //          println!("size_raw is {:?}", &size_raw );
                let a: Vec<_> = size_raw.match_indices("WL_").collect();
                if a.len() == 1 {
                    let x = a[0].0;
                    let mut wl_to_time = size_raw.split_off(x);
    //              println!("wl_to_time is {:?}", &wl_to_time );
                    let b: Vec<_> = wl_to_time.match_indices("00-20").collect();
    //              println!("b is {:?}", b);
                    if b.len() == 1 {
                        let x = b[0].0;
                        let _ = wl_to_time.split_off(x);
    //                  println!("\t\twl_to_time is {:?}", &wl_to_time );
                        wl_time = wl_to_time;
                    }
    
                }
    
                let w: Vec<_> = vstr.rmatch_indices(" ").collect();
                let y = w[0].0 + 1;
                let size_ = vstr.split_off(y);
    //          println!("raw_hashmap.rs -> out_size -> size_ is {:?}", size_);
                match size_.len() {
                    0 => {
                        println!("raw_hashmap.rs -> out_size one -> 没有找到文件大小值");
                    },
                    _ => {
                        size = size_.parse::<u128>().unwrap();
                    }
                }
            }

        } else if vstr.contains(sep2_1_size_parameters) {
            let w: Vec<_> = vstr.match_indices(sep2_1_size_parameters).collect();
            if w.len() == 1 {
                let x = w[0].0;
    //          println!("x_is {}", x);
                let mut size_raw = vstr.split_off(x);
    //          println!("vstr is {:?}", &vstr);
    //          println!("size_raw is {:?}", &size_raw );
                let a: Vec<_> = size_raw.match_indices("WL_").collect();
                if a.len() == 1 {
                    let x = a[0].0;
                    let mut wl_to_time = size_raw.split_off(x);
    //              println!("wl_to_time is {:?}", &wl_to_time );
                    let b: Vec<_> = wl_to_time.match_indices("00-20").collect();
    //              println!("b is {:?}", b);
                    if b.len() == 1 {
                        let x = b[0].0;
                        let _ = wl_to_time.split_off(x);
    //                  println!("\t\twl_to_time is {:?}", &wl_to_time );
                        wl_time = wl_to_time;
                    }
    
                }
    
                let w: Vec<_> = vstr.rmatch_indices(" ").collect();
                let y = w[0].0 + 1;
                let size_ = vstr.split_off(y);
    //          println!("raw_hashmap.rs -> out_size -> size_ is {:?}", size_);
                match size_.len() {
                    0 => {
                        println!("raw_hashmap.rs -> out_size two -> 没有找到文件大小值");
                    },
                    _ => {
                        size = size_.parse::<u128>().unwrap();
                    }
                }
            }


        } else {
            println!("raw_hashmap.rs -> out_size end -> 没有找到文件大小值");

        }



    }
    (wl_time.to_string(), size.to_string())
}

fn split_bao(baobiao: String) -> String {
    println!("baobiao is {}", &baobiao);

    // args_riqi 需要检查的日期
    let args: Vec<String> = env::args().collect();
    let args_riqi = &args[3];
    let args_riqi_u32 = args_riqi[0..4].parse::<u32>().expect("raw_hashmap.rs -> split_bao -> 第三个参数不是数字！");
    let mut sep2_2_size_parameters: String = args_riqi_u32.to_string();
    sep2_2_size_parameters.insert_str(0, "_");
    println!("sep2_2_size_parameters is {:?}", sep2_2_size_parameters);

    let mut stri: String = String::new();
    for j in &[sep2_2_size_parameters] {
        match baobiao.find(j) {
            Some(a) => {
                match a {
                    0 => {
                        println!("raw_hashmap.rs -> split_bao in 256 -> 切割包时为0！");
                    },
                    _ => {
//                      println!("aaaa is {}", &a);
                        let (first, _) = baobiao.split_at(a + 1);
                        stri = first.to_string();
//                      println!("stri ======  {:?}", &stri);
                    }

                }
            },
            None => {
                continue
            }
        }
    }

    return stri
}

//fn vec_hashmap(line: &Vec<HashMap<u64, u128>>) -> Vec<HashMap<u64, u128>> {
//    line.clone()
//}

fn vec_u128(line: &HashMap<String, String>) -> HashMap<String, String> {
    line.clone()
}

fn vec_string(line: Vec< String >) -> Vec< String > {
    line.clone()
}

