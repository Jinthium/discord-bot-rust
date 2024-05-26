use serenity::all::{ResolvedOption, ResolvedValue};
use serenity::builder::CreateCommand;
use crate::bot::command_extensions::CreateCommandExt;

pub fn register() -> CreateCommand {
    CreateCommand::new("test").description("test comomand")
        .add_attachment_option("test_attachment", "attachey attachmento", true)
}

pub fn run(_options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {value: ResolvedValue::Attachment(attachment), ..}) = _options.first() {
        format!("Attachment name: {}, attachment size is about: {:.2}mb", attachment.filename, attachment.size as f64 / 1_000_000.0)
    }else{
        "Attachment not valid".to_string()
    }
}