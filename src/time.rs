use std::env;
use chrono::Duration;
use chrono::prelude::*;



//  小时粒度时间判断    // riqi 获得系统当前日期
pub fn houresult(hm: &str, riqi: String) -> (String, String) {
    let args: Vec<String> = env::args().collect();
    // args_riqi 需要检查的日期
    let args_riqi = &args[3];
    let args_riqi_u32 = args_riqi.parse::<u32>().expect("time.rs -> houresult -> 第三个参数不是数字！");

    let riqi_string = &riqi;
    println!("path args[3] is {:?}", riqi_string );

    let y: i32 = riqi_string[0..4].to_string().parse::<i32>().expect("y not a number!");
    let m: u32 = riqi_string[4..6].to_string().parse::<u32>().expect("m not a number!");
    let d: u32 = riqi_string[6..=7].to_string().parse::<u32>().expect("d not a number!");

    println!("y is {}", y);
    println!("m is {}", m);
    println!("d is {}", d);

    let dt = Utc.ymd(y, m, d);
    let riqi_sub = dt - Duration::seconds(86_400);
    println!("riqi_sub is {:?}", riqi_sub );

//    println!("time.rs > houresult > hm is {}", hm);
//    println!("time.rs > houresult > args_riqi is {}", args_riqi);
//    println!("time.rs > houresult > riqi is {}", riqi);

    let riqi_sub_u32 = riqi_sub.format("%Y%m%d").to_string().parse::<u32>().expect("time.rs -> houresult -> 获取的系统日期不是数字！");

    if riqi_sub_u32 > args_riqi_u32 {
        return ("23".to_string(), args_riqi.to_string())
    } else if riqi_sub_u32 == args_riqi_u32 {
        let hh24: u8 = hm[..2].parse().expect("time.rs -> houresult not a number!");
        match hh24 {
            0 => return ("22".to_string(), args_riqi.to_string()),
            _ => return ("23".to_string(), args_riqi.to_string()),
        }

    } else if riqi_sub_u32 < args_riqi_u32 {
        let hh24: u8 = hm[..2].parse().expect("time.rs -> houresult not a number!");
        match hh24 {
            //  
            0 => return (String::from("因为今天0点数据还没有采集到，返回为空!"), String::from("因为今天0点数据还没有采集到，返回为空!")),
            1 => return (String::from("因为今天1点数据还没有采集到，返回为空!"), String::from("因为今天1点数据还没有采集到，返回为空!")),
            _ => return (format!("{:<02}", hh24 - 2), args_riqi.to_string()),
        }
    } else {
        return (String::from("第三个参数不是数字！"), String::from(" 获取的系统日期不是数字！"))
    }

}


//  分钟粒度时间判断
pub fn minresult(hm: &str, riqi: String) -> (String, String) {
    // result ( 小时分钟 ，  日期  )
    let args: Vec<String> = env::args().collect();
    let args_riqi = &args[3];
    let args_riqi_u32 = args_riqi.to_string().parse::<u32>().expect("time.rs -> minresult -> args_riqi_u32 不是数字！ ");
    let riqi_u32 = riqi.parse::<u32>().expect("time.rs -> minresult -> riqi_u32 不是数字！ ");
    let hh24: u8 = hm[0..2].parse().expect("not a number!");
    let mi: u8 = hm[2..].parse().expect("not a number!");
    match mi {
        mi => {
            //  18：00数据18:30采集，18:45到18:59必须能查到
            if mi >= 45 && mi <=59 {
                if args_riqi_u32 < riqi_u32 {
                    //  riqi 取昨天
                    return ("2345".to_string(), args_riqi.to_string())
                } else if args_riqi_u32 == riqi_u32 {
                    //  riqi 取今天
                    return (format!("{:<02}{mi:<02}", hh24, mi = 00), riqi  )
                } else {
                    return ("hhmm is Wrong!".to_string(), "riqi is Wrong!".to_string())
                }
            } else if mi < 45 && mi >= 30 {
                if args_riqi_u32 < riqi_u32 {
                    //  riqi 取昨天
                    return ("2345".to_string(), args_riqi.to_string())
                } else if args_riqi_u32 == riqi_u32 {
                    return (format!("{:<02}{mi}", hh24 - 1 , mi = 45), riqi  )
                } else {
                    return ("hhmm is Wrong!".to_string(), "riqi is Wrong!".to_string())
                }

            } else if mi < 30 && mi >= 15 {
                if args_riqi_u32 < riqi_u32 {
                    //  riqi 取昨天
                    return ("2345".to_string(), args_riqi.to_string())
                } else if args_riqi_u32 == riqi_u32 {
                    return (format!("{:<02}{mi}", hh24 - 1, mi = 30), riqi  )
                } else {
                    return ("hhmm is Wrong!".to_string(), "riqi is Wrong!".to_string())
                }

            } else if mi < 15 {
                if args_riqi_u32 < riqi_u32 {
                    //  riqi 取昨天
                    return ("2345".to_string(), args_riqi.to_string())
                } else if args_riqi_u32 == riqi_u32 {
                    return (format!("{:<02}{mi}", hh24 - 1, mi = 15), riqi  )
                } else {
                    return ("hhmm is Wrong!".to_string(), "riqi is Wrong!".to_string())
                }

            } else {
                return (String::from("time.rs of minresult is X!!"), String::from("time.rs of minresult is X!"))
            }
        },
    }
}

// 获取分钟粒度所需要检查的最大时间，然后据此生成Vec<String>，

pub fn min_slice(minresult: (String, String)) -> Vec<String> {
    println!("time.rs > min_slice > call minresult  is {} {} _?", &minresult.0, &minresult.1);
    let mut min_slice: Vec<String> = Vec::new();
    let hh24: u8 = minresult.0[..2].parse().expect("not a number!");
    let mi: u8 = minresult.0[2..].parse().expect("not a number!");
    let _hhmm: String = minresult.0;
    println!("time.rs > min_slice > hh24 is {} _?", hh24);
    println!("time.rs > min_slice > mi  is {} _?", mi );

    for i in 00..=hh24 {
        for j in 00..=45 {
            if i < hh24 {
                if j % 15 == 0 {
                    min_slice.push(format!("{:<02}{:<02}", i, j));
                }
            }
            if i == hh24 {
                for j in 00..=mi {
                    if j % 15 == 0 {
                        min_slice.push(format!("{:<02}{:<02}", i, j));
                    }
                }
                break
            }
        }
    }
	let alen = min_slice.len() - 1;
	println!("↓");
	println!("min_slice first is {},  last is {}, length is {}", min_slice[0], min_slice[alen], min_slice.len());
	println!("↑");
    return min_slice
}


pub fn min_slice_5gc2(minresult: (String, String)) -> Vec<String> {
    println!("time.rs > min_slice_5gc2 > call minresult  is {} {}", &minresult.0, &minresult.1);
    let mut min_slice_5gc2: Vec<String> = Vec::new();
    let hhmm: String = minresult.0;

    let hh24: u8 = hhmm[..2].parse().expect("not a number!");

    println!("time.rs > min_slice_5gc2 > hh24 is {}", hh24);

    let mi: u8 = hhmm[2..].parse().expect("not a number!");

    println!("time.rs > min_slice_5gc2 > mi  is {}", mi );


    let riqi: String = minresult.1;

    if hh24 >= 13 {
        min_slice_5gc2.push(format!("{}{}", riqi, "0515"));
        min_slice_5gc2.push(format!("{}{}", riqi, "0530"));
        min_slice_5gc2.push(format!("{}{}", riqi, "0545"));
        for i in 6..=hh24 {
            for j in 00..=45 {
                if j % 15 == 0 {
                    min_slice_5gc2.push(format!("{}{:<02}{:<02}", riqi, i, j));
                }

                if i == hh24 {
                    for k in 15..=mi {
                        if k % 15 == 0 {
                            min_slice_5gc2.push(format!("{}{:<02}{:<02}", riqi, i, k));
                        }
                    }
                    break
                }

            }
        }
    } 
    if hh24 < 13 {
        for i in 0..=hh24 {
            for j in 0..=mi {
                if j % 15 ==0 {
                    min_slice_5gc2.push(format!("{}{:<02}{:<02}", riqi, i, j));
                }
            }
        }
    }
	let alen = min_slice_5gc2.len() - 1;
	println!("↓");
	println!("min_slice_5gc2 first is {},  last is {}, length is {}", min_slice_5gc2[0], min_slice_5gc2[alen], min_slice_5gc2.len());
	println!("↑");
    return min_slice_5gc2
}
