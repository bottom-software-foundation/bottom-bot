use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    macros::group,
};

use bottomify::bottom::{encode_string, decode_string};

#[group]
struct General;

struct Handler;

fn translate(string: &String) -> String {
    match decode_string(&string) {
        Ok(out) => {
            return out;
        },
        _ => {
            return encode_string(&string);
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "🥺" {
            if let Some(rmsg) = &msg.referenced_message {
                let reply = translate(&rmsg.content);
                if let Err(why) = msg.reply(&ctx, reply).await {
                    println!("Error sending message: {:?}", why);
                }
    
            }
        }
    }
}

#[tokio::main]
async fn main() {

    let framework = StandardFramework::new()
        .group(&GENERAL_GROUP);

    // Login with a bot token from the config file
    let mut client = Client::builder(env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in env"))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("{}\nAn error occurred while running the client... exiting", why);
    }
}