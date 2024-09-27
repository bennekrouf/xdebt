
use crate::kpi::parse_version::parse_version;

pub fn compare_versions(current: &str, required: &str) -> bool {
    let (cur_major, cur_minor, _) = parse_version(current);
    let (req_major, req_minor, _) = parse_version(required);

    // Handle wildcard 'x' in the required version (e.g., '1.x')
    if required.contains('x') {
        if req_major == 0 {
            return true; // Major version 'x' matches any major version
        }

        if cur_major != req_major {
            return cur_major > req_major; // Compare only major versions if not equal
        }

        // If major versions are equal, check if the minor version is a wildcard
        if let Some(req_minor) = req_minor {
            if required.contains("x") {
                return true; // Minor version 'x' matches any minor version
            }
            if let Some(cur_minor) = cur_minor {
                return cur_minor >= req_minor;
            }
        }
        return true; // If no minor version in required, only match major version
    }

    // Compare numerical values for versions
    if cur_major > req_major {
        return true;
    }

    if cur_major == req_major {
        if let Some(req_minor) = req_minor {
            if let Some(cur_minor) = cur_minor {
                return cur_minor >= req_minor;
            }
        }
        return true; // If no minor version in required, match major version only
    }

    false
}

