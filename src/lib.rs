pub mod database;

pub struct PwdNumUses {
       pub name: String,
       pub num: i32
}

pub struct Command {
       pub word: String,
       pub pwd_info: Vec<PwdNumUses>
}
