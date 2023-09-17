// 
// 处理比较时间大小以及做差值
//
// use super::time;
// use super::raw_hashmap;
// use super::hxwxn_zero_min;
// use super::arg;
// use std::io::BufReader;
// use std::io::BufRead;
// use std::env;
// use std::fs::File;
// use super::average_value;
use std::collections::HashMap;
// use super::raw_hashmap;


pub fn diff(average_value: HashMap<String, u32>  , hxwxn_zero_diff: HashMap<String, u32>, raw_hashmap: HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>)> ) ->
                                                                HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>)> {

//  println!("difference.rs -> diff -> average_value is {:?}", average_value);
//  println!("difference.rs -> diff -> average_value length is {:?}", average_value.len());
    println!("difference.rs -> diff -> raw_hashmap length is {:?}", raw_hashmap.len());
//  println!("difference.rs -> diff -> hxwxn_zero_diff is {:?}", hxwxn_zero_diff);
//  println!("difference.rs -> diff -> hxwxn_zero_diff length is {:?}", hxwxn_zero_diff.len());
//  存放差值
//  let mut diff_value_zero_display: u32 = u32::MAX;   //  存放差值
//  存放差值

    let mut ret: HashMap<String, ( Vec<String> ,  HashMap<String, String> ,  Vec<String>)> = HashMap::new();

    let x: HashMap<String, (Vec<String> , HashMap<String, String> , Vec<String>)> = raw_hashmap;
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

    
    println!("{}", "XXXX".repeat(4));
    let z: HashMap<String, u32> = hxwxn_zero_diff;
    for (aver_key, aver_value ) in &average_value {
        let aver_value_u32: u32 = *aver_value;
//      let mut change_size: HashMap<String, String> = HashMap::new();
        let mut change_size: HashMap<String, String> = HashMap::new();
        let mut change_size_bak: HashMap<String, String> = HashMap::new();
        let diff_value = *z.get(aver_key).unwrap();
        for (raw_key , raw_value ) in &x {
//          println!("raw_key is {:?}", raw_key );
//          println!("aver_key is {:?}", aver_key );
            match raw_key.contains(aver_key) {
                true => {
                    for (a, b) in &raw_value.1 {
                        let c= b.parse::<u32>().expect("difference.rs -> b is not number !");
//                      println!("c is {:?}", c );
                        if ( (aver_value_u32 > c ) && (aver_value_u32 - c > diff_value )) || ( ( c > aver_value_u32 ) && (c - aver_value_u32 > diff_value )) {
                            change_size.insert(a.to_string(), b.to_string());
                        }

                    }
                },
                false => { continue }
            }
            ///////////////////////////////////////////////////////
//          println!("difference count \t\tchange_size length is {:?}", change_size.len());
            change_size_bak = change_size.clone();
            change_size.clear();
            ret.insert(raw_key.to_string(), ( raw_value.0.to_vec(), change_size_bak, raw_value.2.to_vec()));
        }
    }
    println!("{}", "\\/X".repeat(4));

    let y: &HashMap<String, (Vec<String> , HashMap<String, String> , Vec<String>)> = &ret;
    for (key , value ) in y {
        println!("");
        println!("        ↓        ↓        ↓        ↓");
        let average: u32 = *average_value.get(key).expect("difference.rs -> average_value");
        let diff_value_zero_display = *z.get(key).unwrap();
        println!("key si {} == {} ( {} {} )", key , average
            , average-diff_value_zero_display , average+diff_value_zero_display );
        println!("diff_value_zero_display is {:?}", diff_value_zero_display );
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
        println!("        ↑        ↑        ↑        ↑");
        println!("               ");
        println!("               ");
    }
//  println!("{}", "Y".repeat(9));
//  println!("ret length is {:?}", ret.len());
//  println!("average_value is {:?}", average_value);
    ret
}
