use clap::ArgMatches;
use crate::interpreter::Interpreter;

pub struct CmdConfig <'a> {
    pub src: &'a str,
    pub size: i32,
}

pub fn handle_config(matches: ArgMatches) {
    if let Some(run) = matches.subcommand_matches("run") {
        let cfg = CmdConfig {
            src: match run.value_of("src") {
                Some(src) => src,
                None => panic!("How did you manage to forget the source file??")
            },
            size: match run.value_of("size") {
                Some(size) => size.parse().unwrap_or(1),
                None => {
                    println!("Did not specify size, defaulting to 1");
                    1
                }
            },
        };
        let mut interp = Interpreter::from_config(&cfg);

        if run.is_present("debug") {
            match interp.debug() {
                Err(e) => panic!("{}", e),
                _ => println!("Exiting debugger")
            }
        } else {
            interp.run();
        }
    } else if let Some(info) = matches.subcommand_matches("info") {
        let cfg = CmdConfig {
            src: match info.value_of("src") {
                Some(src) => src,
                None => panic!("How did you manage to forget the source file??")
            },
            size: match info.value_of("size") {
                Some(size) => size.parse().unwrap_or(1),
                None => {
                    println!("Did not specify size, defaulting to 1");
                    1
                }
            },
        };
        let interp = Interpreter::from_config(&cfg);
        interp.info();
    }
}

