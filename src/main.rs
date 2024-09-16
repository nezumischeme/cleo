use std::env;

use serenity::async_trait;
use serenity::builder::{
    CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Recieved command interaction: {command:#?}");
            // Since there's only one command right now, we can respond to everything with "Pong!".
            // Should be fine?
            let data = CreateInteractionResponseMessage::new().content("Pong!");
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("Guild id must be an integer"),
        );
        let ping_command = CreateCommand::new("ping").description("A ping command");
        let _commands = guild_id.set_commands(&ctx.http, vec![ping_command]).await;
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
