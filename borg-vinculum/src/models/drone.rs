use rorm::{Model, Patch};
use uuid::Uuid;

/// The model representing a borg drone instance
#[derive(Model)]
pub struct Drone {
    /// The primary key of the drone
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The name of the drone
    #[rorm(max_length = 255, unique)]
    pub name: String,

    /// The state of the drone
    #[rorm(default = false)]
    pub active: bool,

    /// The token of the drone
    #[rorm(max_length = 255)]
    pub token: String,

    /// The borg repository the drone is using
    #[rorm(max_length = 255, unique)]
    pub repository: String,

    /// The passphrase for the repository
    #[rorm(max_length = 255)]
    pub passphrase: String,

    /// The point in time the drone was created
    #[rorm(auto_create_time)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Patch)]
#[rorm(model = "Drone")]
pub(crate) struct DroneInsert {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) token: String,
    pub(crate) repository: String,
    pub(crate) passphrase: String,
}
