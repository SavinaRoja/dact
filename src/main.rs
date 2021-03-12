use std::collections::HashSet;
use std::fs::File;
use std::iter;
use std::process;

extern crate clap;
extern crate csv;
use clap::{App, Arg};
use csv::{Reader, Writer};

struct CsvInput {
    path: String,
    primary_key: Option<String>,
}

struct CsvReader {
    reader: Reader<File>,
    path: String,
    primary_key: String,
}

fn get_csv_reader(
    csv_input: CsvInput,
    header_set: &mut HashSet<String>,
    header_vec: &mut Vec<String>,
) -> CsvReader {
    let mut reader = csv::Reader::from_path(&csv_input.path)
        .expect(format!("error reading CSV from path {}", csv_input.path).as_str());

    let headers = reader
        .headers()
        .expect(format!("error reading headers in from {}", csv_input.path).as_str());
    match csv_input.primary_key {
        None => {
            let mut first_key = true;
            let mut primary_key = String::new();
            for header in headers.iter() {
                let header_as_string = header.to_string();
                if first_key {
                    primary_key = header_as_string.clone();
                    first_key = false;
                }
                if !header_set.contains(&header_as_string) {
                    header_set.insert(header_as_string.clone());
                    header_vec.push(header_as_string);
                }
            }
            CsvReader {
                reader: reader,
                path: csv_input.path.clone(),
                primary_key: primary_key,
            }
        }
        Some(value) => {
            let mut primary_key_not_seen = true;
            for header in headers.iter() {
                let header_as_string = header.to_string();
                if primary_key_not_seen {
                    if header_as_string == value.to_string() {
                        primary_key_not_seen = false;
                    }
                }
                if !header_set.contains(&header_as_string) {
                    header_set.insert(header_as_string.clone());
                    header_vec.push(header_as_string);
                }
            }
            if primary_key_not_seen {
                println!(
                    "specified primary header \"{}\" not found in {}",
                    value, csv_input.path
                );
                process::exit(1);
            }
            CsvReader {
                reader: reader,
                path: csv_input.path.clone(),
                primary_key: value.to_string(),
            }
        }
    }
}

fn process_inputs(csv_inputs: Vec<CsvInput>) {
    let mut csv_readers: Vec<CsvReader> = Vec::new();
    let mut combined_headers: HashSet<String> = HashSet::new();
    let mut ordered_combined_headers: Vec<String> = Vec::new();

    for csv_input in csv_inputs {
        csv_readers.push(get_csv_reader(
            csv_input,
            &mut combined_headers,
            &mut ordered_combined_headers,
        ))
    }

    dedupe_and_combine(csv_readers, ordered_combined_headers);
}

fn empty_record(size: usize) -> Vec<String> {
    iter::repeat(String::new()).take(size).collect()
}

fn dedupe_and_combine(csv_readers: Vec<CsvReader>, write_headers: Vec<String>) {
    let mut key_set: HashSet<String> = HashSet::new();
    let mut writer = Writer::from_path("deduped_and_combined.csv").expect("Error writing to file:");

    writer
        .write_record(write_headers.iter())
        .expect("error writing to CSV");
    for csv_reader in csv_readers {
        let mut reader = csv_reader.reader;
        let mut header_map: Vec<usize> = Vec::new();
        let mut primary_index: usize = 0;
        for (read_idx, header) in reader
            .headers()
            .expect("must have headers")
            .iter()
            .enumerate()
        {
            let string_header = header.to_string();
            if string_header == csv_reader.primary_key {
                primary_index = read_idx;
            }
            for (idx, write_header) in write_headers.iter().enumerate() {
                if write_header.to_string() == string_header {
                    header_map.push(idx);
                    break;
                }
            }
        }

        for record in reader.records() {
            let read_record = record.expect(
                format!(
                    "error reading record from CSV with path {}",
                    csv_reader.path
                )
                .as_str(),
            );

            let mut out_record = empty_record(write_headers.len());
            let vec_record: Vec<&str> = read_record.iter().collect();
            let primary_value = vec_record[primary_index].to_string();
            if key_set.contains(&primary_value) {
                continue;
            } else {
                key_set.insert(primary_value);
                for (idx, value) in read_record.iter().enumerate() {
                    out_record[header_map[idx]] = value.to_string();
                }
                writer
                    .write_record(out_record)
                    .expect("error writing to CSV");
            }
        }
    }
}

fn main() {
    let matches = App::new("DACT - De-dupe And Combine Tool")
        .version("0.0.1a")
        .author("Pablo Barton <pablo.barton@gmail.com>")
        .about(
            "Solves a common problem. Dedupe and combine CSV files. Can set a primary header on \
each input, otherwise it will default to the first column.",
        )
        .arg(
            Arg::new("input")
                .about("\"filepath[|primary_header]\"")
                .multiple(true)
                .required(true),
        )
        .get_matches();

    let mut csv_inputs: Vec<CsvInput> = Vec::new();

    let inputs = matches
        .values_of("input")
        .expect("needs at least one input");
    for input in inputs {
        match input.rsplit_once("|") {
            None => csv_inputs.push(CsvInput {
                path: input.to_string(),
                primary_key: None,
            }),
            Some((path, primary_key)) => csv_inputs.push(CsvInput {
                path: path.to_string(),
                primary_key: Some(primary_key.to_string()),
            }),
        }
    }
    process_inputs(csv_inputs);
}
