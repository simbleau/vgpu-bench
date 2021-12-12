use std::{ffi::OsString, path::PathBuf};

pub struct NvidiaDriver {
    input_program: PathBuf,
    output_dir: PathBuf,
}

impl NvidiaDriver {
    pub fn new<P>(i: P, o: P) -> Self
    where
        P: Into<PathBuf>,
    {
        NvidiaDriver {
            input_program: i.into(),
            output_dir: o.into(),
        }
    }

    pub fn run(&self) {
        let program_path = OsString::from("nsys");
        let args = [
            "profile",
            "-s",
            "cpu",
            "-o",
            &self.output_dir.join("nvidia").to_string_lossy().to_string(),
            &self.input_program.as_os_str().to_string_lossy().to_string(),
            "--",
            &self.output_dir.to_string_lossy().to_string(),
        ];

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
        match output.status.success() {
            true => output,
            false => {
                eprintln!(
                    "'{}' exited with failure ({}, err: '{}')",
                    program_path.to_string_lossy(),
                    output.status.to_string(),
                    &String::from_utf8_lossy(&output.stderr)
                );
                std::process::exit(1);
            }
        };
    }

    pub fn convert(&self, convert_to: &str) {
        let program_path = OsString::from("nsys");
        let mut output_path = self.output_dir.join("nvidia");
        output_path.set_extension(convert_to);

        let args = [
            "export",
            "-o",
            output_path.as_os_str().to_str().unwrap(),
            "--type",
            convert_to,
            "--separate-strings",
            "true",
            &self
                .output_dir
                .join("nvidia.qdrep")
                .to_string_lossy()
                .to_string(),
        ];

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
        match output.status.success() {
            true => output,
            false => {
                eprintln!(
                    "'{}' exited with failure ({}, err: '{}')",
                    program_path.to_string_lossy(),
                    output.status.to_string(),
                    &String::from_utf8_lossy(&output.stderr)
                );
                std::process::exit(1);
            }
        };
    }
}
