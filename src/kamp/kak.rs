use std::process::{Command, ExitStatus, Stdio};
use std::{ffi::OsStr, io::Write};

pub(crate) fn pipe<S, T>(session: S, cmd: T) -> anyhow::Result<ExitStatus>
where
    T: AsRef<[u8]>,
    S: AsRef<OsStr>,
{
    let mut child = Command::new("kak")
        .arg("-p")
        .arg(session)
        .stdin(Stdio::piped())
        .spawn()?;

    let kak_stdin = match child.stdin.as_mut() {
        Some(stdin) => stdin,
        None => {
            use std::io::{Error, ErrorKind};
            return Err(Error::new(ErrorKind::Other, "cannot capture stdin of kak process").into());
        }
    };

    kak_stdin.write_all(cmd.as_ref())?;

    let status = child.wait()?;

    Ok(status)
}

pub(crate) fn connect<S: AsRef<OsStr>>(session: S, e_cmd: S) -> anyhow::Result<ExitStatus> {
    let status = Command::new("kak")
        .arg("-c")
        .arg(session)
        .arg("-e")
        .arg(e_cmd)
        .status()?;

    Ok(status)
}

pub(crate) struct Sessions(String);

impl Sessions {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.lines().collect::<Vec<_>>().into_iter()
    }
}

pub(crate) fn sessions() -> anyhow::Result<Sessions> {
    let output = Command::new("kak").arg("-l").output()?;

    if !output.status.success() {
        if let Some(code) = output.status.code() {
            anyhow::bail!("kak exited with code: {}", code);
        } else {
            anyhow::bail!("kak terminated by signal");
        }
    }

    let list = String::from_utf8(output.stdout)?;

    Ok(Sessions(list))
}

pub(crate) fn proxy(args: Vec<String>) -> anyhow::Result<()> {
    use std::os::unix::prelude::CommandExt;
    let err = Command::new("kak").args(args).exec();
    Err(err.into())
}
