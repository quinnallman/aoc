pub fn a() -> i64 {
    let key = String::from("ckczppom");

    let mut num = 0;
    loop {
        let input = format!("{}{}", key, num);
        let hash: String = format!("{:x}", md5::compute(input));
        if hash.starts_with("00000") {
            break;
        }

        num += 1;
    }

    num
}

pub fn b() -> i64 {
    let key = String::from("ckczppom");

    let mut num = 0;
    loop {
        let input = format!("{}{}", key, num);
        let hash: String = format!("{:x}", md5::compute(input));
        if hash.starts_with("000000") {
            break;
        }

        num += 1;
    }

    num
}