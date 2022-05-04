use clap::{Arg, Command};

enum Subcmd {
    Edit,
    Copy,
    Read,
    KeyList,
}

struct Conf {
    subcmd: Subcmd,
    key_mode: bool,
}

impl Conf {
    fn new(subcmd_str: &str, key_mode: bool) -> Conf {
        Conf {
            subcmd: Conf::parse_subcmd(subcmd_str),
            key_mode,
        }
    }
    fn parse_subcmd(cmd_str: &str) -> Subcmd {
        match cmd_str {
            "edit" => Subcmd::Edit,
            "copy" => Subcmd::Copy,
            "read" => Subcmd::Read,
            "keylist" => Subcmd::KeyList,
            _ => panic!("unexpected subcommand"),
        }
    }
}

fn main() {
    let matches = Command::new("jiji")
        .version("0.0.1")
        .author("Kodai Kabasawa <kabaaa1126@gmail.com>")
        .about("read / edit /copy JSON")
        .subcommand_required(true)
        .subcommand(Command::new("edit"))
        .subcommand(Command::new("copy"))
        .subcommand(Command::new("read"))
        .subcommand(Command::new("keylist"))
        .arg(
            Arg::new("key")
                .short('k')
                .help("edit / copy / read JSON key mode. default is value mode"),
        )
        .get_matches();
    let (subcmd_str, args) = matches.subcommand().unwrap();
    let key_mode = args.value_of("key").is_some();
    let conf = Conf::new(subcmd_str, key_mode);
}

#[cfg(test)]
mod test {
    #[test]
    fn read_json() {}
}
