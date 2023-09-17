//  获得已经下载文件中的   分钟文件列表
use std::env;
use super::hxwxn_zero_min;
use super::todaylist;
use super::arg;

pub fn onlyliebiao_min(riqi: &str, min_slice: Vec<String>) -> Vec<String> {
    println!("onlyliebiao_min.rs > onlyliebiao_min > riqi is {}", &riqi);
    let args: Vec<String> = env::args().collect();
    let path = &args[2];
    let _arg = arg::arg(path.to_string());
    let liebiao: Vec<String> = hxwxn_zero_min::hxwxn_min();
    let todaymin_list: Vec<String> = todaylist::todaymin_list(&riqi);
    let mut noexistliebiao_min: Vec<String> = Vec::new();


    for j in &liebiao {
        let mut onlyliebiao: Vec<String> = Vec::new();
        let mut hechabiaozhunliebiao_min: Vec<String> = Vec::new();
        let _hxwxn_min_5gc2: Vec<String> = hxwxn_zero_min::hxwxn_min_5gc2();
//        println!("first is {}", &j);
        for a in &todaymin_list {
            let b = format!("{}{}", j, riqi);
            if a.contains(&b) {
                onlyliebiao.push(a.to_string());
            }
        }
/* 		for z in &onlyliebiao {
			println!("onlyliebiao z is {}", z);
		} */

        for k in &min_slice {
            let bb = format!("{}{}{}", &j, riqi, k);
            hechabiaozhunliebiao_min.push(bb.to_string());
        }
		// println!("hechabiaozhunliebiao_min wWW length is {:?}", hechabiaozhunliebiao_min.len());

/* 		for w in &hechabiaozhunliebiao_min {	
			println!("hechabiaozhunliebiao_min w is {}", w);
		} */

        for ii in &hechabiaozhunliebiao_min {
            if !onlyliebiao.contains(ii) {
                noexistliebiao_min.push(ii.to_string());
            }
        }
/* 		for x in &noexistliebiao_min {
			println!("noexistliebiao_min x is {}", x);
		} */

		onlyliebiao.clear();
		hechabiaozhunliebiao_min.clear();
		// noexistliebiao_min.clear();

//    break
    }
	// println!("noexistliebiao_min DDDD length is {:?}", noexistliebiao_min);
	println!("noexistliebiao_min DDDD length is {:?}", noexistliebiao_min.len());

    println!("^^^^^^^^^^^^^^^^^^^^^^^");
    return noexistliebiao_min
}
