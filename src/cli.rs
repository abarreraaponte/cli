use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ctx", version, about = "ContextDrive client on this machine")]
pub struct Ctx {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Command {
    /// Sign in through the browser and store a credential.
    Login,
    /// Walk a local tree and upload it through a grant.
    Ingest,
    /// Read an upload job.
    Status,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotBuilt {
    pub command: &'static str,
}

impl std::fmt::Display for NotBuilt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not built", self.command)
    }
}

impl std::error::Error for NotBuilt {}

pub fn run() -> Result<(), NotBuilt> {
    dispatch(Ctx::parse().command)
}

pub fn dispatch(command: Command) -> Result<(), NotBuilt> {
    let command = match command {
        Command::Login => "ctx login",
        Command::Ingest => "ctx ingest",
        Command::Status => "ctx status",
    };
    Err(NotBuilt { command })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_parse_and_are_not_built() {
        for (args, name) in [
            (["ctx", "login"], "ctx login"),
            (["ctx", "ingest"], "ctx ingest"),
            (["ctx", "status"], "ctx status"),
        ] {
            let parsed = Ctx::try_parse_from(args).unwrap();
            let err = dispatch(parsed.command).unwrap_err();
            assert_eq!(err.command, name);
        }
    }
}
