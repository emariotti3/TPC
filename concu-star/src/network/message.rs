#[derive(Debug, Clone)]
pub struct Message {
	pub id_observatory: usize,
    pub id_photo: usize,
	pub start_time: std::time::Instant,
}
