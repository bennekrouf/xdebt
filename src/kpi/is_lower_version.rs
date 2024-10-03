
use crate::kpi::utils::compare_versions::compare_versions;

pub fn is_lower_version(v1: &str, v2: &str) -> bool {
    compare_versions(v1, v2) == std::cmp::Ordering::Less
}
