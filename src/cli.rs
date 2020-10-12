use clap::{App, Arg};

struct Settings {
    ip_address: String,
}

impl Settings {
    fn parser() -> Self {
        let matches = App::new("brutessh")
            .version("0.0.1")
            .author("github.com/PumpkinSeed")
            .about("SSH brute-force, penetration tester tool")
            .arg(
                Arg::with_name("ip-address")
                    .short("a")
                    .long("ip-address")
                    .value_name("ADDR")
                    .help("Sets the vulnerable IP address")
                    .takes_value(true),
            )
            .get_matches();
        let ip_address = matches.value_of("ip-address").unwrap_or("localhost");
        Settings {
            ip_address: String::from(ip_address),
        }
    }
}
