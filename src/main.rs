mod googlebooks;

use std::env;

use serenity::async_trait;
use serenity::builder::{
    CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::model::application::Interaction;
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Recieved command interaction: {command:#?}");
            // Since there's only one command right now, we have only the one command handler here.
            // TODO: Come up with a smart way to avoid passing in the key here.
            let api_key = env::var("API_KEY").expect("Expected API_KEY in environment");
            if let Some(ResolvedOption {
                value: ResolvedValue::String(query),
                ..
            }) = command.data.options().first()
            {
                let volumes =
                    googlebooks::search_volumes(query.to_string(), api_key.to_string()).await;
                if let Err(why) = volumes {
                    let data = CreateInteractionResponseMessage::new().content(why.to_string());
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        println!("Cannot respond to slash command: {why}");
                    }
                } else {
                    let vs = volumes.unwrap().items;
                    if let Some(v) = vs.first() {
                        let v_info = &v.volume_info;
                        let description = v_info
                            .description
                            .clone()
                            .unwrap_or("No description".to_string());
                        let image = googlebooks::highest_quality_image(&v_info.image_links)
                            .unwrap_or("".to_string());
                        let embed = CreateEmbed::new()
                            .title(&v_info.title)
                            .description(description)
                            .image(image);
                        let data = CreateInteractionResponseMessage::new()
                            .content(&v_info.title)
                            .embed(embed);
                        let builder = CreateInteractionResponse::Message(data);
                        if let Err(why) = command.create_response(&ctx.http, builder).await {
                            println!("Cannot respond to slash command: {why}")
                        }
                    }
                }
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
        let search_option = CreateCommandOption::new(
            CommandOptionType::String,
            "input",
            "Search Google Books for all volumes",
        )
        .required(true);
        let search_command = CreateCommand::new("search")
            .description("Search!")
            .add_option(search_option);
        let commands = guild_id.set_commands(&ctx.http, vec![search_command]).await;

        println!("I now have the following guild commands: {commands:#?}");
    }
}

async fn bot() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[tokio::main]
async fn main() {
    bot().await;
}
