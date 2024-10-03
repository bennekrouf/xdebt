
docker build -t xdebt .
docker run -it -p 9000:9000 xdebt



# xdebt

**xdebt** is a Rust-based application designed to analyze project repositories across platforms like Bitbucket and GitHub. It runs specific analyses for each repository, aggregates the results, and outputs JSON files for further reporting. The application supports custom configurations and multi-project, multi-repository scenarios.

## Features

- Analyze repositories from both Bitbucket and GitHub.
- Custom analysis for Maven, NPM, Docker, .NET, Jenkins, and PHP projects.
- Per-repository and per-project JSON output.
- Consolidated `all_projects.json` output for all projects, with repositories nested under project names.
- Customizable configuration using YAML files.
- Fetch version information from custom YAML input files and external APIs (such as End of Life APIs).
- Version analysis to determine compliance, upgrades needed, and outdated dependencies.

## Requirements

- Rust (v1.x or later)
- Docker (optional, for containerized deployment)

## Installation

### Using Cargo

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/xdebt.git
   cd xdebt


2. Build the project:

cargo build --release

3. Run the project:

cargo run --release


### Using Docker

1. Build the Docker image:

docker build -t xdebt .


2. Run the Docker container:



docker run -v $(pwd)/config:/app/config -t xdebt


Ensure your configuration YAML is correctly mounted in the /app/config folder.


# Configuration


xdebt relies on a YAML configuration file to control its behavior. Here is an example of the configuration:

```
```yaml

platform: bitbucket
base_url: https://dsigit.etat-de-vaud.ch/outils/git
# platform: github
# base_url: https://api.github.com
# user: bennekrouf

force_git_pull: false
force_maven_effective: false
trace_level: info
output_folder: tmp
enable_maven_analysis: false
enable_npm_analysis: true
enable_docker_analysis: false
enable_dotnet_analysis: false
enable_php_analysis: false
enable_jenkins_analysis: false

equivalences:
  hibernate:
    - hibernate
  spring:
    - spring-framework
    - spring-context
    - spring-beans
  spring-boot:
    - spring-boot
  angular:
    - "@angular/core"
    - angular
    - angularjs
  java:
    - java-runtime
    - openjdk
    - jdk
  node:
    - nodejs
    - node
  richface:
    - richfaces
    - richface

roadmap_list:
  - product: "java"
    entries:
      - cycle: "21"
        releaseDate: "2024-01-01"
        eol: null
      - cycle: "8"
        releaseDate: null
        eol: "2023-01-01"

```


- platform: The platform to fetch repositories from (github or bitbucket).
- base_url: The API base URL for the platform.
- force_git_pull: Force a git pull during analysis.
- trace_level: Logging level (info, debug, etc.).
- output_folder: The folder where output JSON files will be written.
- enable_*: Flags to enable or disable specific analyses (e.g., Maven, NPM).
- equivalences: Define project equivalences for version detection.
- roadmap_list: Define version cycles and end-of-life (EOL) data for products.


# Usage
## Analyzing Repositories

To analyze repositories for a project, configure the YAML file for your platform and run the application. For Bitbucket, the project names are extracted using the key field, while for GitHub, the full_name field is used.

##JSON Output

The application generates two types of JSON files:

    Per-Project JSON: Each project generates a file with all analyzed repositories.
    Consolidated all_projects.json: This file contains results for all projects, with repositories nested under each project.

Here is an example of the all_projects.json structure:

```json

{
  "FINANCE": [
    {
      "repository_name": "repo2",
      "debt": {
        "cycle": "5.5.4",
        "product": "esb-client",
        "reason": "No direct match. Version 5.5.4 is outdated as of 2024-10-03. Consider upgrading to 6.x.",
        "status": "Outdated"
      }
    }
  ],
  "AnotherProject": [
    {
      "repository_name": "repoA",
      "debt": {
        "cycle": "5.4.27.",
        "product": "hibernate",
        "reason": "Version 5.4.27. is valid as of 2024-10-03. Valid until 2026-12-31.",
        "status": "Compliant"
      }
    }
  ]
}

```


## External Version Data

In addition to your custom roadmap_list, the application can fetch version information from external APIs (e.g., End of Life APIs) to compare versions and check compliance.

# Development
## Running Tests

Run the test suite using:

```bash
cargo test
```

## Linting

Check for code quality issues with clippy:


```bash
cargo clippy
```


## Formatting

Ensure your code is correctly formatted with:

```bash
cargo fmt
```



# Contributing

Contributions are welcome! To contribute:

    Fork the repository.
    Create a new branch.
    Make your changes.
    Submit a pull request.

# License

This project is licensed under the MIT License. See the LICENSE file for details.

#Contact

For questions or feedback, reach out to the maintainers at mb@mayorana.ch
