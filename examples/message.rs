extern crate show_message;
use show_message::OkOrMessage;

fn main() {
    show_message::show_message("This\nis\na\nmessage!");

    let err: Result<(), String> = Err("This is an error".into());
    err.ok_or_msg();
}
