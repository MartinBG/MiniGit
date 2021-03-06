use std::fmt;
use std::fmt::Display;
use ::checkout::checkout;
use ::cherry_pick::cherry_pick;
use ::commit::commit;
use init::init;
use log::log;
use revert::revert;
use status::status;


//Too many arguments parameter shows the expected number of arguments
#[derive(Debug)]
pub enum CommandParseError {
	TooManyArguments(u32),
	ArgumentMissing(String),
	WrongCommand(String),
	NoCommand
}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match self {
            &CommandParseError::TooManyArguments(expected_arguments) => 
            			write!(f, "Too many arguments specified. Expected: {}", expected_arguments),
            &CommandParseError::ArgumentMissing(ref arg) => write!(f, "Expected parameter: {}", arg),
            &CommandParseError::WrongCommand(ref cmd) => write!(f, "Ivalid command {} given", cmd),
            &CommandParseError::NoCommand => write!(f, "Expected command!\n(init, log, revert, status, checkout, commit, cherry_pick)"),
        }
    }
}

#[derive(Debug)]
pub enum Command {
	Init,
	Log,
	Revert,
	Status,
	Checkout(String),
	Commit(String),
	CherryPick(String, String),
}

pub fn parse_command(args: &Vec<String>) -> Result<Command, CommandParseError> {
	
	match handle_parse_error(args) {
		Ok(_) => (),
		Err(err) => return Err(err)
	}

	match args[1].as_ref() {
		"init" => Ok(Command::Init),
		"log" => Ok(Command::Log),
		"revert" => Ok(Command::Revert),
		"status" => Ok(Command::Status),
		"checkout" => Ok(Command::Checkout(String::from(args[2].clone()))),
		"commit" => Ok(Command::Commit(String::from(args[2].clone()))),
		"cherry_pick" => Ok(Command::CherryPick(String::from(args[2].clone()), String::from(args[3].clone()))),
		_ => Err(CommandParseError::NoCommand)
	}
}

fn handle_parse_error(args: &Vec<String>) -> Result<bool, CommandParseError> {
	let no_arg_cmd: Vec<&str> = vec!("init", "log", "revert", "status");
	let one_arg_cmd: Vec<&str> = vec!("checkout", "commit");
	let commit_id_cmd: Vec<&str> = vec!("checkout", "cherry_pick");
	let all_cmd: Vec<&str> = vec!("init", "log", "revert", "status", "checkout", "commit", "cherry_pick");

	if args.len() == 1 {
		return Err(CommandParseError::NoCommand)
	}

	if args.len() > 2 && no_arg_cmd.contains(&&args[1][..]) {
		return Err(CommandParseError::TooManyArguments(0))
	}

	if args.len() > 3 && one_arg_cmd.contains(&&args[1][..])  {
		return Err(CommandParseError::TooManyArguments(1))
	}

	if args.len() > 4 && args[1] == "cherry_pick" {
		return Err(CommandParseError::TooManyArguments(2))
	}

	if args.len() == 2 && commit_id_cmd.contains(&&args[1][..]) {
		return Err(CommandParseError::ArgumentMissing(String::from("commit id")));
	}

	if args.len() == 2 && args[1] == "commit" {
		return Err(CommandParseError::ArgumentMissing(String::from("message")));
	}

	if args.len() == 3 && args[1] == "cherry_pick" {
		return Err(CommandParseError::ArgumentMissing(String::from("file path")));
	}

	if !all_cmd.contains(&&args[1][..]) {
		return Err(CommandParseError::WrongCommand(String::from(args[1].clone())));
	}

	Ok(true)
}

pub fn run_command(cmd: Command, path: &String) {
	match cmd {
		Command::Init => init(path),
		Command::Log => log(path),
		Command::Revert => revert(path),
		Command::Status => status(path),
		Command::Checkout(commit_id) => checkout(path, &commit_id),
		Command::Commit(message) => commit(path, &message),
		Command::CherryPick(commit_id, filepath) => cherry_pick(path, &commit_id, &filepath),
		_ => panic!("Wrong command passed in run_command")
	}
}