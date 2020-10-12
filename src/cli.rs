use clap::{App, Arg};

pub struct Settings {
    pub address: String,
    pub port: String,
    pub char_list: String,
    pub user: String,
}

impl Settings {
    pub fn parse() -> Self {
        let matches = App::new("brutessh")
            .version("0.0.1")
            .author("github.com/PumpkinSeed")
            .about("SSH brute-force, penetration tester tool")
            .arg(
                Arg::with_name("address")
                    .short("a")
                    .long("address")
                    .value_name("ADDR")
                    .help("Sets the vulnerable IP address")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("PORT")
                    .help("Sets the vulnerable system's port")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("chars")
                    .short("c")
                    .long("chars")
                    .value_name("CHARS")
                    .help("Character-set of the penetration")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("user")
                    .short("u")
                    .long("user")
                    .value_name("USER")
                    .help("Sets the vulnerable system's username")
                    .takes_value(true),
            )
            .get_matches();
        let address = matches.value_of("address").unwrap_or("localhost");
        let port = matches.value_of("port").unwrap_or("22");
        let chars = matches.value_of("chars").unwrap_or("1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let user = matches.value_of("user").unwrap_or("root");
        Settings {
            address: String::from(address),
            port: String::from(port),
            char_list: String::from(chars),
            user: String::from(user),
        }
    }
}
