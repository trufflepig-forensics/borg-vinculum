use rorm::{Model, Patch};
use uuid::Uuid;

/// Used to login etc.
#[derive(Model)]
pub struct Account {
    /// Primary key
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The username of an account
    #[rorm(unique, max_length = 255)]
    pub username: String,

    /// The hashed password of the account
    #[rorm(max_length = 1024)]
    pub password_hash: String,

    /// The point in time when the account logged in recently
    pub last_login_at: Option<chrono::NaiveDateTime>,

    /// The point in time, the account was created
    #[rorm(auto_create_time)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Patch)]
#[rorm(model = "Account")]
pub(crate) struct AccountInsert {
    pub(crate) uuid: Uuid,
    pub(crate) username: String,
    pub(crate) password_hash: String,
}
