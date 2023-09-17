//  获得已经下载文件中的  5gc2  分钟文件列表
//  然后处理生成应该采集但未采集的文件列表
use super::hxwxn_zero_min;
use super::todaylist;

//   onlyliebiao_min_5gc2(&riqi    ,   min_slice_5gc2(minresult: (String, String)))
pub fn onlyliebiao_min_5gc2(riqi: &str, min_slice_5gc2: Vec<String>) -> Vec<String> {
    println!("onlyliebiao_min_5gc2.rs > onlyliebiao_min_5gc2 > riqi is {}", &riqi);
//    println!("min_slice_5gc2 is {:?}", min_slice_5gc2);
//    println!("min_slice_5gc2 length is {}", min_slice_5gc2.len());
    let liebiao: Vec<String> = hxwxn_zero_min::hxwxn_min_5gc2();
    let todaymin_5gc2_list: Vec<String> = todaylist::todaymin_5gc2_list(&riqi);
    let mut noexistliebiao_min_5gc2: Vec<String> = Vec::new();

    if todaymin_5gc2_list.len() != 0 && liebiao.len() != 0 {
        for j in &liebiao {
            let mut onlyliebiao: Vec<String> = Vec::new();
            let mut hechabiaozhunliebiao_min_5gc2: Vec<String> = Vec::new();
//            println!("first is {}", &j);
            for a in &todaymin_5gc2_list {
                let b = format!("{}", j);
                if a.contains(&b) {
                    onlyliebiao.push(a.to_string());
                }
            }
//            println!("onlyliebiao_min_5gc2.rs > liebiao > onlyliebiao is {:?}", onlyliebiao);
//            for i in &onlyliebiao {
//                println!("onlyliebiao_min_5gc2.rs > liebiao > onlyliebiao is {}", i);
//            }
    
            for k in &min_slice_5gc2 {
                let bb = format!("{}{}", &j, k);
                hechabiaozhunliebiao_min_5gc2.push(bb.to_string());
            }
    //        println!("hechabiaozhunliebiao_min_5gc2 is {:?}", hechabiaozhunliebiao_min_5gc2);
//            for i in &hechabiaozhunliebiao_min_5gc2 {
//                println!("hechabiaozhunliebiao_min_5gc2 is {}", i);
//            }
    
            for ii in &hechabiaozhunliebiao_min_5gc2 {
                if !onlyliebiao.contains(ii) {
                    noexistliebiao_min_5gc2.push(ii.to_string());
                }
            }
    //        println!("noexistliebiao_min_5gc2 is {:?}", noexistliebiao_min_5gc2);
//            for i in &noexistliebiao_min_5gc2 {
//                println!("noexistliebiao_min_5gc2 is {}", i);
//            }
//        break
        }

    }
    println!("^^^^^^^^^^^^^^^^^^^^^^^");
    return noexistliebiao_min_5gc2
}
