extern crate show_message;
use show_message::OkOrShow;
use show_message::SomeOrShow;

fn main() {
    show_message::show("This\nis\na\nmessage!");

    let err: Result<(), String> = Err("This is an error".into());
    err.ok_or_show(|e| format!("Internal Error: {}", e));

    let err: Option<()> = None;
    err.some_or_show("Internal Error: It should be some");
}
