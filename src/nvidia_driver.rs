use std::path::PathBuf;

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

    pub fn run(self) {
        // TODO Start NVidia NSight Systems here

        // TODO Wait for completion

        // TODO Retreive output
    }
}
