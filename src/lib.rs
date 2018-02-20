use std::process::{Command, Stdio};
use std::ffi::OsStr;

#[cfg(all(not(target_os = "windows"), not(target_os = "linux"), not(target_os = "dragonfly"),
  not(target_os = "freebsd"), not(target_os = "openbsd")))]
compile_error!("The platform you're compiling for is not supported by show message");

pub trait OkOrMessage {
    type OkType;
    fn ok_or_msg(self) -> Self::OkType;
}
impl<T, E: ::std::fmt::Display> OkOrMessage for Result<T,E> {
    type OkType = T;
    fn ok_or_msg(self) -> T {
        match self {
            Ok(t) => t,
            Err(err) => {
                show_message(format!("{}", err));
                println!("{}", err);
                ::std::process::exit(1);
            },
        }
    }
}

#[cfg(target_os = "windows")]
pub fn show_message<M>(message:  M)
where M: AsRef<::std::ffi::OsStr>
{
    use std::io::Write;

    let mut child = Command::new("msg")
        .args(&["*", &message])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute msg command");
}

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
pub fn show_message<M>(message:  M)
where M: AsRef<OsStr>
{
    Command::new("xmessage")
        .args(&[message])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute xmessage command");
}
