use std::path::PathBuf;

pub fn database_path()-> PathBuf{
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push("data");
    path.push("blog.db");
    path
}
pub fn database_url() -> String {
    format!("sqlite:{}", database_path().to_string_lossy())
}

pub const SERVER_HOST: [u8; 4] = [127, 0, 0, 1];
pub const SERVER_PORT: u16 = 3000;