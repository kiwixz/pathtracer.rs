#[derive(serde::Deserialize)]
pub struct Scene {
    pub width: i32,
    pub height: i32,
    pub samples: i32,
    pub min_bounces: i32,
    pub max_bounces: i32,
    pub background_color: [f64; 3],

    pub camera: Camera,

    pub planes: Vec<Plane>,
    pub spheres: Vec<Sphere>,
}

#[derive(serde::Deserialize)]
pub struct Camera {
    pub position: [f64; 3],
    pub rotation: [f64; 3],
    pub scale: [f64; 3],
    pub field_of_view: f64,
}

#[derive(serde::Deserialize)]
pub struct Plane {
    pub position: [f64; 3],
    pub rotation: [f64; 3],
    pub reflection: Reflection,
    pub emission: [f64; 3],
    pub color: [f64; 3],
}

#[derive(serde::Deserialize)]
pub struct Sphere {
    pub position: [f64; 3],
    pub radius: f64,
    pub reflection: Reflection,
    pub emission: [f64; 3],
    pub color: [f64; 3],
}

#[derive(serde::Deserialize)]
pub enum Reflection {
    #[serde(rename = "diffuse")]
    Diffuse,

    #[serde(rename = "refractive")]
    Refractive,

    #[serde(rename = "specular")]
    Specular,
}
