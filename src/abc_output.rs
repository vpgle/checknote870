use std::collections::HashMap;
// use std::env;

pub fn abc_output(mut difference: HashMap<String , (Vec<String> , HashMap<String, String> , Vec<String> )> , sep2_2_size_parameters: String) {

    let x: HashMap<String , (Vec<String> , HashMap<String, String> , Vec<String> )> = difference;

    let mut ac: Vec<String> = Vec::new();
    let mut b: Vec<String> = Vec::new();


    println!("        ↓        ↓        ↓        ↓");
    println!("##### abc_output.rs -> abc_output  #####");
    for (key, value ) in &x {
        for zero in &value.0 {
            ac.insert(0, zero.to_string());
        }
        for two in &value.2 {
            ac.insert(0, two.to_string());
        }
        for one in &value.1 {
            b.insert(0, one.0.to_string());
        }
    }

//  for a_c in &ac {
//      println!("a_c is {}", a_c );
//  }

//  for b in b {
//      println!("b si {}", b);
//  }

    let mut abc_output_time_list: Vec<String> = Vec::new();
    for every in &ac {
        let mut ev = every.clone();
        let v: Vec<_> = ev.match_indices(&sep2_2_size_parameters).collect();
        let a = v[0].0 + 1;
        let zz = ev.split_off(a);
        abc_output_time_list.insert(0, zz);

    }
    abc_output_time_list.sort();
    abc_output_time_list.dedup();


    println!("defect time list");
    for a in &abc_output_time_list{
        println!("\t\t{:?}", a );
    }

    let mut sometime_vec: Vec<String> = Vec::new();
    for a_two in abc_output_time_list {
        for eve_two in &ac {
            match eve_two.contains(&a_two) {
                true => {
                    sometime_vec.insert(0, eve_two.to_string());
                },
                false => { continue }
            }
        }

        let mut biao_bao: Vec<String> = Vec::new();
        let mut bao: Vec<String> = Vec::new();
        for stv in &mut sometime_vec {
            for j in &["_hn_tpd_smsc_traf_delay", "_hntpd", "_tpd"] {
                let v: Vec<_> = stv.match_indices(j).collect();
                if v.len() == 1 {
                    let b = v[0].0;
                    let mut yy = stv.split_off(b);
                    // println!("sv is {}", &sv);
                    let sv_bak = stv.clone();
                    bao.insert(0, sv_bak);
                    let w: Vec<_> = yy.match_indices(&sep2_2_size_parameters).collect();
                    if w.len() == 1 {
                        let c = w[0].0;
                        let _ = yy.split_off(c);
                        let yy = yy.replacen("_", "", 1 );
                        // println!("yy is {}", &yy);
                        let bb = format!("{}---{}", yy, stv);
                        biao_bao.insert(0,bb);
                    }
                }
            }
        }
        sometime_vec.clear();

        println!("");
        println!("defect time is {}", a_two );
        bao.sort();
        bao.dedup();
        for ba in &bao {
            println!("defect bao is   {}" , ba );
        }
        biao_bao.sort();
        biao_bao.dedup();
        println!("|");
        println!("V");
        println!("{{");
        for bib in &biao_bao {
            println!("  {}", bib );
        }
        println!("}}");
        println!("---- NEXT ----");
        println!("");

    }

    // output_min.rs 
    let mut output_bao_list: Vec<String> = Vec::new();
    let bak_onlyliebiao_min: Vec<String> = ac.to_vec();

    for i in &mut ac {
        for j in &["_hn_tpd_smsc_traf_delay", "_hntpd", "_tpd"] {
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
    println!("|");
    println!("|");
    println!("|");
    println!("|");
    println!("|");
    println!("|");
    println!("|  /");
    println!("| /");
    println!("|/");
    for i in &output_bao_list {
        println!("output_bao_list is {}", i);
    }
    println!("        ↑        ↑        ↑        ↑");
    println!("        ↓        ↓        ↓        ↓");


    let mut samebao: Vec<String> = Vec::new();
    let mut samebaosametime: Vec<String> = Vec::new();
//  let mut hmap: HashMap<String, HashMap<String, String>>  = HashMap::new();
    let mut hmap  = HashMap::new();
    for i in &output_bao_list {
        for j in &bak_onlyliebiao_min {
            if j.contains(i) {
                samebao.push(j.to_string());
            }
        }

        let mut time_list: Vec<String> = Vec::new();
        let bak_samebao: Vec<String> = samebao.to_vec();
        for mut i in bak_samebao {
            let a: Vec<_> = i.rmatch_indices(&sep2_2_size_parameters).collect();
            if a.len() != 0 {
                let b = a[0].0;
                let c = i.split_off(b);
                if !time_list.contains(&c) {
                    time_list.push(c)
                }
            }
        }

        let mut biao_bao: Vec<String> = Vec::new();
        let mut bao: Vec<String> = Vec::new();
        let mut out_time: String = String::new();
        let mut val = HashMap::new();

        for aj in time_list {
            for  mut k in samebao.to_vec() {
                let a: Vec<_> = k.match_indices(&sep2_2_size_parameters).collect();
                let b = a[0].0;
                let c = k.split_off(b);
                if aj == c {
                    samebaosametime.push(k);
                }
            }

            for stv in &samebaosametime {
                let mut sv = stv.clone();
                for j in &["_hn_tpd_smsc_traf_delay", "_hntpd", "_tpd"] {
                    let v: Vec<_> = stv.match_indices(j).collect();
                    if v.len() == 1 {
                        let b = v[0].0;
                        // println!("# b is {}", b);
                        if b < sv.len() {
                            let mut yy = sv.split_off(b);
                            // println!("# yy is {}", yy);
                            // println!("# sv is {}", sv);
                            let yy = yy.replacen("_", "", 1);
                            // println!("sv is {}", &sv);
                            let v = format!("{}---{}", yy.to_string(), sv.to_string());
                            biao_bao.insert(0, v);
                        } else {
                            println!(">>>>>>>>>is {}", stv );
                            println!(">>>>>>>>>is {}", sv );
                            println!("");
                        }
                    }
                }
            }
            biao_bao.sort();
            biao_bao.dedup();
            let aj = aj.replacen("_", "", 1);
            val.insert(aj.to_string(), biao_bao.to_vec());
            biao_bao.clear();
            samebaosametime.clear();

        }
        samebao.clear();
        hmap.insert(i.to_string(), val);
    }


    for (key, value) in &hmap {
        println!("");
        println!("{} is key", key);
        println!("|");
        println!("V");
        for (m , n ) in value {
            println!("{}", m);
            println!("{{");
            for nn in n {
                println!("\t{}", nn);
            }
            println!("}}");
            println!("");
        }
        println!("value is ^^^^^^^^^^^^^^");
    }

    println!("{}", "^^^^^".repeat(4));
    for (key, value ) in &x {
        for two in &value.2 {
            println!("repeat timestamp is {:?} ", two);
        }
    }
    println!("{}", "vvvvv".repeat(4));
}
