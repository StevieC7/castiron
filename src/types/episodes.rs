pub struct Episode {
    pub guid: String,
    pub title: String,
    pub date: String,
    pub played: bool,
    pub played_seconds: i32,
    pub file_path: Option<String>,
    pub url: String,
    pub feed_id: i32,
}
