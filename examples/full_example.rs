extern crate show_message;
use show_message::UnwrapOrShow;

fn main() {
    show_message::show("This\nis\na\nmessage!");

    let err: Result<(), String> = Err("This is an error".into());
    err.unwrap_or_else_show(|e| format!("Internal Error: {}", e));

    let err: Option<()> = None;
    err.unwrap_or_show("Internal Error: It should be some");
}
