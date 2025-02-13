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
    let mut document = Document::new().set("viewBox", (-20, -40, 75, 80));

    match *gj {
        GeoJson::FeatureCollection(ref ctn) => {
            for feature in &ctn.features {
                let mut id: String = "undefined".to_string();
                let mut postal: String = "undefined".to_string();
                if let Some(ref props) = feature.properties {
                    id = props.get("sovereignt").unwrap().to_string().replace("\"", "");
                    postal = props.get("postal").unwrap().to_string().replace("\"", "");
                }
                if let Some(ref geom) = feature.geometry {
                    if let Some(path) = match_geometry(geom, &id, &postal) {
                        document = document.add(path);
                    }
                }
            }
        }
        // GeoJson::Feature(ref feature) => {
        //     if let Some(ref geom) = feature.geometry {
        //         if let Some(path) = match_geometry(geom) {
        //             document = document.add(path);
        //         }
        //     }
        // }
        _ => todo!(),
    }
    svg::save("image.svg", &document).unwrap();
}

/// Process GeoJSON geometries
fn match_geometry(geom: &Geometry, id: &str, postal: &str) -> Option<Path> {
    match &geom.value {
        Value::Polygon(polygon) => {
            let points = polygon[0].clone();
            let mut data = Data::new().move_by((points[0][0], -points[0][1]));
            for pt in &points[1..] {
                data = data.line_to((pt[0], -pt[1]));
            }
            let path = Path::new()
                .set("id", id)
                .set("postal", postal)
                .set("fill", "PapayaWhip")
                .set("stroke", "black")
                .set("stroke-width", "0.1")
                .set("d", data.close())
                .set("onmouseover", format!("top.on_mouse_over(this.id, this.postal)"))
                .set("onmouseout", format!("top.on_mouse_out(this.id)"))
                .set("onclick", format!("top.on_mouse_click(this.id, this.postal)"));
        return Some(path)
        },
        // Value::MultiPolygon(_) => println!("Matched a MultiPolygon"),
        _ => println!("Matched some other geometry"),
    }
    None
}
