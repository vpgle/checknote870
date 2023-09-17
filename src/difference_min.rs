// 
// 处理比较时间大小以及做差值
//
// use super::time;
// use super::raw_hashmap;
// use super::arg;
// use std::io::BufReader;
// use std::io::BufRead;
// use std::fs::File;
// use super::average_value_min;
// use super::raw_hashmap_min;
// use std::env;
use std::collections::HashMap;
// use super::hxwxn_zero_min;

pub fn diff(average_value_min: HashMap<String, u32>  , hxwxn_min_diff: HashMap<String, u32>,  raw_hashmap_min: HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>)> )
                                                                    ->  HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>)> {

//  println!("average_value_min is {:?}", average_value_min);
//  println!("average_value_min length is {:?}", average_value_min.len());
    println!("difference_min.rs -> raw_hashmap_min length is {:?}", raw_hashmap_min.len());
//  println!("hxwxn_min_diff is {:?}", hxwxn_min_diff );
//  println!("hxwxn_min_diff length is {:?}", hxwxn_min_diff.len() );

//  存放差值
//  let mut diff_value_min_display: u32 = u32::MAX;   //  存放差值
//  存放差值
    let mut b: HashMap<String, String> = HashMap::new();

    let mut ret: HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>)> = HashMap::new();

//  let x: HashMap<String, (Vec<String> , HashMap<String, String> , Vec<String>)> = raw_hashmap_min;
//  for (key , value ) in &x {
//      println!("key is {:?}", key );
//      println!("               ");
//      println!("               ");
//      for a in &value.0 {
//          println!("a is {:?}", a);
//      }
//      for b in &value.1 {
//          println!("b is {:?}", b);
//      }
//      for c in &value.2 {
//          println!("c <> is {:?}", c);
//      }
//      println!("               ");
//      println!("               ");
//  }

    let z: HashMap<String, u32> = hxwxn_min_diff;
    for (aver_key, aver_value) in &average_value_min {
        let aver_value_u32: u32 = *aver_value;
        let diff_value_min: u32 = *z.get(aver_key).unwrap();
        for ( raw_key, raw_value ) in &raw_hashmap_min {
            if aver_key == raw_key {
                for ( x, y ) in &raw_value.1 {
                    let y_u32: u32 = y.parse().expect("difference_min.rs -> y not a number!");
                    if ((aver_value_u32 > y_u32) && (aver_value_u32 - y_u32 > diff_value_min )) || ((y_u32 > aver_value_u32 ) && (y_u32 - aver_value_u32 > diff_value_min )) {
                        b.insert(x.to_string(), y.to_string());
                    }
                }

                // 借用值不用手工清理，对比b 和 raw_value.0、raw_value.2可知
                let b_bak = b.clone();
                let raw_value_0 = &raw_value.0;
                let raw_value_2 = &raw_value.2;
                ret.insert(raw_key.to_string(), (raw_value_0.to_vec(), b_bak, raw_value_2.to_vec()  ));
                b.clear();
            } else {
                continue
            }
        }
    }

    println!("VVVVVVVVVVVVVVVVVV");
    println!("{}", "XXXX".repeat(4));
    let xx: &HashMap<String, (Vec<String> , HashMap<String, String> , Vec<String>)> = &ret;
    for (key , value ) in xx {
        let average: &u32  =  average_value_min.get(key).expect("difference_min.rs -> average is not");
        let diff_value_min_display: u32 = *z.get(key).unwrap();
        println!("key si {} ==  {:?} ( {} {} )", key , average 
            , average-diff_value_min_display,average+diff_value_min_display  );
        println!("diff_value_min_display is {:?}", diff_value_min_display );
        println!("               ");
        println!("               ");
//      for a in &value.0 {
//          println!("a is {:?}", a);
//      }
        for b in &value.1 {
            println!("difference_min.rs -> b is {:?}", b);
        }
        for b in value.1.keys() {
            println!("dir *{}*", b);
        }
//      for c in &value.2 {
//          println!("c is {:?}", c);
//      }
        println!("               ");
        println!("               ");
        println!("               ");
        println!("^^^^^^^^^^^^^^^");
    }

//  xx
    ret
}
