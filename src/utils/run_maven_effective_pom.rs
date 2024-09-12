
use std::io::ErrorKind;
use std::error::Error;
use std::process::Command;
use std::fs;

pub fn run_maven_effective_pom(pom_file: &str, repo: &str) -> Result<String, Box<dyn Error>> {
    let output_file = format!("{}/effective_pom.xml", &repo);
    let output_option = format!("-Doutput={}", output_file);
    let pom_file = format!("{}", &pom_file);
    // Print current working directory
    // println!("Current working directory: {}", std::env::current_dir()?.display());
    println!("Running maven effective from {} to {}", &pom_file, &output_option);

    // Check if the POM file exists and is not empty
    let metadata = fs::metadata(&pom_file)
        .map_err(|e| format!("Failed to get metadata for POM file '{}': {}", pom_file, e))?;

    if metadata.len() == 0 {
        return Err(Box::new(std::io::Error::new(ErrorKind::NotFound, "POM file is empty")));
    }

    // Run Maven command
    let output = Command::new("mvn")
        .arg("help:effective-pom")
        .arg("-X")
        .arg("-f")
        .arg(pom_file)
        .arg(output_option)
        .output()
        .map_err(|e| format!("Failed to execute Maven command: {}", e))?;

    if output.status.success() {
        println!("Effective POM generated successfully as '{}'.", output_file);
        println!("Maven stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("Maven stderr: {}", String::from_utf8_lossy(&output.stderr));
        Ok(output_file) // Return the name of the output file
    } else {
        let status_code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Maven command failed with status code: {}", status_code);
        eprintln!("stderr: {}", stderr);
        Err(Box::new(std::io::Error::new(ErrorKind::Other, "Maven command failed")))
    }
}

