use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

pub trait CreateCommandExt {
    fn add_integer_option(&mut self, name: &str, description: &str, min: u64, max: u64, required: bool) -> &mut Self;
}

impl CreateCommandExt for CreateCommand {
    fn add_integer_option(self, name: &str, description: &str, min: u64, max: u64, required: bool) -> Self {
        self.add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                name,
                description
            )
                .min_int_value(min)
                .max_int_value(max)
                .required(required),
        )
    }
}