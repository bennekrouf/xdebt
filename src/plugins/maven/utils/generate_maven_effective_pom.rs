
use std::io::ErrorKind;
use std::error::Error;
use std::process::Command;
use std::fs;
use tracing::{info, trace, error};

pub fn generate_maven_effective_pom(pom_file: &str) -> Result<String, Box<dyn Error>> {
    let effective_pom_file = format!("effective_pom.xml");
    let output_option = format!("-Doutput={}", effective_pom_file);
    let pom_file = format!("{}", &pom_file);

    // Trace the working directory and POM details
    info!("Preparing to run Maven effective-pom for file '{}', outputting to '{}'", &pom_file, &output_option);

    // Check if the POM file exists and is not empty
    let metadata = fs::metadata(&pom_file)
        .map_err(|e| {
            let err_message = format!("Failed to get metadata for POM file '{}': {}", pom_file, e);
            error!("{}", err_message); // Log the error
            err_message
        })?;

    if metadata.len() == 0 {
        let err_message = "POM file is empty".to_string();
        error!("{}", err_message); // Log the error
        return Err(Box::new(std::io::Error::new(ErrorKind::NotFound, err_message)));
    }

    // Trace before running the Maven command
    info!("Running 'mvn help:effective-pom' for POM file '{}'", &pom_file);

    // Run Maven command
    let output = Command::new("mvn")
        .arg("help:effective-pom")
        .arg("-X")  // Verbose output to trace Maven
        .arg("-f")
        .arg(&pom_file)
        .arg(&output_option)
        .output()
        .map_err(|e| {
            let err_message = format!("Failed to execute Maven command: {}", e);
            error!("{}", err_message); // Log the error
            err_message
        })?;

    if output.status.success() {
        info!("Effective POM generated successfully at '{}'", &effective_pom_file);
        trace!("Maven stdout: {}", String::from_utf8_lossy(&output.stdout));
        trace!("Maven stderr: {}", String::from_utf8_lossy(&output.stderr));
        Ok(effective_pom_file.to_string()) // Return the output file name
    } else {
        let status_code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Maven command failed with status code: {}", status_code);
        error!("stderr: {}", stderr);
        Err(Box::new(std::io::Error::new(ErrorKind::Other, "Maven command failed")))
    }
}

