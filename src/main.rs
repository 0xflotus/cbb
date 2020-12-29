use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("hcal")
        .version("0.1.0")
        .about("A converter for numbers")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(
            Arg::new("target")
                .short('t')
                .long("to")
                .takes_value(true)
                .value_name("base")
                .about("Sets the target base."),
        )
        .arg(
            Arg::new("balanced-ternary")
                .short('b')
                .long("balanced-ternary")
                .takes_value(false)
                .about("Converts decimal to balanced ternary"),
        )
        .arg(
            Arg::new("number")
                .about("Sets the number")
                .required(true)
                .index(1_u64),
        )
        .get_matches();

    if matches.is_present("balanced-ternary") {
        if let Some(number) = matches.value_of("number") {
            let mut n = number.parse::<i32>().unwrap_or_else(|_| {
                println!("Error while parsing...");
                std::process::exit(1_i32);
            });

            if n < 0 {
                println!("Please specify a number >= 0");
                std::process::exit(-1i32);
            }

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
            println!("{}", format!("{:0>4}", s));
            std::process::exit(0i32);
        } else {
            println!("Please provide a decimal number!");
            std::process::exit(-1i32);
        }
    }

    let mut target_base = 0x10_u8;
    if let Some(target) = matches.value_of("target") {
        let new_target_base = target.parse::<u8>().unwrap_or_else(|_| {
            println!("Error while parsing...");
            std::process::exit(1_i32);
        });
        if new_target_base > 1_u8 && new_target_base <= 0x24_u8 {
            target_base = new_target_base;
        } else {
            println!("Base should between [1;36)");
            std::process::exit(1_i32);
        }
    }
    if let Some(number) = matches.value_of("number") {
        let the_number = number.parse::<u32>().unwrap_or_else(|_| {
            println!("Error while parsing...");
            std::process::exit(1_i32);
        });
        println!("{}", radix_fmt::radix(the_number, target_base));
    }
}
