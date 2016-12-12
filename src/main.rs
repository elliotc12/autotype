extern crate autotype;
extern crate rusqlite;

use std::process::Command;
use rusqlite::Connection;

fn main() {
    initialize_database();
    println!("Properly initialized.");
}

fn initialize_database() {

    let sqlite_path = "autotype.db";
    let sqlite_conn = Connection::open(&sqlite_path).expect("Could not open sqlite3 db autotype.db");
    
    match sqlite_conn.query_row(".schema dir_history", &[], |row| { row.get(0) })
    {
        Ok(_) => {
            println!("Already exists, returning.");
            return;
        }
        Err(_) => {
            println!("Doesn't exist!");
        }
    }


    sqlite_conn.execute("CREATE TABLE dir_history (num int, dir varchar(200), cmd varchar(100), PRIMARY KEY (cmd));", &[]).expect("Could not create dir_history table.");
}


// let my_pwd_struct = autotype::PwdNumUses {name: "mydir".to_string(), num: 1};
// let my_command = autotype::Command {word: "mycmd".to_string(), pwd_info: vec![my_pwd_struct]};
// println!{"num uses: {}", my_command.pwd_info[0].num};
