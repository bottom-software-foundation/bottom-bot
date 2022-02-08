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
            // Get config file
            config: Config::get(),
        };
        handler
    }
}

fn translate(string: &String) -> String {
    // Attempt to decode bottomspec string
    match decode_string(&string) {
        // If it was decoded, return the decoded string
        Ok(out) => {
            return out;
        },
        // If it wasn't decoded, encode the string as bottomspec and return it
        _ => {
            return encode_string(&string);
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    // On message received
    async fn message(&self, ctx: Context, msg: Message) {
        // Ensure message isn't own
        if !msg.is_own(&ctx).await {
            for prefix in &self.config.prefixes {
                // Create regex matcher for the prefix
                let matcher = RegexBuilder::new(format!("^{}", prefix).as_str()).case_insensitive(true)
                .build().expect("Invalid regex");
                
                // Check if message begins with regex
                if matcher.is_match_at(&msg.content, 0) {
    
                    // Ensure chars variable is in scope
                    #[allow(unused_assignments)]
                    let mut chars: Vec<char> = Vec::new();
                        // If message is a reply to a message
                    if let Some(rmsg) = &msg.referenced_message {
                        // Translate and push to vector
                        chars = translate(&rmsg.content.to_string()).chars().collect();
                    } else {
                        // Remove prefix 
                        let input: String = matcher.replace(&msg.content, "").trim_start().to_string();
                        // Translate and push to vector
                        chars = translate(&input.to_string()).chars().collect();
                    }
    
                    // Create vector of messages to be sent
                    let mut replies: Vec<Vec<char>> = Vec::new();
    
                    // While the last message in the list is still over 2000 characters, split it 
                    while chars.len() > 2000 {
                        // Split after 2000 characters into the left and right parts of the message
                        let (left, right) = chars.split_at(2000);
                        // Get the position of the last 👈 character
                        let pos = left.iter().rposition(|&r| r == '👈').unwrap() + 1;
                        // Split the left part of the message to the last 👈 and the remainder
                        // Ensuring the last character is always a 👈 makes sure no data is lost after splitting messages
                        let (reply, remainder) = left.split_at(pos);
                        // Push the message up until the 👈 to the to-send list
                        replies.push(reply.to_vec());
                        // Create a vector from the remainder
                        let mut remainder = remainder.to_vec();
                        // Append the leftover characters to the right of the 2000th character to the remainder
                        remainder.append(&mut right.to_vec());
                        // Overwrite what's left to do with the new remainder
                        chars = remainder;
                    }

                    // Append the remainder
                    replies.push(chars);
    
                    // Send every reply
                    for reply in &replies {
                        // If this is the first message in the list, send it as a reply
                        if reply == replies.get(0).unwrap() {
                            // Ensure message isn't empty. The second message should never be able to be null
                            if !reply.is_empty() {
                                if let Err(why) = msg.reply(&ctx, reply.iter().collect::<String>()).await {
                                    println!("Error sending message: {:?}", why);
                                }
                            }
                        // Send the rest as messages in the channel, looks cleaner and avoids ping spam
                        } else {
                            if let Err(why) = msg.channel_id.say(&ctx, reply.iter().collect::<String>()).await {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                    }
                    // Break loop, prefix was found
                    break;
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