#[allow(dead_code, unused, warnings)]
// #[warn(unused_variables)]
//use std::io::prelude::*;
//use std::fs::File;
//use std::io::BufReader;
use std::env;
use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;

mod output_min_5gc2;
mod output_min;
mod output;
mod onlyliebiao_min_5gc2;
mod onlyliebiao_min;
mod onlyliebiao;
mod hxwxn_zero_min;
mod arg;
mod todaylist;
mod time;

mod difference;
mod difference_min;
mod difference_5gc2;

mod raw_hashmap;
mod raw_hashmap_min;
mod raw_hashmap_5gc2;

mod average_value;
mod average_value_min;
mod average_value_5gc2;

mod abc_output;
mod abc_output_min;
mod abc_output_5gc2;

fn ope_line(line: &String) -> String {
    line.clone()
}

fn main() {


    let args: Vec<String> = env::args().collect();
    let path = &args[2];
    let zzz = arg::arg(path.to_string());
    let riqi_string = &args[3];
    println!("main.rs > path args[3] is {:?}", riqi_string );

    // args_riqi 需要检查的日期
    let args_riqi_u32 = &args[3][0..4].parse::<u32>().expect("abc_output.rs -> abc_output -> 第三个参数不是数字！");
    let mut sep2_2_size_parameters: String = args_riqi_u32.to_string();
    sep2_2_size_parameters.insert_str(0, "_");
    println!("main.rs > sep2_2_size_parameters is {:?}", sep2_2_size_parameters);

    let y: i32 = riqi_string[0..4].to_string().parse::<i32>().expect("main.rs > y not a number!");
    let m: u32 = riqi_string[4..6].to_string().parse::<u32>().expect("main.rs > m not a number!");
    let d: u32 = riqi_string[6..=7].to_string().parse::<u32>().expect("main.rs > d not a number!");

    println!("main.rs > y is {}", y);
    println!("main.rs > m is {}", m);
    println!("main.rs > d is {}", d);

    let dt = Utc.ymd(y, m, d);
    // 获取第二天日期
    let dt2 = dt + Duration::seconds(86_400);
    let dt3 = dt - Duration::seconds(86_400 * 15);
    let mon2 = dt2.format("%b").to_string();
    let mon3 = dt3.format("%b").to_string();
    let year2 = dt2.format("%Y").to_string();
    let year3 = dt3.format("%Y").to_string();
    println!("main.rs > month2 is {}", &mon2);
    println!("main.rs > month3 is {}", &mon3);
    println!("main.rs > year2 is {}", &year2);
    println!("main.rs > year3 is {}", &year3);



    //  2022-02-02 add
    //  V
    let mut sep1 = String::from(" ");
    let x = y.to_string();
    sep1.insert_str(1, &x);
    sep1.insert_str(5, "-");
    println!("main.rs > sep1 is #{}#", sep1);

    let mut sep2 = String::from(" ");
    sep2.insert_str(1, &mon2);
    sep2.insert_str(4, " ");
    println!("main.rs > sep2 is #{}#", &sep2);

    let mut sep2_1 = String::from(" ");
    sep2_1.insert_str(1, &mon3);
    sep2_1.insert_str(4, " ");
    println!("main.rs > sep2_1 is #{}#", &sep2_1);

    //  ^
    //  2022-02-02 add

    let local: DateTime<Local> = Local::now();
    let riqi = local.format("%Y%m%d").to_string();
    let hm = local.format("%H%M").to_string();
    let hh24 = local.format("%H").to_string();
    let _mi = local.format("%M").to_string();


    println!("main.rs > riqi is {}", riqi);
    println!("main.rs > hm is {}", hm);
    println!("main.rs > hh24 is {}", hh24);

    println!("{}",  "-__-".repeat(4));

    println!("main.rs > houresult is {:?}", time::houresult(&hm, riqi.to_string()));
    println!("main.rs > minesult is {:?}", time::minresult(&hm, riqi.to_string()));

    println!("");

    println!("##### main.rs #####");
    println!("{}",  "~~~~~~~~~~".repeat(4));

    if zzz == "S01_FTP_5GC_0002_S" {
        let x: HashMap<String, (Vec<String> , HashMap<String, String> , Vec<String>)> = 
        difference_5gc2::diff(average_value_5gc2::average_value_5gc2(&time::minresult(&hm, riqi.to_string()).1
                                                                 , &sep2
                                                                 , &sep2_1
                                                                 )
                             , hxwxn_zero_min::hxwxn_min_5gc2_diff()

                             , raw_hashmap_5gc2::hashmap_size_filename(time::minresult(&hm, riqi.to_string())
                                                                     , &sep2
                                                                     , &sep2_1
                                                                     , time::min_slice(time::minresult(&hm, riqi.to_string()))
                                                                     )
                             );
        abc_output_5gc2::abc_output_5gc2(x, sep2_2_size_parameters );
    } else {


        let m: HashMap<String, (Vec<String> , HashMap<String, String> , Vec<String>)> = 
        difference_min::diff(average_value_min::average_value_min(&time::minresult(&hm, riqi.to_string()).1
                                                                 , &sep2
                                                                 , &sep2_1
                                                                 )
                            , hxwxn_zero_min::hxwxn_min_diff()

                            , raw_hashmap_min::hashmap_size_filename(time::minresult(&hm, riqi.to_string())
                                                                    , &sep2
                                                                    , &sep2_1
                                                                    , time::min_slice(time::minresult(&hm, riqi.to_string()))
                                                                    )
                            );
        let sep2_2_size_parameters_bak: String = sep2_2_size_parameters.clone() ;
        abc_output_min::abc_output_min(m, sep2_2_size_parameters_bak );

        print!("{}", "   ".repeat(3)); print!("{}", "   ".repeat(3)); println!("{}", "(_)".repeat(3));
        print!("{}", "   ".repeat(3)); print!("{}", "   ".repeat(3)); print!("{}", "(_)".repeat(1)); print!("{}", "   ".repeat(1)); print!("{}", "(_)".repeat(1)); println!("{}", "   ".repeat(3));
        print!("{}", "   ".repeat(3)); print!("{}", "   ".repeat(3)); print!("{}", "(_)".repeat(1)); print!("{}", "   ".repeat(1)); print!("{}", "(_)".repeat(1)); println!("{}", "   ".repeat(3));
        print!("{}", "   ".repeat(3)); print!("{}", "   ".repeat(3)); print!("{}", "(_)".repeat(1)); print!("{}", "   ".repeat(1)); print!("{}", "(_)".repeat(1)); println!("{}", "   ".repeat(3));
        print!("{}", "   ".repeat(3)); print!("{}", "   ".repeat(3)); print!("{}", "(_)".repeat(1)); print!("{}", "   ".repeat(1)); print!("{}", "(_)".repeat(1)); println!("{}", "   ".repeat(3));
        print!("{}", "   ".repeat(3)); print!("{}", "(_)".repeat(4)); print!("{}", "   ".repeat(1)); println!("{}", "(_)".repeat(4));
        print!("{}", "   ".repeat(3)); print!("{}", "   ".repeat(1)); print!("{}", "(_)".repeat(1)); print!("{}", "   ".repeat(5)); println!("{}", "(_)".repeat(1));
        print!("{}", "   ".repeat(3)); print!("{}", "   ".repeat(2)); print!("{}", "(_)".repeat(1)); print!("{}", "   ".repeat(3)); println!("{}", "(_)".repeat(1));
        print!("{}", "   ".repeat(3)); print!("{}", "   ".repeat(3)); print!("{}", "(_)".repeat(1)); print!("{}", "   ".repeat(1)); println!("{}", "(_)".repeat(1));
        print!("{}", "   ".repeat(3)); print!("{}", "   ".repeat(4)); println!("{}", "(_)".repeat(1));

        let y: HashMap<String, (Vec<String> , HashMap<String, String> , Vec<String>)> = 
        difference::diff(average_value::average_value(&time::houresult(&hm, riqi.to_string()).1
                                                     , &sep2
                                                     , &sep2_1
                                                     )
                        , hxwxn_zero_min::hxwxn_zero_diff()

                        , raw_hashmap::hashmap_size_filename(time::houresult(&hm, riqi.to_string())
                                                     , &sep2
                                                     , &sep2_1
                                                     )
                        );
        abc_output::abc_output(y, sep2_2_size_parameters );

   }


    println!("vvvvvv main.rs vvvvv");

    println!("");
    println!("");
    println!("{}",  "~~~~~~~~~~".repeat(4));

    println!("");
    println!("");
    println!("VVV 小时粒度 VVV");
    println!("");
    println!("VVV 分钟粒度 VVV");



}



