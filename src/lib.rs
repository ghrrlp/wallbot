use std::collections::HashMap;

use serenity::{
    async_trait,
    model::{
        channel::{Channel, Message},
        id::UserId,
        gateway::Ready,
        permissions::Permissions,
        user::CurrentUser,
    },
    prelude::*,
};
use tokio::sync::RwLock;

pub struct BotIdentity {
    pub user: CurrentUser,
    pub min_perm: Permissions,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BotInviteLink {
    pub user_id: UserId,
    pub perm_bits: u64,
}

impl BotIdentity {
    pub fn invite_link(&self) -> BotInviteLink {
        BotInviteLink {
            user_id: self.user.id,
            perm_bits: self.min_perm.bits(),
        }
    }
}

impl std::fmt::Display for BotInviteLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "https://discord.com/oauth2/authorize?client_id={}&permissions={}&integration_type=0&scope=bot",
            self.user_id,
            self.perm_bits,
        )
    }
}

pub struct Handler {
    opts: HandlerOptions,
    bot_identity: RwLock<Option<BotIdentity>>,
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
            bot_identity: RwLock::new(None),
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
        self.bot_identity.write().await.replace(new_botself);
    }
}
