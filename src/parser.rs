use clap::{Arg, Command, value_parser};

pub struct Request {
    pub(crate) typex: String,
    pub(crate) valuex: String
}

#[allow(dead_code)]
pub fn initialize() -> Request {
    let app = Command::new("ProSint")
        .version("1.0")
        .author("KodakSec")
        .about("Github/KodakSec")
        .arg(
            Arg::new("mail")
                .conflicts_with("ip")
                .short('m')
                .long("mail")
                .value_parser(value_parser!(String))
                .help_heading("GENERAL options")
                .help("Gather information on a ProtonMail"),
        )
        .arg(
            Arg::new("ip")
                .conflicts_with("mail")
                .short('i')
                .long("ip")
                .value_parser(value_parser!(String))
                .help_heading("GENERAL options")
                .help("Check if IP is from ProtonVPN"),
        )

        .get_matches();

    let mut typex = String::new();
    let mut valuex = String::new();

    if let Some(cli_value) = app.get_one::<String>("mail") {
        typex = "mail".to_string();
        valuex = cli_value.to_string();
    }

    if let Some(cli_value) = app.get_one::<String>("ip") {
        typex = "ip".to_string();
        valuex = cli_value.to_string();
    }

    let final_req = Request {
        typex,
        valuex,
    };

    final_req

}