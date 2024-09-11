
use std::io::ErrorKind;
use std::error::Error;
use std::process::Command;

pub fn run_maven_effective_pom(pom_file: &str, repo: &str) -> Result<String, Box<dyn Error>> {
    let output_file = format!("effective_{}.xml", repo);
    let output_option = format!("-Doutput={}", output_file);

    // Print current working directory
    println!("Current working directory: {}", std::env::current_dir()?.display());

    // Run Maven command
    let output = Command::new("mvn")
        .arg("help:effective-pom")
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
        eprintln!("Maven command failed with status: {:?}", output.status);
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Err(Box::new(std::io::Error::new(ErrorKind::Other, "Maven command failed")))
    }
}

