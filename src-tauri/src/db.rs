mod models;
use models::email::Email;
use models::email_metadata::EmailMetadata;
use tauri_plugin_sql::Migration;

pub fn migrations() -> Vec<Migration> {
    // Define migrations to set up the database schema
    return vec![Email::migration(), EmailMetadata::migration()];
}
