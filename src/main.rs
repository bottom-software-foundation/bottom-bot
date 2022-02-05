use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    macros::group,
};
use regex::RegexBuilder;

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
            let matcher = RegexBuilder::new(format!("^{}", prefix).as_str()).case_insensitive(true)
            .build().expect("Invalid regex");
            if matcher.is_match_at(&msg.content, 0) {
                #[allow(unused_assignments)]
                let mut reply = String::new();
                if let Some(rmsg) = &msg.referenced_message {
                    reply = translate(&rmsg.content);
                } else {
                    let mut input: String =  matcher.replace(&msg.content, "").to_string();
                    if input.starts_with(" ") {
                        input = input.strip_prefix(" ").unwrap().to_string();
                    }
                    reply = translate(&input.to_string());
                }
                if reply.len() > 2000 {
                    reply = "its too big!! it wont fit!".to_string()
                } else if reply.is_empty() {
                    reply = "uwu?".to_string()
                }
                if let Err(why) = msg.reply(&ctx, reply).await {
                    println!("Error sending message: {:?}", why);
                }
                break;
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