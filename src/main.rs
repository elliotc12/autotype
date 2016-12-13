extern crate autotype;

fn main() {
    let _sqlite_conn = autotype::database::initialize_database();
    loop {
        match wait_for_action() {
            UserAction::_Cmd(cmd) => respond_to_cmd(cmd),
            UserAction::_CD(dir) => respond_to_cd(dir),
            UserAction::_PrintProbabilities(cmd_fragment) => print_probabilities(cmd_fragment),
            UserAction::_Autocomplete(cmd_fragment) => autocomplete(cmd_fragment),
            UserAction::_Nothing => println!("nothing doing!"),
        };
    }
}

fn respond_to_cmd(_cmd: String) {

}

fn respond_to_cd(_dir: String) {

}

fn print_probabilities(_cmd_fragment: String) {

}

fn autocomplete(_cmd_fragment: String) {

}

enum UserAction {
    _Cmd (String),
    _CD (String),
    _PrintProbabilities (String),
    _Autocomplete (String),
    _Nothing,
}

fn wait_for_action() -> UserAction {
    std::thread::sleep(std::time::Duration::from_millis(5000));
    UserAction::_Nothing
}


// let my_pwd_struct = autotype::PwdNumUses {name: "mydir".to_string(), num: 1};
// let my_command = autotype::Command {word: "mycmd".to_string(), pwd_info: vec![my_pwd_struct]};
// println!{"num uses: {}", my_command.pwd_info[0].num};
