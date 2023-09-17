use std::collections::HashMap;

pub fn output(mut liebiao: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut output_bao_list: Vec<String> = Vec::new();
    let bak_liebiao: Vec<String> = liebiao.to_vec();

    for i in &mut liebiao {
        for j in &["_hntpd", "_hn_tpd_smsc_traf_delay", "_tpd"] {
            let a: Vec<_> = i.match_indices(j).collect();
            if a.len() != 0 {
                let b = a[0].0;
                let _zz = i.split_off(b);

                if !output_bao_list.contains(&i.to_string()) {
                    output_bao_list.push(i.to_string());
                }
            }
        }
    }
	for i in &output_bao_list {
		println!("output_bao_list is {}", i);
	}
	println!("        ↑        ↑        ↑        ↑");
	println!("        ↓        ↓        ↓        ↓");


    let mut samebao: Vec<String> = Vec::new();
    let mut samebaosametime: Vec<String> = Vec::new();
//    let mut new_samebaosametime: Vec<String> = Vec::new();
//    let mut bak_samebaosametime: Vec<String> = Vec::new();
    let mut hmap = HashMap::new();
//    let mut i = String::new();
    for i in &output_bao_list {
//        println!("i = {}", &i);
        for j in &bak_liebiao {
//            println!("j = {}", &j);
            if j.contains(i) {
                samebao.push(j.to_string());
            }
        }
        let mut time_list: Vec<String> = Vec::new();
        let bak_samebao: Vec<String> = samebao.to_vec();
        for mut i in bak_samebao {
            let a: Vec<_> = i.rmatch_indices("_20").collect();
            let b = a[0].0;
            let c = i.split_off(b);
            if !time_list.contains(&c) {
                time_list.push(c)
            }
        }

        for aj in time_list {
            for  mut k in samebao.to_vec() {
                let a: Vec<_> = k.match_indices("_20").collect();
                let b = a[0].0;
                let c = k.split_off(b);
                if aj == c {
                    let ab = format!("{}{}", k, aj);
                    samebaosametime.push(ab);
                }
            }

            samebaosametime.push(":::::::::::::::::::::::".to_string());
            samebaosametime.push("VVVVVVVV".to_string());

        }
        hmap.insert(i.to_string(), samebaosametime.to_vec());
        samebaosametime.clear();
        samebao.clear();
    }

    return hmap

}
