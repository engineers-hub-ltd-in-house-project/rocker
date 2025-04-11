use clap::{App, AppSettings, Arg, SubCommand};
use std::error::Error;

mod commands;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("rocker")
        .version("0.1.0")
        .author("Rocker Team")
        .about("Container management system written in Rust")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Run a container")
                .arg(
                    Arg::with_name("detach")
                        .short("d")
                        .long("detach")
                        .help("Run container in background"),
                )
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .takes_value(true)
                        .help("Assign a name to the container"),
                )
                .arg(
                    Arg::with_name("volume")
                        .short("v")
                        .long("volume")
                        .takes_value(true)
                        .multiple(true)
                        .help("Bind mount a volume"),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .takes_value(true)
                        .multiple(true)
                        .help("Publish a container's port to the host"),
                )
                .arg(
                    Arg::with_name("image")
                        .required(true)
                        .help("The image to run"),
                )
                .arg(
                    Arg::with_name("command")
                        .multiple(true)
                        .help("Command to run"),
                ),
        )
        .subcommand(
            SubCommand::with_name("ps")
                .about("List containers")
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Show all containers (default shows just running)"),
                ),
        )
        .subcommand(
            SubCommand::with_name("images")
                .about("List images")
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Show all images (default hides intermediate images)"),
                ),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Build an image from a Rockerfile")
                .arg(
                    Arg::with_name("tag")
                        .short("t")
                        .long("tag")
                        .takes_value(true)
                        .help("Name and optionally a tag in the 'name:tag' format"),
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .help("Name of the Rockerfile (Default is 'Rockerfile')"),
                )
                .arg(
                    Arg::with_name("path")
                        .default_value(".")
                        .help("Path to the build context"),
                ),
        )
        .subcommand(
            SubCommand::with_name("compose")
                .about("Define and run multi-container applications")
                .subcommand(
                    SubCommand::with_name("up")
                        .about("Create and start containers")
                        .arg(
                            Arg::with_name("detach")
                                .short("d")
                                .long("detach")
                                .help("Detached mode: Run containers in the background"),
                        )
                        .arg(
                            Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .takes_value(true)
                                .help("Specify an alternate compose file"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("down")
                        .about("Stop and remove containers, networks, images, and volumes")
                        .arg(
                            Arg::with_name("volumes")
                                .long("volumes")
                                .help("Remove named volumes declared in the volumes section"),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(run_matches)) => {
            commands::run::execute(run_matches)?;
        }
        ("ps", Some(ps_matches)) => {
            commands::ps::execute(ps_matches)?;
        }
        ("images", Some(images_matches)) => {
            commands::images::execute(images_matches)?;
        }
        ("build", Some(build_matches)) => {
            commands::build::execute(build_matches)?;
        }
        ("compose", Some(compose_matches)) => {
            match compose_matches.subcommand() {
                ("up", Some(up_matches)) => {
                    commands::compose::up::execute(up_matches)?;
                }
                ("down", Some(down_matches)) => {
                    commands::compose::down::execute(down_matches)?;
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }

    Ok(())
} 
