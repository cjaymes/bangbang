use std::f32::consts::PI;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
pub struct Vertex2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
pub struct Vertex3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex3D {
    pub fn to_string(&self) -> String {
        let mut s: String = self.x.to_string().to_owned();
        s.push_str(", ");
        s.push_str(&self.y.to_string().to_owned());
        s.push_str(", ");
        s.push_str(&self.z.to_string().to_owned());
        s
    }

    pub fn is_inside(&self, s: &Shape3D) -> bool {
        match s {
            Shape3D::Sphere { center, radius } => {
                let x1 = std::primitive::f32::powf(self.x - center.x, 2.0);
                let y1 = std::primitive::f32::powf(self.y - center.y, 2.0);
                let z1 = std::primitive::f32::powf(self.z - center.z, 2.0);
                (x1 + y1 + z1) < (radius * radius)
            },
            _ => panic!("Unimplemented")
        }
    }

    pub fn is_on(&self, s: &Shape3D) -> bool {
        match s {
            Shape3D::Sphere { center, radius } => {
                let x1 = std::primitive::f32::powf(self.x - center.x, 2.0);
                let y1 = std::primitive::f32::powf(self.y - center.y, 2.0);
                let z1 = std::primitive::f32::powf(self.z - center.z, 2.0);
                (x1 + y1 + z1) == (radius * radius)
            },
            _ => panic!("Unimplemented")
        }
    }

    pub fn is_on_or_inside(&self, s: &Shape3D) -> bool {
        match s {
            Shape3D::Sphere { center, radius } => {
                let x1 = std::primitive::f32::powf(self.x - center.x, 2.0);
                let y1 = std::primitive::f32::powf(self.y - center.y, 2.0);
                let z1 = std::primitive::f32::powf(self.z - center.z, 2.0);
                (x1 + y1 + z1) <= (radius * radius)
            },
            _ => panic!("Unimplemented")
        }
    }

    pub fn is_outside(&self, s: &Shape3D) -> bool {
        match s {
            Shape3D::Sphere{ center, radius} => {
                let x1 = std::primitive::f32::powf(self.x - center.x, 2.0);
                let y1 = std::primitive::f32::powf(self.y - center.y, 2.0);
                let z1 = std::primitive::f32::powf(self.z - center.z, 2.0);
                (x1 + y1 + z1) > (radius * radius)
            },
            _ => panic!("Unimplemented")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Shape2D {
    Square {
        top_left: Vertex2D,
        width: f32
    },
    Rectangle {
        top_left: Vertex2D,
        width: f32,
        height: f32
    },
    Circle {
        center: Vertex2D,
        radius: f32
    },
    Polygon2D {
        vertices: Vec<Vertex2D>
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub struct Polygon3D {
    pub vertices: Vec<Vertex3D>,
}

impl Polygon3D {
    pub fn surface_area(self) -> f32 {
        Shape3D::Polygon3D(self).surface_area()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Shape3D {
    Cube {
        center: Vertex3D,
        side: f32
    },
    Cuboid {
        center: Vertex3D,
        width: f32,
        height: f32,
        length: f32,
    },
    Cone {
        center: Vertex3D,
        radius: f32,
        height: f32
    },
    Cylinder {
        center: Vertex3D,
        radius: f32,
        height: f32
    },
    Sphere {
        center: Vertex3D,
        radius: f32
    },
    Polygon3D(Polygon3D),
    Polyhedron {
        faces: Vec<Polygon3D>,
    },
}

impl Shape3D {
    pub fn diameter(&self) -> f32 {
        match self {
            Shape3D::Sphere { center: _, radius } => {
                2.0 * radius
            },
            _ => {
                panic!("Unimplemented")
            }
        }
    }
    pub fn surface_area(&self) -> f32 {
        match self {
            Shape3D::Sphere { center:_, radius } => {
                4.0 * PI * radius.powf(2.0)
            },

            Shape3D::Cylinder { center:_, radius, height } => {
                2.0 * PI * radius * (height + radius)
            },

            Shape3D::Cone { center: _, radius, height } => {
                let l = (height.powf(2.0) + radius.powf(2.0)).sqrt();
                PI * radius * (l + radius)
            },

            Shape3D::Cube { center: _, side } => {
                6.0 * side.powf(2.0)
            },

            Shape3D::Cuboid { center: _, width, height, length } => {
                2.0 * ((length * width) + (width * height) + (length * height))
            },

            Shape3D::Polygon3D(poly) => {
                let mut area: f32 = 0.0;
                let mut j = poly.vertices.len() - 1;
                for i in 0 .. poly.vertices.len() {
                    let vi = poly.vertices[i];
                    let vj = poly.vertices[j];
                    area += (vj.x + vi.x) * (vj.y - vi.y);
                    j = i;
                }
                area / 2.0
            },

            Shape3D::Polyhedron { faces } => {
                let mut area: f32 = 0.0;
                for f in faces {
                    area += f.clone().surface_area();
                }
                area
            },
        }
    }

    pub fn volume(&self) -> f32 {
        match self {
            Shape3D::Cube { center:_, side } => {
                side.powf(3.0)
            },

            Shape3D::Cylinder { center:_, radius, height } => {
                PI * radius.powf(2.0) * height
            },

            Shape3D::Sphere { center:_, radius } => {
                4.0 / 3.0 * PI * radius.powf(3.0)
            },

            Shape3D::Cone { center:_, radius, height } => {
                1.0 / 3.0 * PI * radius.powf(2.0) * height
            },

            Shape3D::Cuboid { center: _, width, height, length } => {
                length * width * height
            },

            _ => panic!("Unimplemented"),
        }
    }
}


#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
}
