use crate::{
	config::Config,
	core::{arguments::Args, exit::Exit, help::build_help},
	display::{CrossTerm, Display},
	input::{EventHandler, KeyBindings},
	process::{exit_status::ExitStatus, modules::Modules, Process},
	todo_file::TodoFile,
	view::View,
};

pub(super) fn load_config() -> Result<Config, Exit> {
	Config::new().map_err(|err| Exit::new(ExitStatus::ConfigError, format!("{:#}", err).as_str()))
}

pub(super) fn load_todo_file(filepath: &str, config: &Config) -> Result<TodoFile, Exit> {
	let mut todo_file = TodoFile::new(filepath, config.undo_limit, config.git.comment_char.as_str());
	if let Err(err) = todo_file.load_file() {
		return Err(Exit::new(ExitStatus::FileReadError, err.to_string().as_str()));
	}

	if todo_file.is_noop() {
		return Err(Exit::new(
			ExitStatus::Good,
			"A noop rebase was provided, skipping editing",
		));
	}

	if todo_file.is_empty() {
		return Err(Exit::new(
			ExitStatus::Good,
			"An empty rebase was provided, nothing to edit",
		));
	}

	Ok(todo_file)
}

pub(super) fn run_process(todo_file: TodoFile, event_handler: EventHandler, config: &Config) -> Exit {
	let display = Display::new(CrossTerm::new(), &config.theme);
	let mut process = Process::new(
		todo_file,
		event_handler,
		View::new(
			display,
			config.theme.character_vertical_spacing.as_str(),
			config
				.key_bindings
				.help
				.first()
				.map_or(String::from("?"), String::from)
				.as_str(),
		),
	);
	match process.run(Modules::new(config)) {
		Ok(status) => Exit::from(status),
		Err(err) => Exit::new(ExitStatus::FileWriteError, err.to_string().as_str()),
	}
}

pub fn run(args: &Args) -> Exit {
	if let Some(filepath) = args.todo_file_path().as_ref() {
		let config = match load_config() {
			Ok(config) => config,
			Err(exit) => return exit,
		};
		let todo_file = match load_todo_file(filepath, &config) {
			Ok(todo_file) => todo_file,
			Err(exit) => return exit,
		};
		let event_handler = EventHandler::new(KeyBindings::new(&config.key_bindings));
		run_process(todo_file, event_handler, &config)
	}
	else {
		Exit::new(
			ExitStatus::StateError,
			build_help(Some(String::from("A todo file path must be provided."))).as_str(),
		)
	}
}
