extern crate show_message;
use show_message::OkOrShow;

fn main() {
    show_message::show("This\nis\na\nmessage!");

    let err: Result<(), String> = Err("This is an error".into());
    err.ok_or_show(|e| format!("Internal Error: {}", e));
}
