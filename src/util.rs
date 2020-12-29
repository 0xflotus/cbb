pub mod cbb {
    pub fn int_to_bal_ternary(mut n: i128) -> std::string::String {
        let mut s = format!("{}", "");
        while n > 0 {
            let mut rem = n.rem_euclid(3);
            n = n / 3;
            if rem == 2 {
                rem = -1;
                n += 1;
            }
            if rem == 0 {
                s = format!("0{}", s);
            } else {
                if rem == 1 {
                    s = format!("+{}", s);
                } else {
                    s = format!("-{}", s);
                }
            }
        }
        return format!("{:0>4}", s);
    }
}
