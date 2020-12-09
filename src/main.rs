extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    const INSTALL: &str = "install";
    const START: &str = "start";
    const PS: &str = "ps";
    const PROCESS: &str = "process";
    const PACKAGE: &str = "package";

    let matches = App::new("Kiln")
        .version("0.1")
        .about("Package manager for the Tezos Ecosystem")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name(INSTALL)
                .about("Install a package")
                .arg(
                    Arg::with_name(PACKAGE)
                        .value_name("PACKAGE")
                        .required(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            SubCommand::with_name(START)
                .about("Start a package process")
                .arg(Arg::with_name(PROCESS).value_name("PROCESS").required(true)),
        )
        .subcommand(SubCommand::with_name(PS).about("Get running processes"))
        .get_matches();

    match matches.subcommand() {
        (INSTALL, Some(_)) => {}
        (START, Some(sub_m)) => {
            println!("Starting {}... started", sub_m.value_of(PROCESS).unwrap())
        }
        (PS, Some(_)) => {
            println!("tezedge: Running as PID=4026\nbaker: Running as PID=3332")
        }
        _ => {}
    }
}
