extern crate geo;
extern crate geojson;
extern crate rayon;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[allow(unused)]
#[path = "src/util.rs"]
mod util;
use util::Country;

use std::env;
use std::fs::{read_to_string, write};
use std::path::Path;

const SOURCE_DATA: &str = "data/ne_10m_admin_0_countries.geojson";

use geojson::{FeatureCollection, GeoJson, Feature, Geometry, Value};

fn countries_to_geojson(countries: Vec<Country>) -> String {
    GeoJson::from(FeatureCollection {
        bbox: None,
        foreign_members: None,
        features: countries.into_iter().map(|c| Feature {
            bbox: None,
            geometry: Some(Geometry {
                bbox: None,
                value: Value::from(&c.fast_geometry),
                foreign_members: None
            }),
            id: None,
            properties: None,
            foreign_members: None
        }).collect(),
    }).to_string()
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("countries.json");
    let dest_path2 = Path::new(&out_dir).join("countries.geojson");

    let countries = util::parse_from_source(&read_to_string(SOURCE_DATA).unwrap());

    write(&dest_path, serde_json::to_string(&countries).unwrap()).unwrap();
    write(&dest_path2, countries_to_geojson(countries)).unwrap();
    println!("cargo:rerun-if-changed={}", SOURCE_DATA);
}
