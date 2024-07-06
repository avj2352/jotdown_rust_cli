use clap::{ Command };
/**
 * Consists of "jd" - update commands
 * LIST ==============================================================
 */

/**
 * UPDATE command
 * update 1 "new description"
 * @returns {Command} returns subcommand
 */
pub fn gen_update_command() -> Command {
    Command::new("update")
        .about("update todo - description")
        .arg(clap::Arg::new("idx").index(1))
        .arg(clap::Arg::new("desc").index(2))
}


/**
 * UPDATE command
 * update 1 "new description"
 * @returns {Command} returns subcommand
 */
pub fn gen_update_tag_command() -> Command {
    Command::new("tag")
        .about("update todo - tag")
        .arg(clap::Arg::new("idx").index(1))
        .arg(clap::Arg::new("tag").index(2))
}


