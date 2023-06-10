use async_trait::async_trait;

use crate::lib::{io::io_helper::clear, modify::command::Command};

pub struct ClearCommand;

#[async_trait]
impl Command for ClearCommand {
    async fn run(&self) {
        clear();
    }

    fn description(&self) -> &str {
        "clear the screen"
    }
}
