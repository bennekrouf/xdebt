

pub fn compare_versions(v1: &str, v2: &str) -> std::cmp::Ordering {
    let parts1 = v1.split('.').collect::<Vec<&str>>();
    let parts2 = v2.split('.').collect::<Vec<&str>>();

    for (p1, p2) in parts1.iter().zip(parts2.iter()) {
        if *p1 == "x" || *p2 == "x" {
            continue;
        }

        let num1 = p1.parse::<u32>().unwrap_or(0);
        let num2 = p2.parse::<u32>().unwrap_or(0);

        match num1.cmp(&num2) {
            std::cmp::Ordering::Equal => continue,
            non_eq => return non_eq,
        }
    }

    parts1.len().cmp(&parts2.len())
}
