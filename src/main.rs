use crate::bot::discord_bot::DiscordBot;

mod bot;
mod utils;

#[tokio::main]
async fn main() {
    let discord_bot = DiscordBot::new();
    let mut client = discord_bot.build_client().await.expect("error while making client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
