
# Discord-Bot-Rust

A simple implementation of the serenity.rs crate in rust made with ease-of-use in mind.


## How to use

#### Basic example of making a new instance of DiscordBot.

```rs
use crate::bot::discord_bot::DiscordBot;

#[tokio::main]
async fn main() {
    let discord_bot = DiscordBot::new();
    let mut client = discord_bot.build_client().await.expect("error while making client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

```

#### Adding commands is really simple, go into the ```discord_bot.rs``` file and go to the structure implementation of ```EventHandler```. inside of the ```ready``` function you can address commands to either be global, or guild specific as shown below.

```rs
    async fn ready(&self, ctx: Context, bot: Ready) {
        println!("Connected to bot: {}", bot.user.name);

        //created a GuildID channel using the environment var. (GUILD_ID)
        let guild_id = GuildId::new(
            Util::get_env_var("GUILD_ID")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        //registers guild specific commands.
        let guild_commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
            ])
            .await;

        for guild_command in guild_commands.unwrap() {
            println!("Created Guild Specific Command: {}", guild_command.name)
        }

        //example for creating global commands
        let global_command =
            Command::create_global_command(&ctx.http, commands::test::register())
                .await;

        println!("Created Global Command: {}", global_command.unwrap().name)
    }
```

#### To create commands, you need to make a rust file for said command inside the ```commands``` directory. Here is an example of the ```ping``` command

```ping.rs```

```rs
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
```

#### Adding options to the commands is simplier due to the ```command_extensions.rs``` file. Look inside there for a basic understanding of the options.
