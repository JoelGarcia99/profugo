use std::{
    fs::File, io::{self, Read}, path::Path, process::Output
};

use dump_db::{dump_mysql, dump_postgres};
use types::{DBEngine, ProfugoConfig, ProfugoContainersConfig};

mod dump_db;
mod types;

fn format_datetime(date: chrono::DateTime<chrono::Local>) -> String {
    date.format("%Y-%m-%d %Hh %Mm %Ss").to_string()
}

fn write_log_file(logs: &String, dir: &str) {
    match std::fs::write(
        format!("{}/logs-{}.txt", dir, format_datetime(chrono::Local::now())),
        logs,
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("Error writing logs to file: {}", e);
        }
    }
}

fn dump_schema(db_engine: DBEngine, credential: &ProfugoContainersConfig) -> io::Result<Output> {
    match db_engine {
        DBEngine::Postgres => dump_postgres(credential),
        DBEngine::MySQL => dump_mysql(credential),
    }
}

fn main() {
    // the total number of stages to be made, so far there are only two, the first one 
    // will validaty JSON integrity and the second one will dump the schema of all the 
    // databases defined in the config JSON
    const TOTAL_STAGES: u8 = 2;

    println!("+=====================================================+");
    println!("+                                                     +");
    println!("+                Profugo Database Dump                +");
    println!("+                                                     +");
    println!("+=====================================================+");
    println!("\n");
    println!(
        "[Profugo stage 1/{}] Checking integrity of config.json...",
        TOTAL_STAGES
    );

    // this will store logs for all the processes with failures to be writtenn to a file 
    // later
    let mut logs = String::new();

    let config_path = Path::new("config.json");
    let mut config_file = File::open(config_path).expect("Unable to open config file, please make sure it exists");
    let mut config_json = String::new();
    config_file.read_to_string(&mut config_json).expect("Unable to read JSON config file");

    let config: ProfugoConfig = match serde_json::from_str(&config_json) {
        Ok(config) => config,
        Err(e) => {
            logs.push_str(&format!(
                "[{} | Profugo] Error parsing config.json: {}",
                chrono::Local::now(),
                e
            ));
            write_log_file(&logs, "./logs");
            panic!(
                "[{} | Profugo] Error parsing config.json: {}",
                chrono::Local::now(),
                e
            );
        }
    };
    println!(
        "[{} | Profugo] Integrity check passed!\n",
        chrono::Local::now()
    );

    println!(
        "[{} | Profugo stage 2/{}] Dumping databases...",
        chrono::Local::now(),
        TOTAL_STAGES
    );

    // defining counters for user feedback
    let mut db_counter = 0i8; // it'll store the index of the DB to be dumped
    let total_dbs = config.credentials.len();

    for credential in config.credentials {

        let db_engine = match DBEngine::from_str(&credential.engine) {
            Some(engine) => engine,
            None => {
                logs.push_str(&format!(
                    "[{} | Profugo] Unknown database engine: {}",
                    chrono::Local::now(),
                    credential.engine
                ));
                continue;
            }
        };

        db_counter += 1;

        println!(
            "[{} | Profugo stage 2/{} - DB {}/{}] Dumping database {} from container {}",
            chrono::Local::now(),
            TOTAL_STAGES,
            db_counter,
            total_dbs,
            credential.db_name,
            credential.container_name
        );

        let output = match dump_schema(db_engine, &credential) {
            Ok(output) => output,
            Err(e) => {
                let log_content = format!(
                    "[{} | Profugo] Error running pg_dump: {}",
                    chrono::Local::now(),
                    e
                );

                logs.push_str(&log_content);
                println!("{}", &log_content);
                continue;
            }
        };

        println!(
            "[{} | Profugo stage 2/{} - DB {}/{}] Database {} dumped successfully!",
            chrono::Local::now(),
            TOTAL_STAGES,
            db_counter,
            total_dbs,
            credential.db_name
        );

        let output_file_name = format!(
            "dump__{}__{}__{}__{}.sql",
            credential.engine,
            format_datetime(chrono::Local::now()),
            credential.container_name,
            credential.db_name
        );

        // write the output to a file
        println!(
            "[{} | Profugo stage 2/{} - DB {}/{}] Writing output to {}/{}",
            chrono::Local::now(),
            TOTAL_STAGES,
            db_counter,
            total_dbs,
            &config.output_dir,
            &output_file_name
        );

        let parsed_output = String::from_utf8_lossy(&output.stdout);

        match std::fs::write(
            format!("{}/{}", &config.output_dir, &output_file_name),
            &*parsed_output,
        ) {
            Ok(_) => {}
            Err(e) => {
                let log_content = format!(
                    "[{} | Profugo, {} - {}] Error writing output to {}/{}: {}",
                    chrono::Local::now(),
                    credential.container_name,
                    credential.db_name,
                    &config.output_dir,
                    &output_file_name,
                    e
                );

                logs.push_str(&log_content);
                println!("{}", &log_content);
                continue;
            }
        };

        println!(
            "[Profugo stage 2/{} - DB {}/{}] Done!",
            TOTAL_STAGES, db_counter, total_dbs
        );
    }

    if !logs.is_empty() {
        write_log_file(&logs, &config.log_dir);
    }
    println!("All done!");
}
