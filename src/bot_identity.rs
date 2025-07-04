use serenity::model::{
    id::UserId,
    permissions::Permissions,
    user::CurrentUser,
};

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
