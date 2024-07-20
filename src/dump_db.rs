use std::{io, process::{Command, Output}};

use crate::ProfugoContainersConfig;

pub fn dump_postgres(credential: &ProfugoContainersConfig) -> io::Result<Output> {
    Command::new("docker")
        .arg("exec")
        .arg(&credential.container_name)
        .arg("pg_dump")
        .arg("-U")
        .arg(&credential.user)
        .arg(&credential.db_name)
        .output()
}

pub fn dump_mysql(credential: &ProfugoContainersConfig ) -> io::Result<Output> {
    Command::new("docker")
        .arg("exec")
        .arg(&credential.container_name)
        .arg("mysqldump")
        .arg("-u")
        .arg(&credential.user)
        .arg(format!("-p{}", &credential.password))
        .arg(&credential.db_name)
        .output()
}
