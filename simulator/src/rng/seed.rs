use uuid::Uuid;
lazy_static! {
    pub static ref SEED: Uuid = Uuid::new_v4();
}
