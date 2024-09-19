use crate::kpi::parse_version::parse_version;

pub fn compare_versions(current: &str, required: &str) -> bool {
    let (cur_major, cur_minor, _) = parse_version(current);
    let (req_major, req_minor, _req_patch) = parse_version(required);

    if cur_major > req_major {
        return true;
    }
    if cur_major == req_major {
        if let Some(req_minor) = req_minor {
            if let Some(cur_minor) = cur_minor {
                return cur_minor >= req_minor;
            }
        }
        return true;  // if no minor version in required, match major version only
    }
    false
}
