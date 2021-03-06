use crate::{
	input::EventHandler,
	process::{process_result::ProcessResult, state::State},
	todo_file::TodoFile,
	view::{render_context::RenderContext, view_data::ViewData, ViewSender},
};

pub trait ProcessModule {
	fn activate(&mut self, _rebase_todo: &TodoFile, _previous_state: State) -> ProcessResult {
		ProcessResult::new()
	}

	fn deactivate(&mut self) {}

	fn build_view_data(&mut self, _render_context: &RenderContext, _rebase_todo: &TodoFile) -> &ViewData;

	fn handle_events(
		&mut self,
		_event_handler: &EventHandler,
		_view_sender: &ViewSender,
		_rebase_todo: &mut TodoFile,
	) -> ProcessResult;
}
