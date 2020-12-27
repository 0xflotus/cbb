use clap::{App, Arg};

fn main() {
    let matches = App::new("hcal")
        .version("0.1.0")
        .about("A converter for numbers")
        .arg(
            Arg::new("target")
                .short('t')
                .long("to")
                .takes_value(true)
                .value_name("base")
                .about("Sets the target base."),
        )
        .arg(
            Arg::new("number")
                .about("Sets the number")
                .required(true)
                .index(1_u64),
        )
        .get_matches();

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
