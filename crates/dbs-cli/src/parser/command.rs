use super::args::*;
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Subcommand};

#[derive(Debug)]
pub enum CliSub {
    Bind(BindArgs),
    Create(CreateArgs),
    Config(ConfigArgs),
}

/// trigger the corresponding command args group
impl FromArgMatches for CliSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("bind", args)) => Ok(Self::Bind(BindArgs::from_arg_matches(args)?)),
            Some(("config", args)) => Ok(Self::Config(ConfigArgs::from_arg_matches(args)?)),
            Some(("create", args)) => Ok(Self::Create(CreateArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
               ErrorKind::UnrecognizedSubcommand,
                "Invalid subcommands, please type `help` to get more assistance."
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Invalid subcommands, please type `help` to get more assistance."
            ))
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("bind", args)) => *self = Self::Bind(BindArgs::from_arg_matches(args)?),
            Some(("config", args)) => *self = Self::Config(ConfigArgs::from_arg_matches(args)?),
            Some(("create", args)) => *self = Self::Create(CreateArgs::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::UnrecognizedSubcommand,
                    "Invalid subcommands, please type `help` to get more assistance."
                ))
            },
            None => (),
        }
        Ok(())
    }
}

/// Commands
impl Subcommand for CliSub {
    fn augment_subcommands(cmd: Command<'_>) -> Command<'_> {
        cmd.subcommand(BindArgs::augment_args(Command::new("bind").display_order(1)))
            .subcommand(CreateArgs::augment_args(Command::new("create").display_order(2)))
            .subcommand(ConfigArgs::augment_args(Command::new("config").display_order(3)))
            .subcommand_required(true)
    }

    fn augment_subcommands_for_update(cmd: Command<'_>) -> Command<'_> {
        CliSub::augment_subcommands(cmd)
    }

    fn has_subcommand(name: &str) -> bool {
        matches!(name, "create" | "config")
    }
}

