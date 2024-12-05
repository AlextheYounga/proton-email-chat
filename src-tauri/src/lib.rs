mod models;
mod handlers;
use tauri_plugin_sql::Builder as SqlBuilder;
use tauri_plugin_sql::Migration;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


fn migrations() -> Vec<Migration> {
	// Define migrations to set up the database schema
	use models::email::Email;
	use models::email_metadata::EmailMetadata;
    return vec![Email::migration(), EmailMetadata::migration()];
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	handlers::key_handler::generate_key_file().expect("Failed to generate key file");
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
		.plugin(SqlBuilder::default()
		.add_migrations("sqlite:database.db", migrations()).build())
        .invoke_handler(tauri::generate_handler![
			handlers::greet_handler::greet,
			handlers::key_handler::get_key
			])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
