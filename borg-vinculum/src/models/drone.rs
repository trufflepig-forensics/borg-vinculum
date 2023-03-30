use rorm::fields::{BackRef, ForeignModel};
use rorm::{field, Model, Patch};
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

    /// The stats of this drone
    pub stats: BackRef<field!(DroneStats::F.drone)>,
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

/// The stats of a drone
#[derive(Model)]
pub struct DroneStats {
    /// The primary key of the drone
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The drone that owns this stats
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub drone: ForeignModel<Drone>,

    /// The duration in seconds that the pre hook took to execute
    pub pre_hook_duration: Option<i64>,
    /// The duration in seconds that the post hook took to execute
    pub post_hook_duration: Option<i64>,
    /// The duration in seconds that the archive creation took
    pub create_duration: i64,
    /// The duration in seconds that the complete operation took
    pub complete_duration: i64,
    /// Original file size in bytes
    pub original_size: i64,
    /// Compressed file size in bytes
    pub compressed_size: i64,
    /// Deduplicated file size in bytes
    pub deduplicated_size: i64,
    /// Number of archived files
    pub nfiles: i64,

    /// The point in time, this stats were collected
    #[rorm(auto_create_time)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Patch)]
#[rorm(model = "DroneStats")]
pub(crate) struct DroneStatsInsert {
    pub(crate) uuid: Uuid,
    pub(crate) drone: ForeignModel<Drone>,
    pub(crate) pre_hook_duration: Option<i64>,
    pub(crate) post_hook_duration: Option<i64>,
    pub(crate) create_duration: i64,
    pub(crate) complete_duration: i64,
    pub(crate) original_size: i64,
    pub(crate) compressed_size: i64,
    pub(crate) deduplicated_size: i64,
    pub(crate) nfiles: i64,
}
