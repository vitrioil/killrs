use std::fs;

pub fn read_smap(pid: i32) {
    let data = fs::read_to_string(format!("/proc/{pid}/smaps_rollup"))
                    .expect("{pid} process id does not exist!");
    // let (mut private_huge, mut shared_huge, mut shared, mut private, mut pss): (&str, &str, &str, &str, &str);
    for line in data.split("\n") {
       // if line.starts_with("Private_Hugetlb") {
       //     (_, private_huge) = line.split_once(":").unwrap();
       // }
       // else if line.starts_with("Shared_Hugetlb") {
       //     (_, shared_huge) = line.split_once(":").unwrap();
       // }
       // else if line.starts_with("Shared") {
       //     (_, shared) = line.split_once(":").unwrap();
       // }
       // else if line.starts_with("Private") {
       //     (_, private) = line.split_once(":").unwrap();
       // }
       // else if line.starts_with("Pss") {
       //     (_, pss) = line.split_once(":").unwrap();
       // }
       println!("{}", line);
    }
    //println!("{} {} {} {} {}", private_huge, shared_huge, shared, private, pss)
}
