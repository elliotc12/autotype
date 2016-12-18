extern crate rusqlite;

use self::rusqlite::Connection;
use std::collections::HashMap;

use super::types as types;

pub fn initialize_database() -> rusqlite::Connection {
    let sqlite_path = "autotype.db";
    let sqlite_conn = Connection::open(&sqlite_path).expect("Could not open sqlite3 db autotype.db");

    match sqlite_conn.execute("CREATE TABLE dir_history (num int, dir varchar(200), cmd varchar(100), PRIMARY KEY (cmd));", &[]) {
        Ok(rows) => {
            println!("Created table, must be a first-time user. c_int return: {}", rows);
        }
        Err(err) => {
            println!("Table exists, returning! Error: {}", err);
            return sqlite_conn;
        }
    }

    let cmd_history_create_sql: &str = "CREATE TABLE cmd_history (num int, cmd varchar(100), prev_cmd varchar(100), num_back int, PRIMARY KEY (cmd));";
    sqlite_conn.execute(cmd_history_create_sql, &[]).expect("Failed to make table cmd_history");

    sqlite_conn
}

pub fn get_new_probability_struct<'a>(dir: String) -> (Vec<String>, HashMap<&'a str, &'a types::CmdInfo<'a>>) {
    let all_cmds: Vec<String> = vec![];
    let cmd_map: HashMap<&str, & types::CmdInfo> = HashMap::new();
    (all_cmds, cmd_map)
}
