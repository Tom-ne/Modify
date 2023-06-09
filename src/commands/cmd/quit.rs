use async_trait::async_trait;

use crate::lib::modify::command::Command;

pub struct QuitCommand;

#[async_trait]
impl Command for QuitCommand {
    async fn run(&self) {}

    fn description(&self) -> &str {
        "exit Modify"
    }
}
