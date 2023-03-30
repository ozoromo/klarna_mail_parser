use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref CLUTTER_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        // Add more lines with matching locality below here
        ("DE", "Informationen zu deinem Kauf auf Rechnung bei "),
        // Add more lines with matching locality above here
    ]);

    // Add more currency symbols into the [] brackets below in order to match the regex
    pub static ref CURRENCY_REGEX: Regex = Regex::new(r"\d+[,.]\d+\s[€$]").unwrap();
}

// Stuff below here shouldn't have to be changed when adding localities.

pub fn get_subject_clutter(locality: &str) -> String {
    match CLUTTER_MAP.get(locality) {
        Some(clutter) => clutter.to_string(),
        None => panic!("No clutter found for given localtiy: {}", locality)
    }
}