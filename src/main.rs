use std::env;
use data_reader::return_response;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use dotenvy::dotenv;
mod data_reader;
use data_reader::read_data_from_file;

struct Handler;


// called whenever the new message is received
// dispatched through thread pool
#[async_trait]
impl EventHandler for Handler {
    
    async fn message(&self, ctx:Context, msg:Message) {
        // read the phrases and commands from the json
        match read_data_from_file() {
            Ok(data) => { 

                let response = return_response(&data, &msg.content);

                // making sure that it doesn't create an infinite loop with itself                
                let bot_id = ctx.http.get_current_user().await.unwrap().id;

                if !response.is_empty() &&  msg.author.id != bot_id {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                        println!("Error sending message: {why:?}");
                    }
                }
    
        }
            Err(e) => println!("Error reading data: {}", e),
        }
    
    }

    // print when live
    async fn ready(&self, _: Context, ready:Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

    dotenv().ok();
    let token= env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler).await.expect("Error creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
