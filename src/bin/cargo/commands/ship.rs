use cargo::util::command_prelude::*;

pub fn cli() -> Command {
    clap::Command::new("ship")
        .about("Ship.")
        .override_usage(
            color_print::cstr!("\
       <bright-cyan,bold>cargo ship</>
        "))
        .after_help(color_print::cstr!("Run `<bright-cyan,bold>cargo help ship</>` for more detailed information.\n"))
}

pub fn exec(_: &mut GlobalContext, _: &ArgMatches) -> CliResult {
    println!(color_print::cstr!("<bright-cyan,bold>Cargo</> ship? Car no go ship. Car go vroom."));
    CliResult::Ok(())
}
