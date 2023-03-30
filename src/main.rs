use chrono::DateTime;
use csv::Writer;
use lazy_static::lazy_static;
use mail_parser::Message;
use std::{env, fs};
use std::fs::{DirEntry, File};
use indicatif::ProgressBar;

lazy_static! {
    static ref verbose: bool = false;
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let input_folder = current_dir.join("klarna_mails/input");
    let output_folder = current_dir.join("klarna_mails/output");

    if !current_dir.join("/klarna_mails").exists() {
        fs::create_dir_all(&input_folder).unwrap();
        fs::create_dir_all(&output_folder).unwrap();
        println!("Looks like you might be running this the first time, I created the folder for you");
        println!("Place your emails in .eml format into the input folder and run the programm again");
        println!("After running the programm again the resulting csv file will be saved in the output folder");
    }

    // Check if the folders exist
    if !input_folder.exists() {
        fs::create_dir_all(&input_folder).unwrap();
    }
    if !output_folder.exists() {
        fs::create_dir_all(&output_folder).unwrap();
    }

    let mut output_writer = Writer::from_path(output_folder.join("transactions.csv")).unwrap();

    let entries = fs::read_dir(&input_folder).unwrap();
    let entry_count = &entries.count();

    let progress_bar = ProgressBar::new(*entry_count as u64);

    for entry in fs::read_dir(&input_folder).unwrap() {
        let entry = entry.unwrap();
        let _path = entry.path();

        parse_content(&entry, &mut output_writer);

        progress_bar.inc(1);
    }
    output_writer.flush().expect("Failed to write output file, is the output folder writable?");
    progress_bar.finish();
}

fn parse_content(input_file: &DirEntry, writer: &mut csv::Writer<File>) {
    lazy_static! {
        static ref REFERENCE_REGEX: regex::Regex = regex::Regex::new(r"\s\d{13}\s").unwrap();
        static ref PRICE_REGEX: regex::Regex = regex::Regex::new(r"\d+[,.]\d+\s[€$]").unwrap();
    }
    let file_path = input_file.path();
    let content = fs::read_to_string(&file_path).unwrap();

    let message = Message::parse(content.as_bytes()).unwrap();
    let message_body = message.body_text(0).unwrap();
    let payee = message
        .subject()
        .unwrap_or_else(|| panic!("Failed to get email subject, please check the input file! File: {}", &file_path.display()))
        .replace("Informationen zu deinem Kauf auf Rechnung bei ", "");
    let date = DateTime::parse_from_rfc3339(&message.date().unwrap().to_rfc3339()).unwrap().format("%Y-%m-%d").to_string();
    let reference_id = REFERENCE_REGEX.find(&message_body).expect("Failed to find the reference ID, has its format changed?").as_str();
    let price = PRICE_REGEX.find(&message_body).expect("Failed to find price, your currency might not yet be supported!").as_str().replace(" €", "");

    if *verbose {
    println!("Payee: {}", payee);
    println!("Date: {}", date);
    println!("Reference ID: {}", reference_id);
    println!("Price: {}", price);
    println!("------------------");
    }

    writer.write_record(&[payee, date, reference_id.to_owned(), price]).expect("Failed to write record, is the output folder still writeable?");
}
