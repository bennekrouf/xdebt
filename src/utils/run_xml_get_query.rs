
use std::error::Error;
use tracing::{debug, trace};
use roxmltree::Document;

use crate::utils::run_get_request::run_get_request;
use crate::models::AppConfig;

pub fn run_xml_get_query(
    config: &AppConfig,
    file_url: &str,
) -> Result<Document<'static>, Box<dyn Error>> {
    // Use the reusable run_get_request function to get the raw body
    let body = run_get_request(config, file_url)?;

    // Trace the body received
    debug!("Received body: {}", &body);

    // Convert the body into a 'static lifetime by leaking the String
    let body_static: &'static str = Box::leak(body.into_boxed_str());

    // Parse the body as XML using roxmltree
    match Document::parse(body_static) {
        Ok(xml_doc) => {
            // Trace successful XML parsing
            trace!("Successfully parsed XML");
            Ok(xml_doc)
        }
        Err(e) => {
            trace!("Error parsing XML: {}", e);
            Err(format!("Error parsing XML: {}", e).into())
        }
    }
}

