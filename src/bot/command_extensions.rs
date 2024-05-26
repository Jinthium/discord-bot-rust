use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};

//base trait to use for the extensions
pub trait CreateCommandExt {
    fn add_integer_option(self, name: &str, description: &str, min: u64, max: u64, required: bool) -> Self;
    fn add_number_option(self, name: &str, description: &str, min: f64, max: f64, required: bool) -> Self;
    fn add_attachment_option(self, name: &str, description: &str, required: bool) -> Self;
    fn add_user_option(self, name: &str, description: &str, required: bool) -> Self;
}

// extensions to the serenity library for ease-of-use in command options.
impl CreateCommandExt for CreateCommand {

    ///simplifies the process of creating an integer option to the command.
    fn add_integer_option(self, name: &str, description: &str, min: u64, max: u64, required: bool) -> Self {
        self.add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                name,
                description,
            )
                .min_int_value(min)
                .max_int_value(max)
                .required(required),
        )
    }

    ///simplifies the process of creating a number option to the command.
    fn add_number_option(self, name: &str, description: &str, min: f64, max: f64, required: bool) -> Self {
        self.add_option(
            CreateCommandOption::new(
                CommandOptionType::Number,
                name,
                description,
            )
                .min_number_value(min)
                .max_number_value(max)
                .required(required)
        )
    }

    ///simplifies the process of creating an attachment option to the command.
    fn add_attachment_option(self, name: &str, description: &str, required: bool) -> Self {
        self.add_option(
            CreateCommandOption::new(
                CommandOptionType::Attachment,
                name,
                description,
            ).required(required)
        )
    }

    ///simplifies the process of creating a user option to the command.
    fn add_user_option(self, name: &str, description: &str, required: bool) -> Self {
        self.add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                name,
                description,
            ).required(required)
        )
    }
}