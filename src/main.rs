use std::{env, fmt::format, process::{Stdio, Command}, io::{BufReader, BufRead, Read, self, stdout, Write, StdoutLock}, fs::File, error};

use clap::Parser;
use snafu::Snafu;

fn main() {
    let args = Args::parse();
    match bruteforce(args.starter, args.path) {
        Ok(key) => {
            println!("Found! Key: '{}'", key)
        }
        Err(e) => {
            print!("Error: {}", e)
        }
    }
}

fn bruteforce(starter: String, path: String) -> Result<String, Box<dyn error::Error>>{

    let processed = str::replace(&starter, " ", "");

    if processed.len() > 30 {
        return Err(Box::new(BruteforcerError::InvalidPasskey));
    }

    let keys = 30 - starter.len();

    if keys <= 0 {
        return Err(Box::new(BruteforcerError::InvalidPasskey));
    } else if keys > 10 {
        return Err(Box::new(BruteforcerError::ImpossibleOperation));
    } else if !processed.chars().all(char::is_numeric) {
        return Err(Box::new(BruteforcerError::InvalidPasskey));
    }

    let mut lock = stdout().lock();

    writeln!(lock, "Finding {} digits", keys)?;

    let mut num: u64 = 0;
    for i in 0..keys.pow(keys.try_into().unwrap()) {
        for k in 0..9 {
            let key: String = format!("{}{:0keys$}", processed, num);
            
            if let Some(correct) = try_run_fast(&key, &path, &mut lock)? {
                return Ok(correct);
            }
            num += 1;
        }
    }
    println!("{}", num);

    return Err(Box::new(BruteforcerError::UnreachableError));
}

fn try_run_fast(keya: &String, path: &String, out: &mut StdoutLock) -> Result<Option<String>, Box<dyn error::Error>> {

    writeln!(out, "Trying {}", keya)?;
    let backup_file = File::open(path)?;


    //println!("cipher_text: {:?}", cipher_text);


    Ok(Some(keya.to_string()))
}


fn try_run_slow(key: &String, path: &String, out: &mut StdoutLock) -> Result<Option<String>, Box<dyn error::Error>> {

    writeln!(out, "Trying {}", key)?;

    let args = vec!["-i".to_string(), path.to_string(), "-p".to_string(), key.to_string()];
    
    let mut process = Command::new("signalbackup-tools")
    .args(args)
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute command");

let stdout = process.stdout.take().unwrap();
let reader = BufReader::new(stdout);
for line in reader.lines() {
    let line_str = line.unwrap();
    writeln!(out, "{}", line_str)?;

    if line_str.contains("INCORRECT") {
        process.kill().expect("failed to kill process");
        break;
    }
}

let status = process.wait().expect("failed to wait for process");
if status.success() {
    Ok(Some(key.to_string()))
} else {
    Ok(None)
}
}

#[derive(Debug, Snafu)]
enum BruteforcerError {
    #[snafu(display("The starting passkey provided is invalid"))]
    InvalidPasskey,

    #[snafu(display("The operation chosen will never complete in your lifetime"))]
    ImpossibleOperation,

    #[snafu(display("The key is wrong"))]
    WrongKey,

    #[snafu(display("??? something happened"))]
    UnreachableError,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// The key to start with
   #[arg(short, long, required = true)]
   starter: String,

    /// Number of times to greet
    #[arg(short, long, required = true)]
    path: String,
}