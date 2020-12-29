mod util;
use clap::{App, AppSettings, Arg};
use num::BigInt;
use util::cbb;

fn main() {
    let matches = App::new("cbb")
        .version("0.1.6")
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
            let big_n: BigInt = number.parse::<BigInt>().unwrap_or_else(|_| {
                println!("Error while parsing BigInt...");
                std::process::exit(1_i32);
            });
            if big_n.ge(&BigInt::from(i128::MAX)) {
                println!("Please specify a number < {}", i128::MAX);
                std::process::exit(-1i32);
            }

            let n = number.parse::<i128>().unwrap_or_else(|_| {
                println!("Error while parsing...");
                std::process::exit(1_i32);
            });
            println!("{}", cbb::int_to_bal_ternary(n));
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
