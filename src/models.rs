//! Models used in the Actix Web application.
//!
//! This module defines data structures that represent application entities,
//! such as `User` and `AppInfo`. These structures are used to serialize
//! data into JSON responses for various endpoints.

use serde::Serialize;

/// Represents a user with a name.
///
/// # Description
/// This struct is used in the `/a/{name}` endpoint to dynamically generate
/// a JSON response containing a user-provided name. It derives `Serialize`
/// to enable JSON serialization.
///
/// # Fields
/// - `name` (String): The name of the user.
///
/// # Example
/// ```
/// use crate::models::User;
///
/// let user = User {
///     name: "Dom".to_string(),
/// };
/// println!("{:?}", serde_json::to_string(&user).unwrap());
/// // Output: {"name":"Dom"}
/// ```
#[derive(Serialize)]
pub struct User {
    /// The name of the user.
    pub name: String,
}

/// Represents application metadata.
///
/// # Description
/// This struct is used in the `/status` endpoint to return information about
/// the application's name and version. It derives `Serialize` to enable JSON
/// serialization for HTTP responses.
///
/// # Fields
/// - `name` (String): The name of the application.
/// - `version` (String): The version of the application.
///
/// # Example
/// ```
/// use crate::models::AppInfo;
///
/// let app_info = AppInfo {
///     name: "Hello Actix App".to_string(),
///     version: "1.0.0".to_string(),
/// };
/// println!("{:?}", serde_json::to_string(&app_info).unwrap());
/// // Output: {"name":"Hello Actix App","version":"1.0.0"}
/// ```
#[derive(Serialize)]
pub struct AppInfo {
    /// The name of the application.
    pub name: String,
    /// The version of the application.
    pub version: String,
}
