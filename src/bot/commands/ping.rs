use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use crate::bot::command_extensions::CreateCommandExt;

pub fn run(_options: &[ResolvedOption]) -> String {
    "pong".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
        .add_integer_option("ping_testing", "test my dad", 0, 100, true)
}