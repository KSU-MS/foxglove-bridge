mod commands;
mod foxglove_utils;
mod random_utils;
mod sources;

use random_utils::{parse_device_file, LoggerType};

fn main() {
    let args = LoggerType::evaluate_args();

    for logger_type in LoggerType::all().iter() {
        evaluate_loggers(&args, logger_type.arg_name());
    }
}

fn evaluate_loggers(args: &clap::ArgMatches, key: &str) {
    if let Some(values) = args.get_many::<String>(key) {
        for value in values {
            let (device, file) = parse_device_file(value, key);

            std::thread::spawn(move || {
                if file.is_some() {
                    println!("Starting logger on {} -> {}", device, file.unwrap());
                } else {
                    println!("Starting logger on {} -> Global", device);
                }
            });
        }
    }
}
