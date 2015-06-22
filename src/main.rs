extern crate rustc_serialize;
extern crate csv;

use std::io::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::Path;
use rustc_serialize::json;

#[derive(RustcDecodable)]
struct Record {
    profile_id: u32,
    garment_id: u32,
    score: f64,
}

fn save_score_map(score_map: HashMap<u32,f64>, profile_id: u32, json_output_dir: &Path) {
    let json_score_map = json::encode(&score_map).ok().expect("failed to jsonify score map");
    
    let filename = format!("{}.json", profile_id);
    let json_path = json_output_dir.join(filename);

    let json_buf = json_score_map.as_bytes();
    let mut f = File::create(json_path.as_path()).unwrap();
    f.write_all(json_buf).unwrap();
    println!("{}", json_score_map);
}

fn process_csv(input_csv: &String, output_dir: &String) {
    let csv_filepath = Path::new(&input_csv);
    let json_output_dir = Path::new(&output_dir);

    let mut reader = csv::Reader::from_file(&csv_filepath).unwrap().has_headers(false);

    let mut current_profile_id: u32 = 0;
    let mut score_map: HashMap<u32, f64> = HashMap::new();

    for record in reader.decode() {
        let record: Record = record.unwrap();
        println!("{}, {}, {}", record.profile_id, record.garment_id, record.score);
        if record.profile_id != current_profile_id {
            if current_profile_id != 0 {
                save_score_map(score_map, current_profile_id, json_output_dir);
            }
            current_profile_id = record.profile_id;
            score_map = HashMap::new();
        }
        score_map.insert(record.garment_id, record.score);
    }
    save_score_map(score_map, current_profile_id, json_output_dir);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    process_csv(&args[1], &args[2]);
}
