use clap::{value_parser, Arg, Command};

pub fn get_port() -> Option<usize> {
    let matches = build_command().get_matches();

    if let Some(val) = matches.get_one("port") {
        Some(*val)
    } else {
       None
    }
}


fn build_command() -> Command {
    Command::new("ccmemcached")
        .author("Bayram, bkulyev@gmail.com")
        .version("0.1.0")
        .about("mini Memcached server")
        .arg(
            Arg::new("port")
                .required(false)
                .value_parser(value_parser!(usize))
                .short('p')
                .long("port")
                .help("Port to accept incoming TCP connections)"),
        )
}