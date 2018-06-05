extern crate geo;
extern crate geojson;
extern crate rayon;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use geo::prelude::*;
pub use geo::Point;

pub mod util;
pub use util::Country;

pub static PROCESSED_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/countries.json"));

lazy_static! {
    pub static ref COUNTRIES: Vec<Country> = { serde_json::from_str(PROCESSED_JSON).unwrap() };
}

pub fn find_country(coord: &Point<f64>) -> Option<&str> {
    const MAX_SLOW: usize = 25;
    let mut slow_lookup = [None; MAX_SLOW];
    let mut i = 0;
    let mut result = None;
    let mut force_slow = false;

    let bbox_countries = COUNTRIES
        .iter()
        .filter(|country| country.bbox.contains(coord));

    for country in bbox_countries {
        if country.fast_geometry.contains(coord) {
            if result.is_none() {
                result = Some(country);
            } else {
                force_slow = true;
            }
        }

        // collect up to MAX_SLOW countries for later,
        // slower analysis if we either don't find any
        // result at all, or if we find multiple results
        // in the possibly-overlapping fast geometries.
        if i < MAX_SLOW {
            slow_lookup[i] = Some(country);
            i += 1;
        } else if country.geometry.contains(coord) {
            return Some(&country.country_code);
        }
    }

    if result.is_none() || force_slow {
        for country in slow_lookup.iter().flat_map(|v| v) {
            if country.geometry.contains(coord) {
                return Some(&country.country_code);
            }
        }
    }

    result.map(|r| r.country_code.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_geojson() {
        assert!(COUNTRIES.len() > 0);
    }

    #[test]
    fn find_coord() {
        let coord = Point::new(-122.3164206, 37.5506619);
        assert_eq!(find_country(&coord).unwrap(), "US");
    }
}
