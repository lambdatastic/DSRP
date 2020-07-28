use std::{
    collections::HashMap,
    error::Error,
    io,
};

use serde::Deserialize;

const METAL_SIZES: [i16; 7] = [1000, 800, 600, 400, 200, 100, 50];
const CERAMIC_SIZES: [i16; 7] = [800, 640, 480, 320, 160, 80, 40];

#[derive(Debug,Deserialize)]
struct Road {
    name: String,
    crystal_current: i16,
    crystal_total: i16,
    metal_current: i16,
    metal_total: i16,
    ceramic_current: i16,
    ceramic_total: i16,
}

fn parse_from_stdin() -> Result<Vec<Road>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut roads = Vec::new();

    for line in reader.deserialize() {
        let road: Road = line?;
        roads.push(road);
    }

    Ok(roads)
}

fn main() {
    let roads = parse_from_stdin().expect("Did not parse roads data.");

    let mut road_segments = Vec::new();

    let mut crystal_total = 0;

    let mut metal_manifest: HashMap<i16, i16> = HashMap::new();
    let mut ceramic_manifest: HashMap<i16, i16> = HashMap::new();

    for road in roads {
        road_segments.push(road.name);

        crystal_total += road.crystal_total - road.crystal_current;

        let mut metal_remaining = road.metal_total - road.metal_current;

        for size in &METAL_SIZES {
            while (metal_remaining - size) >= 0 {
                *metal_manifest.entry(*size).or_insert(0) += 1;
                metal_remaining -= size;
            }
        }

        if metal_remaining > 0 {
           *metal_manifest.entry(*METAL_SIZES.last().unwrap()).or_insert(0) += 1;
        }

        let mut ceramic_remaining = road.ceramic_total - road.ceramic_current;

        for size in &CERAMIC_SIZES {
            while (ceramic_remaining - size) >= 0 {
                *ceramic_manifest.entry(*size).or_insert(0) += 1;
                ceramic_remaining -= size;
            }
        }

        if ceramic_remaining > 0 {
           *ceramic_manifest.entry(*CERAMIC_SIZES.last().unwrap()).or_insert(0) += 1;
        }
    }

    println!("!!! Road Construction Report !!!");
    println!("--------------------------------");

    println!("Road Segments:");
    for segment in road_segments {
        println!("{}", segment);
    }
    println!("-----");

    println!("Crystal Total: {}", crystal_total);
    println!("-----");

    println!("Metal Manifest:");
    for size in &METAL_SIZES {
        if metal_manifest.contains_key(size) {
            println!("{} x Metals ({})", metal_manifest.get(size).unwrap(), size);
        }
    }
    println!("-----");

    println!("Ceramic Manifest:");
    for size in &CERAMIC_SIZES {
        if ceramic_manifest.contains_key(size) {
            println!("{} x Ceramics ({})", ceramic_manifest.get(size).unwrap(), size);
        }
    }
}
