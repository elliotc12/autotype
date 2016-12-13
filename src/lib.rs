use std::collections::HashMap;

pub mod database;

pub struct PwdNumUses {
       pub name: String,
       pub num: i32
}

pub struct Command {
       pub word: String,
       pub pwd_info: Vec<PwdNumUses>
}

pub struct CmdInfo {
    pub this_dir_frequency: i32,
    pub total_frequency: i32,
    pub PrevCmdMap: HashMap<&str, PrevCmdInfo>,
}

pub struct PrevCmdInfo {
    pub one_back_frequency: i32,
    pub two_back_frequency: i32,
    pub three_back_frequency: i32,
}
