// 
// 处理比较时间大小以及做差值
//
// use super::time;
// use super::raw_hashmap;
// use super::arg;
// use std::io::BufReader;
// use std::io::BufRead;
// use std::fs::File;
// use super::average_value_5gc2;
// use super::raw_hashmap_5gc2;
// use std::env;
use std::collections::HashMap;
// use super::hxwxn_zero_min;

pub fn diff(average_value_5gc2: HashMap<String, u32>  , hxwxn_min_5gc2_diff: HashMap<String, u32>,  raw_hashmap_5gc2: HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>)> )
                                                                    ->  HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>)> {

//  println!("average_value_5gc2 is {:?}", average_value_5gc2);
//  println!("average_value_5gc2 length is {:?}", average_value_5gc2.len());
    println!("difference_min.rs -> raw_hashmap_5gc2 length is {:?}", raw_hashmap_5gc2.len());
//  println!("hxwxn_min_5gc2_diff is {:?}", hxwxn_min_5gc2_diff );
//  println!("hxwxn_min_5gc2_diff length is {:?}", hxwxn_min_5gc2_diff.len() );

//  存放差值
//  let mut diff_value_min_5gc2_display: u32 = u32::MAX;   //  存放差值
//  存放差值
    let mut b: HashMap<String, String> = HashMap::new();

    let mut ret: HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>)> = HashMap::new();

    let x: HashMap<String, (Vec<String> , HashMap<String, String> , Vec<String>)> = raw_hashmap_5gc2;
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
//          println!("c is {:?}", c);
//      }
//      println!("               ");
//      println!("               ");
//  }

    let z: HashMap<String, u32> = hxwxn_min_5gc2_diff;
    for (aver_key, aver_value) in &average_value_5gc2 {
        let aver_value_u32: u32 = *aver_value;
        let diff_value_min_5gc2: u32 = *z.get(aver_key).unwrap();
        for ( raw_key, raw_value ) in &x {
            if aver_key == raw_key {
                for ( x, y ) in &raw_value.1 {
                    let y_u32: u32 = y.parse().expect("difference_min.rs -> y not a number!");
                    if ((aver_value_u32 > y_u32) && (aver_value_u32 - y_u32 > diff_value_min_5gc2 )) || ((y_u32 > aver_value_u32 ) && (y_u32 - aver_value_u32 > diff_value_min_5gc2 )) {
                        b.insert(x.to_string(), y.to_string());

                    }

                }

                // 借用值不用手工清理，对比b 和 raw_value.0、raw_value.2可知
                let b_bak = b.clone();
                let raw_value_0 = &raw_value.0;
//              println!("raw_value_0 is {:?}", &raw_value_0);
                let raw_value_2 = &raw_value.2;
//              println!("raw_value_2 is {:?}", &raw_value_2);
                ret.insert(raw_key.to_string(), (raw_value_0.to_vec(), b_bak, raw_value_2.to_vec()  ));
                b.clear();


            } else {
                continue
            }

        }

    }

    println!("VVVVVVVVVVVVVVVVVV");
    println!("{}", "XXXX".repeat(4));
    println!("ret length is {}", ret.len());
    let xx: &HashMap<String, (Vec<String> , HashMap<String, String> , Vec<String>)> = &ret;
    for (key , value ) in xx {
        println!("        ↓        ↓        ↓        ↓");
        let average: &u32  =  average_value_5gc2.get(key).expect("difference_5gc2.rs -> average is not");
        let diff_value_min_5gc2_display: u32 = *z.get(key).unwrap();
        println!("key si {} == {:?} ( {} {} )", key, average
            , average-diff_value_min_5gc2_display , average+diff_value_min_5gc2_display);
        println!("diff_value_min_5gc2_display is {:?}",diff_value_min_5gc2_display );
        println!("               ");
        println!("               ");
//      for a in &value.0 {
//          println!("a is {:?}", a);
//      }
        for b in &value.1 {
            println!("b is {:?}", b);
        }
        for b in value.1.keys() {
            println!("dir *{}*", b);
        }
//      for c in &value.2 {
//          println!("c is {:?}", c);
//      }
        println!("               ");
        println!("        ↑        ↑        ↑        ↑");
        println!("               ");
    }

//  xx
    ret
}
