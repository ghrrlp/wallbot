use serenity::{
    async_trait,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        permissions::Permissions,
    },
    prelude::*,
};
use tokio::sync::RwLock;

mod bot_identity;
use bot_identity::BotIdentity;

pub struct Handler {
    opts: HandlerOptions,
    botself: RwLock<Option<BotIdentity>>,
}

#[derive(Clone, Debug)]
pub struct HandlerOptions {
    pub min_perm: Permissions,
    pub prefix: &'static str,
}

impl Handler {
    pub fn new(opts: &HandlerOptions) -> Self {
        Self {
            opts: opts.clone(),
            botself: RwLock::new(None),
        }
    }

    async fn log_message_pretty(&self, ctx: &Context, msg: &Message) {
        let author_string = match msg.author.discriminator {
            Some(d) => format!("{}#{}", msg.author.name, d),
            _ => msg.author.name.clone(),
        };
        match msg.channel(&ctx).await {
            Ok(Channel::Guild(chan)) => {
                let guild_name = match chan.guild_id.to_partial_guild(ctx).await {
                    Ok(partial_guild) => partial_guild.name.clone(),
                    Err(e) => {
                        log::error!("{:?}", e);
                        "??".to_owned()
                    }
                };
                log::info!("[{}@{}#{}] {}", author_string, guild_name, chan.name, msg.content);
            }
            Ok(Channel::Private(dm)) => {
                log::info!("[{}##{}] {}", author_string, dm.name(), msg.content);
            }
            Ok(_) => {
                log::info!("[{}@??] {}", author_string, msg.content);
            }
            Err(e) => {
                log::info!("[{}@!!] {}", author_string, msg.content);
                log::error!("{:?}", e);
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        self.log_message_pretty(&ctx, &msg).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("{} ({}) is connected!", ready.user.name, ready.user.id);
        let new_botself = BotIdentity {
            user: ready.user,
            min_perm: self.opts.min_perm,
        };
        log::info!("Invite via {}", new_botself.invite_link());
        self.botself.write().await.replace(new_botself);
    }
}
