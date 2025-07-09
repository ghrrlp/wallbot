use serenity::{
    model::permissions::Permissions,
    prelude::*,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    assert!(dotenvy::dotenv().is_ok());
    env_logger::init();

    let token = std::env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = wallbot::Handler::new(
        &wallbot::HandlerOptions {
            min_perm: {
                let bits = std::env::var("DISCORD_PERM")?.parse()?;
                Permissions::from_bits(bits)
                    .expect("DISCORD_PERM is not a valid Permissions integer")
            },
            prefix: std::env::var("PREFIX")?.leak(),
        },
        &wallbot::AppOptions {
            wall_limit_width: std::env::var("WALL_LIMIT_WIDTH")?.parse()?,
            wall_limit_height: std::env::var("WALL_LIMIT_HEIGHT")?.parse()?,
            slots_versions_user: std::env::var("SLOTS_VERSIONS_USER")?.parse()?,
            slots_versions_guild: std::env::var("SLOTS_VERSIONS_GUILD")?.parse()?,
            slots_drafts: std::env::var("SLOTS_DRAFTS")?.parse()?,
            credit_default_gain: std::env::var("CREDIT_DEFAULT_GAIN")?.parse()?,
            credit_default_cap: std::env::var("CREDIT_DEFAULT_CAP")?.parse()?,
            credit_default_initial: std::env::var("CREDIT_DEFAULT_INITIAL")?.parse()?,
        },
    );
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

    Ok(client.start().await?)
}
