use serenity::{
    model::permissions::Permissions,
    prelude::*,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    assert!(dotenv::dotenv().is_ok());
    env_logger::init();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = wallbot::Handler::new(&wallbot::HandlerOptions {
        min_perm: {
            let bits = std::env::var("DISCORD_PERM").expect("Expected a DISCORD_PERM")
                .parse().expect("Expected DISCORD_PERM to be a u64");
            Permissions::from_bits(bits)
                .expect("Expected DISCORD_PERM to be valid Permissions bits")
        },
        prefix: std::env::var("PREFIX").expect("Expected a PREFIX").leak(),
    });
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

    Ok(client.start().await?)
}
