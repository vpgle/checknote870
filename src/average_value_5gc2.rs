// use super::todaylist;
// use super::time;
// use super::arg;
use super::hxwxn_zero_min;
use super::ope_line;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fs::File;
use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;

// 没有时间范围限制，这是在整个文件范围内找到所有文件，求出均值
//HashMap<String ,          Vec<HashMap<u128,   i128>> >
//        文件名带时间                  时间戳  文件大小
pub fn average_value_5gc2(riqi: &str, sep2_size_parameters: &str, sep2_1_size_parameters: &str) 
                        -> HashMap<String, u32> {
    let mut aver_hashmap: HashMap<String, u32> = HashMap::new();
    let mut hashmap_filename_timestamp_size: HashMap<String , Vec<HashMap<u128, i128>> >  = HashMap::new();
//    let mut hmap = HashMap::new();
    // 生成 sep1、sep2 分隔符

    println!("average_value_5gc2.rs -> riqi is {}", &riqi);
//  let local: DateTime<Local> = Local::now();
//  let y_ear = local.year();

    let y: i32 = riqi[0..4].to_string().parse::<i32>().expect("average_value_min.rs > y not a number!");
    let m: u32 = riqi[4..6].to_string().parse::<u32>().expect("average_value_min.rs > m not a number!");
    let d: u32 = riqi[6..=7].to_string().parse::<u32>().expect("average_value_min.rs > d not a number!");

    println!("average_value_min.rs > y is {}", y);
    println!("average_value_min.rs > m is {}", m);
    println!("average_value_min.rs > d is {}", d);

    let dt = Utc.ymd(y, m, d);
    // 获取第二天日期
    let dt_yesterday = dt - Duration::seconds(86_400);
    let dt_tomorrow = dt + Duration::seconds(86_400);
//  let mon_2 = dt.format("%b").to_string();
//  let mon_3 = dt_3.format("%b").to_string();
    let mut year_today = dt.format("%Y").to_string();
    let mut year_yesterday = dt_yesterday.format("%Y").to_string();
    let mut year_tomorrow = dt_tomorrow.format("%Y").to_string();
    year_today.insert_str(4, "-");
    year_yesterday.insert_str(4, "-");
    year_tomorrow.insert_str(4, "-");

    let mut v_sep1_filename: Vec<String> = Vec::new();
    println!("average_value_min.rs > year_2 is {}", &year_today);
    println!("average_value_min.rs > year_3 is {}", &year_yesterday);
    println!("average_value_min.rs > year_4 is {}", &year_tomorrow);

    if &year_today == &year_yesterday && &year_today == &year_tomorrow {
        v_sep1_filename.insert(0,year_today.to_string() );
    }
    if &year_today == &year_yesterday && &year_today != &year_tomorrow && &year_yesterday != &year_tomorrow {
        v_sep1_filename.insert(0, year_today.to_string());
        v_sep1_filename.insert(0, year_tomorrow.to_string());
    }
    if &year_today != &year_yesterday && &year_today == &year_tomorrow && &year_yesterday != &year_tomorrow {
        v_sep1_filename.insert(0, year_yesterday.to_string());
        v_sep1_filename.insert(0, year_today.to_string());
    }
    if &year_today != &year_yesterday && &year_today != &year_tomorrow && &year_yesterday != &year_tomorrow {
        println!("average_value_min.rs 里没有找到 年分隔符 分隔异常 - v_sep1_filename!!!");
    }

//  println!("average_value_5gc2.rs -> y_ear is {:?}", y_ear);
//  println!("average_value_5gc2.rs -> x is {:?}", x );
    println!("average_value_5gc2.rs -> average_value_5gc2 -> v_sep1_filename is |{:?}|", v_sep1_filename);
//  let mon = local.format("%b").to_string();
    let sep2_size = sep2_size_parameters.to_string();
    println!("average_value_5gc2.rs -> average_value_5gc2 -> sep2_size is |{}|", sep2_size );
    let sep2_1_size = sep2_1_size_parameters.to_string();
    println!("average_value_5gc2.rs -> average_value_5gc2 -> sep2_1_size is |{}|", sep2_1_size );


    // 获得今天的所有   分钟粒度 文件
    //  hxwxn_min_5gc2  来源 mecheck_note 模板 中 的 分钟 粒度
    let hxwxn_min_5gc2: Vec<String> = hxwxn_zero_min::hxwxn_min_5gc2();
    println!("average_value_5gc2.rs > todaylist.rs > todayhourlist > hxwxn_min_5gc2 length is {}", &hxwxn_min_5gc2.len());

    let mut count = 0;


    let mut hashmap_timestamp_size: Vec<HashMap<u128, i128>> = Vec::new();

    let mut inner_hmap : HashMap<u128, i128> = HashMap::new();



    for j in &hxwxn_min_5gc2 {
        // println!("|raw_hashmap -> j is {}", j );
        let s_j = j.to_string();
        // println!("||raw_hashmap -> s_j is {}", s_j );
        let mut size_ = String::new() ;
        let j_bak = j.clone();
        let mut riqi_bak = riqi.clone();
        // riqi_bak.remove(0);
        // let h = format!("{}{}", j_bak, &riqi_bak);
        let h = format!("{}{}", j_bak, &riqi_bak);

        let args: Vec<String> = env::args().collect();
        let path = &args[2];
        let file = File::open(path).unwrap();
        let fin = BufReader::new(file);
        let mut s: Vec<String>  = Vec::new();
        let riqi: String = format!("{}{}", "_", riqi);

        for line in fin.lines() {
            let mut new_line = ope_line(&line.unwrap());
            let mut new_line_bak =  new_line.clone();
            let s_j = String::new();
//          println!("average_value_5gc2.rs -> &riqi_bak is {}", &riqi_bak);
//          println!("average_value_5gc2.rs -> h is {}", h);
//          println!("v_sep1_filename --- {:?}", v_sep1_filename);

            if new_line.contains(&h) && new_line.contains("zip") {

                let mut v = Vec::new();

                let a: &String = &v_sep1_filename[0];
                let v_a: Vec<_> = new_line.match_indices(a).collect();

                if  v_a.len() > 0{
                    v = v_a;
                } else {
                    println!("average_value_5gc2.rs 里没有找到 年分隔符 - v_sep1_filename!!!");
                }

                if v.len() != 0 {
                    let y = v[0].0;
//                  println!("y is {}", y );
                    let mut zhi = new_line.split_off(y);
//                  println!("first zhi is {}", zhi);
    
                    let v: Vec<_> = zhi.match_indices("00_X").collect();
                    let y = v[0].0 + 4;
                    let mut timestamp = zhi.split_off(y);
//                  println!("second filename is {}", zhi);
    
                    let v: Vec<_> = zhi.match_indices("WL_").collect();
                    let y = v[0].0;
                    let mut new_filename = zhi.split_off(y);
//                  println!("new_filename is  {}", new_filename);
    
                    let v: Vec<_> = new_filename.match_indices("0-20").collect();
                    let y = v[0].0 + 1;
                    let mut finally_filename = new_filename.split_off(y);
//                  println!("new_filename is  {}", new_filename);
      
                    let v: Vec<_> = timestamp.match_indices(".zip").collect();
                    let y = v[0].0;
                    let mut zip = timestamp.split_off(y);
                    let timestamp = timestamp.parse::<u128>().unwrap();
//                  println!("timestamp is {}", timestamp);
    
                    let mut size: i128  = 999_999_999_999_999;
//                  println!("size default is {}", &size);
                    if new_line.len() != 0 {
//                      println!("{:?}", &new_line);
  
                        if new_line.contains(&sep2_size) {
                            let w: Vec<_> = new_line.match_indices(&sep2_size).collect();
//                          println!("w is is  {:?}", &w);
                            if w.len() != 0 {
//                              println!("\tindex w is {:?}", w);
                                let x = w[0].0;
//                              println!("\tindex w x is {:?}", x);
                                let mut size_raw = new_line.split_off(x);
//                              println!("\tsize_raw is |{}|", new_line);
                                let w: Vec<_> = new_line.rmatch_indices(" ").collect();
//                              println!("w w w  is {:?}", w[0].0 );
                                let y = w[0].0 + 1 ;
//                              println!("y is {}", y);
//                              println!("new_line length is {}", &new_line.len() );
                                let size_ = new_line.split_off(y);
//                              println!("\t\tsize_ is |{}|", size_);
                                if size_.len() != 0 {
                                    size = size_.parse::<i128>().unwrap();
                                } else {
                                    println!("获取文件大小失败 -> {:?}", new_line_bak );
                                }
//                              println!("\t\t|size| is |{}|", size );
                            } else {
                                println!("找不到获取文件大小的 sep2_size 分隔符!");
                            }

                        } else if new_line.contains(&sep2_1_size) {
                            let w: Vec<_> = new_line.match_indices(&sep2_1_size).collect();
//                          println!("w is is  {:?}", &w);
                            if w.len() != 0 {
//                              println!("\tindex w is {:?}", w);
                                let x = w[0].0;
//                              println!("\tindex w x is {:?}", x);
                                let mut size_raw = new_line.split_off(x);
//                              println!("\tsize_raw is |{}|", new_line);
                                let w: Vec<_> = new_line.rmatch_indices(" ").collect();
//                              println!("w w w  is {:?}", w[0].0 );
                                let y = w[0].0 + 1 ;
//                              println!("y is {}", y);
//                              println!("new_line length is {}", &new_line.len() );
                                let size_ = new_line.split_off(y);
//                              println!("\t\tsize_ is |{}|", size_);
                                if size_.len() != 0 {
                                    size = size_.parse::<i128>().unwrap();
                                } else {
                                    println!("获取文件大小失败 -> {:?}", new_line_bak );
                                }
//                              println!("\t\t|size| is |{}|", size );
                            } else {
                                println!("找不到获取文件大小的 sep2_1_size 分隔符!");
                            }

                        } else {
                            println!("找不到获取文件大小的 sep2_size / sep2_1_size 分隔符!");
                        }

                    }
                    // inner_hmap : HashMap<>
                    // inner_hmap.insert(timestamp, size);
                    inner_hmap.insert(timestamp, size);
                    let inner_hmap_bak = inner_hmap.clone();
//                  println!("average_value_5gc2.rs -> inner_hmap is {:?}", inner_hmap);
//                  println!("average_value_5gc2.rs -> inner_hmap_bak is {:?}", inner_hmap_bak);

                    // hashmap_timestamp_size : Vec<>
                    // println!("average_value_5gc2.rs -> hashmap_timestamp_size  before is {:?}", &hashmap_timestamp_size);
                    hashmap_timestamp_size.insert(0, inner_hmap_bak);
//                  println!(" ###  hashmap_timestamp_size is ->   {:?} length is {}", &hashmap_timestamp_size, &hashmap_timestamp_size.len());
                    let mut hashmap_timestamp_size_bak = &hashmap_timestamp_size;
                    // println!("average_value_5gc2.rs -> hashmap_timestamp_size after is {:?}", hashmap_timestamp_size);

                    // hashmap_filename_timestamp_size : HashMap<>
                    hashmap_filename_timestamp_size.insert(j.to_string(), hashmap_timestamp_size_bak.to_vec());
                    // println!("\taverage_value_min.rs -> hashmap_filename_timestamp_size is {:?}", hashmap_filename_timestamp_size);

                    // println!("\t\t inner_hmap_bak is {:?}", &inner_hmap_bak);
                } else {
                    println!("找不到获取文件名的 sep1_file_name 分隔符!");
                }
//              println!("\t\t inner_hmap is {:?}", inner_hmap);
    
            }

            // break
//            println!("|||raw_hashmap -> s_j is {}", s_j );
            inner_hmap.clear();
        }
//      println!("");
//      println!("||||raw_hashmap -> s_j_outer is {}", s_j );
        hashmap_timestamp_size.clear();

    }
//  let hashmap_filename_timestamp_size_bak = hashmap_filename_timestamp_size;
    //
//  println!("average_value_5gc2.rs -> average_value_5gc2 -> sep2_size is |{}|", sep2_size );

//  println!("average_value_5gc2.rs -> hashmap_filename_timestamp_size is {:?}", &hashmap_filename_timestamp_size.len());
    let mut store_value: Vec<f32> = Vec::new();
    let mut sort_value:  Vec<f32> = Vec::new();

    let mut key_store_value: HashMap< String, Vec< f32 > > = HashMap::new();
    let mut key_sort_value:  HashMap< String, Vec< f32 > > = HashMap::new();

    println!("在average_value_5gc2.rs 中 生成 ");
    for (key , value ) in &hashmap_filename_timestamp_size {
//      println!("key is {:?} <<<< average_value_5gc2.rs >>>> value is {:?}", key, value );
//      println!("key is {} ======== {}", key, value.len());
        for i in value {
            for ( a, b ) in i {
                store_value.insert(0, (*b) as f32);
            }
        }
//      println!("store_value raw is {:?}", store_value);
//      println!("");
        count += 1;

        // 211 -> 223: 冒泡排序，把值从小到大排序
        let mut a: f32 = f32::MIN;
        let mut b: f32 = f32::MAX;
        for i in 0..store_value.len() - 1{
            for j in 0..store_value.len() -1 -i {
                if store_value[j] > store_value[j + 1] {
                    a = store_value[j];
                    b = store_value[j + 1];
                    store_value[j] = b;
                    store_value[j + 1] = a;
                }
            }
        }

        // 求平均值
//      let mut aver_store_value  = store_value.clone();
//      因定义 store_value 时是 可变的，所有可以不用 clone 到 aver_store_value ，便可直接使用 store_value

        // 一般会出现正常文件大小的1倍或多倍，暂未发现有xml有数据、csv无数据这种情况下的文件大小倍
        // 数，即缺失数据情况下文件大学的倍数
        // 233 -> 244: 先去掉最大值和最小值，然后在比较此时VEC中最大值和最小值除法值是否大于1.5,大于1.5继续去掉
        // 最大值和最小值，以此往复
        if store_value.len() > 2 {
            store_value.pop();
            store_value.remove(0);
            let length = store_value.len();
            let mut bei: f32 = store_value[length - 1]/store_value[0];

            'counting_up: loop {
                if bei > 1.5 && length > 2 {
                    store_value.pop();
                    store_value.remove(0);
                    let length = store_value.len();
                    if length > 2 {
                        bei = store_value[length - 1]/store_value[0];
                    } else if length == 2 {
                        bei = store_value[length - 1]/store_value[0];
                        break;
                    } else if length == 1 {
                        bei = 1.0;
                        break;
                    } else if length == 0 {
                        println!("average_value_min.rs -> hxwxn_min 'counting_up : remove store_value length = 0, one 需要重写!!! ");
                    } else {
                        println!("average_value_min.rs -> hxwxn_min 'counting_up : remove store_value length = 0, two 需要重写!!! ");
                    }
                } else {
                    break 'counting_up;
                }

            }

            // 均值  由函数sum_ 实现
            let average_value_min: u32 = sum_(&store_value) as u32;
            aver_hashmap.insert(key.to_string(), average_value_min);
        } else if store_value.len() == 2 {
            // 均值  由函数sum_ 实现
            let average_value_min: u32 = sum_(&store_value) as u32;
            aver_hashmap.insert(key.to_string(), average_value_min);
        } else if store_value.len() == 1 {
            let average_value_min: u32 = sum_(&store_value) as u32;
            aver_hashmap.insert(key.to_string(), average_value_min);
        } else {
            println!("average_value_min.rs -> average_value_min -> 个数不对  ");
        }

        store_value.clear();
//      println!("average_value_min is {:?}", &average_value_min);
//      aver_hashmap.insert(key.to_string(), average_value_min);
//      return aver_hashmap;
    }

    println!("################### average_value_5gc2.rs #######################");
    return aver_hashmap
}

//fn vec_hashmap(line: &Vec<HashMap<u128, i128>>) -> Vec<HashMap<u128, i128>> {
//    line.clone()
//}

        // sum 所有数之和
fn sum_(vc: &Vec<f32>) -> u32 {
    let mut sum: f32 = 0.0;
    for i in 0..vc.len() {
        sum += vc[i];
    }

    let vclen: f32 = vc.len() as f32;
    let average: u32 = (sum/ vclen) as u32;
    average
}

