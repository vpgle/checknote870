

// 待核查文件名与被需要检查内容关联，需要对文件名是绝对路径或仅是文件名进行处理
pub fn arg(mut path: String) -> String {
    let ind: Vec<_> = path.rmatch_indices("/").collect();
    if ind.len() != 0 {
        let y = ind[0].0 + 1;
        let mut zzz = path.split_off(y);
        let ind: Vec<_> = zzz.rmatch_indices("_20").collect();
        let y = ind[0].0;
        let _yyy = zzz.split_off(y);
//        println!("path is {}", path);
//        println!("zzz is {}", zzz);
        return zzz;
    } else {
        let ind: Vec<_> = path.rmatch_indices("_20").collect();
        let y = ind[0].0;
        let _yyy = path.split_off(y);
        return path;
    }

}

