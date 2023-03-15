use log::LevelFilter;
use structopt::StructOpt;

#[derive(StructOpt)]
/// Replace the content of the ~/.aws directory with either ~/.aws.v1 or ~/.aws.v2 in order to switch
/// between file compatible with either the AWS java SDK v1 and the AWS java SDK v2.
///
/// Also keeps the aws_access_key_id and aws_secret_access_key properties when making the switch.
pub struct Cli {
    #[structopt(short, long="log-level", possible_values = &level_filters(), case_insensitive = true, default_value="DEBUG")]
    /// Select log level
    pub log_level: LevelFilter,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    UseV1,
    UseV2,
    UseV3
}

fn level_filters() -> [&'static str; 6] {
    [
        LevelFilter::Off.as_str(),
        LevelFilter::Error.as_str(),
        LevelFilter::Warn.as_str(),
        LevelFilter::Info.as_str(),
        LevelFilter::Debug.as_str(),
        LevelFilter::Trace.as_str(),
    ]
}
