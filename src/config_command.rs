use clap::{Arg, App, ArgMatches};

pub fn get_command() -> ArgMatches<'static>{
    App::new("Wallet Registe system parameter configmation")
            .version("0.1.0")
            .author("luo4lu <luo4lu@163.com>")
            .about("Go to the server and request the address")
            .arg(Arg::with_name("wrs")
                .short("w")
                .long("wrs")
                .help("set self Wallet Registe system IP addr and port")
                .takes_value(true))
            .get_matches()

}