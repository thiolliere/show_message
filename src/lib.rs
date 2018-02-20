use std::process::{Command, Stdio};
use std::ffi::OsStr;

#[cfg(all(not(target_os = "windows"), not(target_os = "linux"), not(target_os = "dragonfly"),
  not(target_os = "freebsd"), not(target_os = "openbsd")))]
compile_error!("The platform you're compiling for is not supported by show message");

pub trait OkOrShow {
    type OkType;
    type ErrType;
    fn ok_or_show<F>(self, func: F) -> Self::OkType
    where F: Fn(Self::ErrType) -> String;
}

impl<T, E> OkOrShow for Result<T, E> {
    type OkType = T;
    type ErrType = E;
    fn ok_or_show<F>(self, func: F) -> Self::OkType
    where F: Fn(Self::ErrType) -> String
    {
        match self {
            Ok(t) => t,
            Err(err) => {
                show(func(err));
                ::std::process::exit(1);
            },
        }
    }
}

#[cfg(target_os = "windows")]
pub fn show<M>(message:  M)
where M: AsRef<::std::ffi::OsStr>
{
    use std::io::Write;

    let mut child = Command::new("msg")
        .args(&[OsStr::new("*"), message.as_ref()])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute msg command");
}

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
pub fn show<M>(message:  M)
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
