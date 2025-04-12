/// Get the user's username.
#[cfg(unix)]
pub fn username() -> String {
  use libc::passwd as c_passwd;
  use std::ffi::CStr;
  use std::mem;

  const BUF_SIZE: usize = 16_384; // size from the man page
  let mut buffer = mem::MaybeUninit::<[libc::c_char; BUF_SIZE]>::uninit();
  let mut passwd = mem::MaybeUninit::<c_passwd>::uninit();
  let mut _passwd = mem::MaybeUninit::<*mut c_passwd>::uninit();

  let passwd = unsafe {
    let ret = libc::getpwuid_r(
      libc::geteuid(),
      passwd.as_mut_ptr(),
      buffer.as_mut_ptr() as *mut libc::c_char,
      BUF_SIZE,
      _passwd.as_mut_ptr(),
    );

    if ret != 0 {
      return "Unknown".to_string().into();
    }

    let _passwd = _passwd.assume_init();

    if _passwd.is_null() {
      return "Unknown".to_string().into();
    }

    passwd.assume_init()
  };

  unsafe { CStr::from_ptr(passwd.pw_name).to_string_lossy().to_string() }
}

#[cfg(windows)]
pub use whoami::username;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_username() {
    let expected = whoami::username();
    assert_eq!(expected, username());
  }
}
