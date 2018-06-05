use std::sync::Mutex;

use geo::map_coords::MapCoordsInplace;
use geo::prelude::*;
use geo::simplifyvw::SimplifyVWPreserve;
use geo::{Bbox, Closest, MultiPolygon, Point, Polygon};

use geojson::{conversion::TryInto, GeoJson, Value};
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub country_code: String,
    pub geometry: MultiPolygon<f64>,
    pub fast_geometry: MultiPolygon<f64>,
    pub bbox: Bbox<f64>,
}

pub fn coord_count(poly: &MultiPolygon<f64>) -> u64 {
    let coord_count = Mutex::new(0u64);

    poly.map_coords(&|c| {
        *coord_count.lock().unwrap() += 1;
        *c
    });

    coord_count.into_inner().unwrap()
}

pub fn multipolygon_contain(outer: &MultiPolygon<f64>, inner: &mut MultiPolygon<f64>) -> bool {
    let contained = Mutex::new(true);

    inner.map_coords_inplace(&|c| {
        let point = Point::from(*c);
        if !outer.contains(&point) {
            match outer.closest_point(&point) {
                Closest::SinglePoint(p) | Closest::Intersection(p) => (p.x(), p.y()),
                Closest::Indeterminate => {
                    *contained.lock().unwrap() = false;
                    *c
                }
            }
        } else {
            *c
        }
    });

    contained.into_inner().unwrap()
}

pub fn parse_from_source(source: &str) -> Vec<Country> {
    let geojson = source.parse::<GeoJson>().unwrap();
    match geojson {
        GeoJson::FeatureCollection(fc) => fc.features
            .into_par_iter()
            .map(|feature| {
                let properties = feature.properties.expect("country has no properties!");
                let geometry_value = feature.geometry.expect("country has no geometry!").value;

                let name = properties
                    .get("NAME_LONG")
                    .expect("country has no name!")
                    .as_str()
                    .expect("country name is not a string!")
                    .to_string();

                let country_code = properties
                    .get("ISO_A2")
                    .expect("country has no country code!")
                    .as_str()
                    .expect("country code is not a string!")
                    .to_string();

                let geometry: MultiPolygon<f64> = match geometry_value {
                    Value::Polygon(_) => {
                        // the match statement guarantees this will not fail:
                        let poly: Polygon<f64> = geometry_value.try_into().expect("impossible");
                        poly.into()
                    }
                    // the match statement guarantees this will not fail:
                    Value::MultiPolygon(_) => geometry_value.try_into().expect("impossible"),
                    val => panic!("invalid geometry: {:#?}", val),
                };

                let bbox = geometry
                    .bbox()
                    .expect("country's geometry did not have a bounding box?");

                let mut fast_geometry = geometry.simplifyvw_preserve(&1.0);

                assert!(multipolygon_contain(&geometry, &mut fast_geometry));

                Country {
                    name,
                    country_code,
                    geometry,
                    fast_geometry,
                    bbox,
                }
            })
            .collect(),
        _ => panic!("invalid JSON for geocoder"),
    }
}
