#[derive(serde::Deserialize)]
pub struct Scene {
    pub width: i32,
    pub height: i32,

    pub iterations: i32,
    pub supersampling: i32,
    pub samples: i32,
    pub bounces: i32,

    pub background_color: [f64; 3],
    pub epsilon: f64,

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

#[derive(Clone, serde::Deserialize)]
pub struct Object {
    #[serde(default)]
    pub emission: [f64; 3],

    #[serde(default)]
    pub color: [f64; 3],

    #[serde(default)]
    pub specular: f64,

    #[serde(default)]
    pub refraction: f64,
}

#[derive(serde::Deserialize)]
pub struct Plane {
    pub position: [f64; 3],
    pub rotation: [f64; 3],

    #[serde(flatten)]
    pub object: Object,
}

#[derive(serde::Deserialize)]
pub struct Sphere {
    pub position: [f64; 3],
    pub radius: f64,

    #[serde(flatten)]
    pub object: Object,
}
