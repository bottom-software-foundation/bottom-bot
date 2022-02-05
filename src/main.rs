use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    macros::group,
};

use bottomify::bottom::{encode_string, decode_string};

mod config;
use config::Config;

#[group]
struct General;

struct Handler {
    // Config for the bot
    config: Config,
}

impl Handler {
    pub fn new() -> Handler {
        let handler = Handler {
            config: Config::get(),
        };
        handler
    }
}

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
        for prefix in &self.config.prefixes {
            if msg.content.to_lowercase() == prefix.to_lowercase() {
                if let Some(rmsg) = &msg.referenced_message {
                    let reply = translate(&rmsg.content);
                    if let Err(why) = msg.reply(&ctx, reply).await {
                        println!("Error sending message: {:?}", why);
                    }
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
    let mut client = Client::builder(Config::get().bot_token)
        .event_handler(Handler::new())
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("{}\nAn error occurred while running the client... exiting", why);
    }
}