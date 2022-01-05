extern crate systemstat;

use chrono::Utc;
use clap::{App, Arg};
use erased_serde::Serialize;
use std::{
    io::Write,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Barrier,
    },
    thread,
    time::Duration,
};
use systemstat::{Platform, System};

#[derive(Debug, serde::Serialize)]
struct Log {
    date: String,
    utilization: f64,
}

pub fn main() {
    let matches = App::new("Memory Usage Monitor")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .about(
            "Runs an input program and queries memory usage with a sample rate",
        )
        .arg(
            Arg::with_name("sample rate")
                .short("s")
                .help("Select a sample rate per second (e.g. 1000)")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Select an output directory (ex: ./output/)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("Select an input program (ex: ./program.sh)")
                .takes_value(true)
                .required(true),
        )
        .arg(Arg::with_name("app_args").multiple(true))
        .get_matches();

    // Sanitize args
    let output_dir = Path::new(matches.value_of("output").unwrap());
    std::fs::create_dir_all(output_dir).expect(
        format!("could not create dir: '{}'", output_dir.display()).as_str(),
    );
    let input_path = Path::new(matches.value_of("input").unwrap());
    assert!(
        input_path.exists() && input_path.is_file(),
        "input path does not exist"
    );
    let sample_rate = matches
        .value_of("sample rate")
        .unwrap_or("1000")
        .parse::<i32>()
        .expect("sample rate must be an i32 integer");

    // Getting ready
    let input_program = input_path.display().to_string();
    let mut args = Vec::<String>::new();
    let app_args = matches.values_of("app_args").unwrap();
    if app_args.len() > 0 {
        for app_string in app_args {
            args.push(app_string.to_string());
        }
    }

    // Used for syncronizing the start of program and
    let barrier = Arc::new(Barrier::new(2));
    // Used to signal program completion
    let complete = Arc::new(AtomicBool::new(false));

    let prog_barrier = barrier.clone();
    let prog_complete = complete.clone();
    let prog_handle = thread::Builder::new()
        .name("program execution thread".to_string())
        .spawn(move || {
            // Wait for start barrier from both threads
            prog_barrier.wait();

            // Call program here
            vgpu_bench::util::call_program(input_program, args).unwrap();

            // Signal thread is complete
            prog_complete.store(true, Ordering::Relaxed);
        })
        .unwrap();

    let log_barrier = barrier.clone();
    let log_complete = complete.clone();
    let log_handle = thread::Builder::new()
        .name("logging thread".to_string())
        .spawn(move || {
            // Getting ready
            let delta_t = 1_f64 / sample_rate as f64;
            let sys = System::new();
            let mut logs = Vec::<Log>::new();

            // Wait for start barrier from both threads
            log_barrier.wait();

            // Start logging
            while log_complete.load(Ordering::Relaxed) == false {
                match sys.memory() {
                    Ok(mem) => {
                        // Log here
                        logs.push(Log {
                            date: Utc::now()
                                .format("%d-%m-%Y_%H-%M-%S.%f")
                                .to_string(),

                            utilization: (mem.free.as_u64() as f64
                                / mem.total.as_u64() as f64),
                        });
                    }
                    Err(_) => panic!("Could not query system memory"),
                }
                thread::sleep(Duration::from_secs_f64(delta_t));
            }

            logs
        })
        .unwrap();

    print!("Running memory monitor...");
    std::io::stdout().flush().unwrap();
    prog_handle.join().unwrap();
    println!("Done");

    print!("Collecting output...");
    std::io::stdout().flush().unwrap();
    let logs = log_handle.join().unwrap();
    let output_path = output_dir.join("mem_usage");
    let rows: Vec<Box<dyn Serialize>> = logs
        .into_iter()
        .map(|x| -> Box<dyn Serialize> { Box::new(x) })
        .collect();
    vgpu_bench::util::write_csv(&output_path, &rows)
        .expect("Could not write output file");
    println!("Done");

    println!("Execution finished. Exiting...");
}
