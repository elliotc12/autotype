use std::collections::HashMap;

pub struct PwdNumUses {
       pub name: String,
       pub num: i32
}

pub struct Command<'a> {
       pub word: String,
       pub pwd_info: Vec<&'a PwdNumUses>
}

pub struct CmdInfo<'a> {
    pub this_dir_frequency: i32,
    pub total_frequency: i32,
    pub prev_cmd_map: HashMap<&'a str, &'a PrevCmdInfo>,
}

pub struct PrevCmdInfo {
    pub one_back_frequency: i32,
    pub two_back_frequency: i32,
    pub three_back_frequency: i32,
}
