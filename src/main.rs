#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate log; // Used for logging
use env_logger::Env;
use roux::Reddit;
#[tokio::main]
async fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    match Reddit::new(
        dotenv!("VYOM_USERAGENT"),
        dotenv!("VYOM_CLIENT_ID"),
        dotenv!("VYOM_CLIENT_SECRET"),
    )
    .username(dotenv!("VYOM_USERNAME"))
    .password(dotenv!("VYOM_PASSWORD"))
    .login()
    .await
    {
        // Try to make a new client with the credentials
        Ok(client) => match client.inbox().await {
            // Fetch the inbox of the logged in user
            Ok(listing) => {
                for message in listing.data.children.iter() {
                    if message.data.new && message.data.r#type == "username_mention" {
                        match client
                            .comment(
                                "You have been Noted by Vyom. Please Stand By!",
                                &message.data.name.as_str(),
                            )
                            .await
                        {
                            Ok(_) => info!("Replied to {}", message.data.name),
                            Err(_) => error!("Failed to reply to mention"),
                        };
                    }
                }
            }
            Err(_) => {
                error!("Failed to fetch messages");
            }
        },
        Err(e) => panic!(e),
    }
}
