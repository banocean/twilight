use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker, UserMarker},
    Id,
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

/// Add a role to a member in a guild.
///
/// # Examples
///
/// In guild `1`, add role `2` to user `3`, for the reason `"test"`:
///
/// ```no_run
/// use twilight_http::{request::AuditLogReason, Client};
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = Id::new(1);
/// let role_id = Id::new(2);
/// let user_id = Id::new(3);
///
/// client.add_guild_member_role(guild_id, user_id, role_id)
///     .reason("test")?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct AddRoleToMember<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    role_id: Id<RoleMarker>,
    user_id: Id<UserMarker>,
    reason: Option<&'a str>,
}

impl<'a> AddRoleToMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        role_id: Id<RoleMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            role_id,
            user_id,
            reason: None,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for AddRoleToMember<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl TryIntoRequest for AddRoleToMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::AddMemberRole {
            guild_id: self.guild_id.get(),
            role_id: self.role_id.get(),
            user_id: self.user_id.get(),
        });

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
