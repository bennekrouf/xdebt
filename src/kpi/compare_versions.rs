
use crate::kpi::parse_version::parse_version;

pub fn compare_versions(current: &str, required: &str) -> bool {
    let (cur_major, cur_minor, _) = parse_version(current);
    let (req_major, req_minor, _) = parse_version(required);

    // Handle wildcard 'x' in the required version (e.g., '5.x')
    if required.contains('x') {
        if req_major == 0 {
            return true; // Major version 'x' matches any major version
        }

        if cur_major > req_major {
            return true; // Current major version is higher than required
        } else if cur_major == req_major {
            // If the major version matches, check minor version
            if let Some(req_minor) = req_minor {
                // Check if the required minor version is specified as one
                if req_minor == 1 && current.contains('.') {
                    return true; // Accept any valid version for minor '1.x'
                }
                return true; // Accept any current version if minor is unspecified or wildcard
            }
            return true; // If no minor version in required, only match major version
        }
    }

    // Compare numerical values for versions without wildcards
    if cur_major > req_major {
        return true;
    }

    if cur_major == req_major {
        if let Some(req_minor) = req_minor {
            if let Some(cur_minor) = cur_minor {
                return cur_minor >= req_minor; // Current minor must meet or exceed required
            }
        }
        return true; // If no minor version in required, match major version only
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_versions() {
        // Test case 1: "1.x" and "2.x"
        assert_eq!(compare_versions("1.x", "2.x"), false, "1.x should not be compatible with 2.x");

        // Test case 2: "1.2" and "1.x"
        assert_eq!(compare_versions("1.2", "1.x"), true, "1.2 should be compatible with 1.x");

        // Test case 3: "1.3" and "5.x"
        assert_eq!(compare_versions("1.3", "5.x"), false, "1.3 should not be compatible with 5.x");

        // Test case 4: "1.2" and "5.3.4"
        assert_eq!(compare_versions("1.2", "5.3.4"), false, "1.2 should not be compatible with 5.3.4");

        // Test case 5: "5.3.4" and "4.2"
        assert_eq!(compare_versions("5.3.4", "4.2"), true, "5.3.4 should be compatible with 4.2");
    }
}
