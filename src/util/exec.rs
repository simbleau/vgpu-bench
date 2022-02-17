use crate::Result;
use anyhow::bail;
use log::{error, trace};
use std::{
    ffi::{OsStr, OsString},
    process::Output,
};

pub fn call_program<I, S>(program_path: S, args: I) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    I: Clone,
    S: AsRef<OsStr>,
{
    let program_path = OsString::from(program_path.as_ref());

    trace!(
        "executing process '{} {}'",
        program_path.to_string_lossy(),
        args.clone()
            .into_iter()
            .map(|arg| arg.as_ref().to_string_lossy().to_string())
            .intersperse(" ".to_string())
            .collect::<String>()
    );

    // Run program
    let output = std::process::Command::new(&program_path)
        .args(args)
        .output()
        .expect(
            format!(
                "'{}' was unable to execute, is it in your PATH?",
                program_path.to_string_lossy()
            )
            .as_str(),
        );

    // Check status code
    let output = match output.status.success() {
        true => output,
        false => {
            error!(
                "'{}' exited with failure ({}, err: '{}')",
                program_path.to_string_lossy(),
                output.status.to_string(),
                &String::from_utf8_lossy(&output.stderr)
            );
            bail!(
                "'{}' exited with failure ({}, err: '{}')",
                program_path.to_string_lossy(),
                output.status.to_string(),
                &String::from_utf8_lossy(&output.stderr)
            );
        }
    };

    trace!(
        "completed python3 program '{}' successfully",
        program_path.to_string_lossy()
    );
    Ok(output)
}
