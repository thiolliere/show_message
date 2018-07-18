#[cfg(target_os = "windows")]
extern crate user32;
#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(all(not(target_os = "windows"), not(target_os = "linux"), not(target_os = "dragonfly"),
  not(target_os = "freebsd"), not(target_os = "openbsd"), not(target_os = "macos")))]
pub fn show<M>(message:  M)
where M: AsRef<str>
{
    println!("{}", message.as_ref());
}

pub trait UnwrapOrShow: Sized {
    type OkType;
    type ErrType;
    fn convert(self) -> Result<Self::OkType, Self::ErrType>;
    fn unwrap_or_show(self, msg: impl AsRef<str>) -> Self::OkType {
        match self.convert() {
            Ok(ok) => ok,
            Err(_) => {
                show(msg);
                ::std::process::exit(1);
            }
        }
    }
    fn unwrap_or_else_show(self, func: impl Fn(Self::ErrType) -> String) -> Self::OkType {
        match self.convert() {
            Ok(ok) => ok,
            Err(err) => {
                show(func(err));
                ::std::process::exit(1);
            }
        }
    }
}

impl<T, E> UnwrapOrShow for Result<T, E> {
    type OkType = T;
    type ErrType = E;
    fn convert(self) -> Result<Self::OkType, Self::ErrType> {
        self
    }
}

impl<T> UnwrapOrShow for Option<T> {
    type OkType = T;
    type ErrType = ();
    fn convert(self) -> Result<Self::OkType, Self::ErrType> {
        self.ok_or(())
    }
}

#[cfg(target_os = "windows")]
pub fn show<M>(message:  M)
where M: AsRef<str>
{
    use std::ffi::CString;
    use std::ptr::null_mut;
    use user32::MessageBoxA;
    use winapi::winuser::{MB_OK, MB_SYSTEMMODAL};

    println!("{}", message.as_ref());

    let lp_text = CString::new(message.as_ref()).unwrap();
    let lp_caption = CString::new("Message").unwrap();

    let window_type = MB_OK | MB_SYSTEMMODAL;

    unsafe {
        MessageBoxA(
            null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            window_type
        );
    }
}

#[cfg(target_os = "macos")]
pub fn show<M>(message:  M)
where M: AsRef<str>
{
    use std::process::{Command, Stdio};

    println!("{}", message.as_ref());

    let script = format!("display dialog \"{}\" buttons {{\"OK\"}}", message.as_ref());

    Command::new("osascript")
        .args(&["-e", &script])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute osascript command");
}

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
pub fn show<M>(message:  M)
where M: AsRef<str>
{
    use std::process::{Command, Stdio};

    println!("{}", message.as_ref());

    Command::new("xmessage")
        .args(&[message.as_ref()])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute xmessage command");
}
