use tauri_plugin_sql::Migration;
use tauri_plugin_sql::MigrationKind;

pub struct Email;

impl Email {
    pub fn migration() -> Migration {
        return Migration {
            version: 1,
            description: "create emails table",
            sql: "CREATE TABLE IF NOT EXISTS emails (
				id INTEGER PRIMARY KEY,
				proton_id TEXT NOT NULL,
				subject TEXT NULL,
				sender_name TEXT NOT NULL,
				sender_address TEXT NOT NULL,
				content TEXT NOT NULL,
				timestamp TIMESTAMP DEFAULT NULL,
				created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
				updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			);",
            kind: MigrationKind::Up,
        };
    }
}
