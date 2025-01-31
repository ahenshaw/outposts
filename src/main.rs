use geojson::{GeoJson, Geometry, Value};
use anyhow::Result;
use std::fs;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

fn main() -> Result<()> {
    let data: String = fs::read_to_string("data/africa.geojson")?;

    let geojson: GeoJson = data.parse::<GeoJson>()?;
    // let feature: Feature = Feature::try_from(geojson)?;

    process_geojson(&geojson);
    Ok(())
}

/// Process top-level GeoJSON Object
fn process_geojson(gj: &GeoJson) {
    let mut document = Document::new().set("viewBox", (-30, -60, 90, 120));

    match *gj {
        GeoJson::FeatureCollection(ref ctn) => {
            for feature in &ctn.features {
                if let Some(ref props) = feature.properties {
                    print!("{} ", props.get("sovereignt").unwrap().to_string());
                }
                if let Some(ref geom) = feature.geometry {
                    if let Some(path) = match_geometry(geom) {
                        document = document.add(path);
                    }
                }
            }
        }
        GeoJson::Feature(ref feature) => {
            if let Some(ref geom) = feature.geometry {
                if let Some(path) = match_geometry(geom) {
                    document = document.add(path);
                }
            }
        }
        GeoJson::Geometry(_) => todo!(),
    }
    svg::save("image.svg", &document).unwrap();
}

/// Process GeoJSON geometries
fn match_geometry(geom: &Geometry) -> Option<Path> {
    match &geom.value {
        Value::Polygon(polygon) => {
            let points = polygon[0].clone();
            let mut data = Data::new().move_by((points[0][0], -points[0][1]));
            for pt in &points[1..] {
                data = data.line_to((pt[0], -pt[1]));
            }
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.1)
                .set("d", data.close());
            return Some(path)
        },
        // Value::MultiPolygon(_) => println!("Matched a MultiPolygon"),
        _ => println!("Matched some other geometry"),
    }
    None
}
