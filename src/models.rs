use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub name: String,
}

#[derive(Serialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}
