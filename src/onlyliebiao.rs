use super::hxwxn_zero_min;
use super::todaylist;
// use super::time;

//  获得已经下载文件中的小时文件列表
pub fn onlyliebiao(riqi: &str, hm: &str) -> Vec<String> {
    println!("onlyliebiao call riqi is {}", riqi.to_string());
    println!("onlyliebiao call hm is {}", hm.to_string());
    let liebiao: Vec<String> = hxwxn_zero_min::hxwxn_zero();
    let todayhourlist: Vec<String> = todaylist::todayhourlist(&riqi);
    let mut noexistliebiao: Vec<String> = Vec::new();




//    println!("{:?}", &liebiao[..3]);
    for j in &liebiao {
        let mut onlyliebiao: Vec<String> = Vec::new();
        let mut hechabiaozhunliebiao: Vec<String> = Vec::new();
//        println!("first is {}", &j);
        for a in &todayhourlist {
            let b = format!("{}{}", j, riqi);
            if a.contains(&b) {
                onlyliebiao.push(a.to_string());
            }
        }
/*         println!("onlyliebiao is {:?} ", onlyliebiao );
		for z in &onlyliebiao {
			println!("onlyliebiao z is {}", z);
		} */
/*         println!("onlyliebiao length is {}", onlyliebiao.len());
        println!("+++++++++++++++++"); */
        let num: u8 = hm.to_string().parse().expect("not a number");
//        let num: u8 = 11;
        for k in 00..=num {
            let bb = format!("{}{}{:<02}", &j,  riqi, k);
            hechabiaozhunliebiao.push(bb.to_string());
        }
//        println!("hechabiaozhunliebiao is {:?}", hechabiaozhunliebiao);
//        println!("hechabiaozhunliebiao length is {:?}", hechabiaozhunliebiao.len());
/* 		for w in &hechabiaozhunliebiao {
			println!("hechabiaozhunliebiao w is {}", w);
		} */
        for ii in &hechabiaozhunliebiao {
//            println!("hechabiaozhunliebiao length is {}", hechabiaozhunliebiao.len());
            if !onlyliebiao.contains(ii) {
                noexistliebiao.push(ii.to_string());
            }
        }
/* 		for x in &noexistliebiao {
			println!("noexistliebiao x is {}", x);
		} */
//    break
    }
    println!("^^^^^^^^^^^^^^^^^^^^^^^");
    return noexistliebiao
}
