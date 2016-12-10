extern crate autotype;

fn main() {
    let my_pwd_struct = autotype::PwdNumUses {name: "mydir".to_string(), num: 1};
    let my_command = autotype::Command {word: "mycmd".to_string(), pwd_info: vec![my_pwd_struct]};
    println!{"num uses: {}", my_command.pwd_info[0].num};
}
