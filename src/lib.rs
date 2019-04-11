extern crate geo;
extern crate geojson;
extern crate rayon;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate ruru;

use ruru::{AnyObject, Class, Float, NilClass, Object, RString};

use geo::prelude::*;
pub use geo::Point;

pub mod util;
pub use util::Country;

pub static PROCESSED_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/countries.json"));

lazy_static! {
    pub static ref COUNTRIES: Vec<Country> = {
        let mut countries: Vec<Country> = serde_json::from_str(PROCESSED_JSON).unwrap();
        for country in countries.iter_mut() {
            country.initialize();
        }
        countries
    };
}

class!(Geocoder);

methods!(
    Geocoder,
    _itself,
    fn find_country_rb(x: Float, y: Float) -> AnyObject {
        match (x, y) {
            (Ok(x), Ok(y)) => match find_country(&Point::new(x.to_f64(), y.to_f64())) {
                Some(country_code) => RString::new(country_code).value().into(),
                None => NilClass::new().value().into(),
            },
            _ => NilClass::new().value().into(),
        }
    }
);

#[no_mangle]
pub extern "C" fn init_geocoder() {
    Class::new("Geocoder", None).define(|itself| {
        itself.def_self("find_country", find_country_rb);
    });
}

pub fn find_country(coord: &Point<f64>) -> Option<&str> {
    for country in COUNTRIES.iter() {
        if !country.in_bboxes(coord) {
            continue;
        }

        if !country.geometry.contains(coord) {
            continue;
        }

        return Some(&country.country_code)
    }

    return None
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

    #[test]
    fn find_coord_edgecase_works() {
        let coord = Point::new(28.2030663, 45.905787);
        assert_eq!(find_country(&coord).unwrap(), "MD");
    }
}
