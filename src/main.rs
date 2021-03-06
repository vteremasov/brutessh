extern crate ssh2;
extern crate clap;

mod cli;
mod ssh;
mod port;
mod word;

fn main() {
    match port::scan("localhost", "22") {
        Some(err) => panic!(err),
        None => {},
    };

    let settings = cli::Settings::parse();

    penetrate(settings)
}

fn penetrate(settings: cli::Settings) {
   let mut char_list: Vec<String> = settings.char_list
       .split("")
       .map(|s| String::from(s))
       .collect::<Vec<String>>();

    char_list.drain(0..1); // remove first space
    char_list.drain(char_list.len()-1..); // remove last space

    let v = word::Variations::new(char_list, 4);
   
    let mut h = ssh::Handler::default();
    let mut c = ssh::Config::default();
    c.set_host(settings.address);
    c.set_port(settings.port);
    h.set_config(c);
    
    match h.connect() {
        Some(err) => panic!(err),
        None => {},
    };

    for w in v {
        match h.check(&settings.user[..], &w[..]) {
            Err(_) => {},
            Ok(res) => if res {
                println!("{}", w);
            },
        }
    }

    match h.disconnect() {
        Some(err) => panic!(err),
        None => {},
    }
}
