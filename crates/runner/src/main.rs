use aoc;
use getopts::Options;
use std::process;
use url::Url;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("", "profile", "");
    opts.optflag("", "manifest-path", "");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("y", "year", "The year of the advent of code");
    opts.optflag("d", "day", "The day of the advent of code to run");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        process::exit(0);
    }

    let session_id = match aoc::session::get_session_id(&std::env::current_dir().unwrap()) {
        Some(id) => id,
        None => {
            println!("Session ID not found. Please create a .session file in the current directory with your session ID.");
            process::exit(2);
        }
    };

    let year = match matches.opt_get::<i32>("y") {
        Ok(Some(y)) => y,
        _ => {
            args[1].parse().unwrap()
        }
    };

    let day = match matches.opt_get::<i32>("d") {
        Ok(Some(d)) => d,
        _ => {
            args[2].parse().unwrap()
        }
    };


    let context = aoc::Context {
        url: Url::parse("https://adventofcode.com").unwrap(),
        data_dir: std::env::current_dir().unwrap().join("Data"),
        session_id,
    };
    let input = aoc::get_input(&year, &day, &context).await;
    match input {
        Ok(lines) => {
            let start_time = std::time::Instant::now();
            let (part1, part2) = match day {
                1 => (day1::part1(&lines), day1::part2(&lines)),
                _ => {
                    println!("Day {} not implemented", day);
                    process::exit(3);
                }
            };
            let duration = start_time.elapsed();
            println!("Elapsed: {:?}", duration);
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        }
        Err(e) => println!("Error: {}", e),
    }
}
