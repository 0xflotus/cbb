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

    pub fn int_to_unbal_ternary(n: i128) -> std::string::String {
        let e = n / 0x03;
        let q = n % 0x03;
        let s: std::string::String;
        if n == 0x00 {
            s = format!("{:04}", 0x00);
        }
        else if e == 0x00 {
            s = format!("{:04}", q);
        }
        else {
            s = format!("{:04}", format!("{}{}", int_to_unbal_ternary(e), q));
        }
        let i: i128 = s.parse::<i128>().unwrap();
        return format!("{:04}", i);
    }
}
