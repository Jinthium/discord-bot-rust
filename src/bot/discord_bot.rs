use serenity::{async_trait, Client, Error};
use serenity::all::{Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler, GuildId, Interaction, Ready};
use serenity::prelude::GatewayIntents;
use crate::bot::commands;
use crate::utils::util::Util;

// structure for holding the EventHandler implementation.
struct Handler;
// structure for holding the DiscordBot implementation.
pub struct DiscordBot;

// struct implementation for the DiscordBot
// can be called using DiscordBot::new() provided you have the BOT_TOKEN env-var set.
impl DiscordBot {
    pub fn new() -> Self {
        Self { }
    }

    /// creates the client for the bot with the intents of GatewayIntents::MESSAGE_CONTENT.
    /// make sure to change your intents to match what you have on the developer api panel.
    pub async fn build_client(&self) -> Result<Client, Error> {
        return Ok(Client::builder(Self::bot_token(), GatewayIntents::MESSAGE_CONTENT).event_handler(Handler).await.expect("Error creating client"))
    }

    /// grabs the BOT_TOKEN environment variable from the runtime.
    fn bot_token() -> String {
        return Util::get_env_var("BOT_TOKEN")
    }
}

#[async_trait]
impl EventHandler for Handler {
    ///called when the bot is ready and the client is connected.
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

    ///creates the interaction between the user of the command and the bot.
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        //beginning of command interaction
        if let Interaction::Command(command) = interaction {
            println!("Received Command request \"{}\" from user: {}", command.data.name, command.user.name);

            //the content from the interaction, aka what command was used and its arguments (ex: /ping)
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "test" => Some(commands::test::run(&command.data.options())),
                _ => Some("Not implemented yet".to_string()),
            };

            //creates the response to send back to the user, aka the end of the interaction
            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);

                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}