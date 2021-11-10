#![allow(dead_code)]

// Commands
pub const MOVE_TO: &str = "M";
pub const LINE_TO: &str = "L";
pub const HORIZONTAL_LINE_TO: &str = "H";
pub const VERTICAL_LINE_TO: &str = "V";
pub const CUBIC_CURVE_TO: &str = "C";
pub const QUADRATIC_CURVE_TO: &str = "Q";
pub const ELLIPTICAL_ARC_TO: &str = "A";
pub const CLOSE_PATH: &str = "Z";

#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Line,
    Triangle,
    Polygon,
    BezierCurve,
    Bezigon,
    CubicBezierCurve,
    CubicBezigon,
}

impl Primitive {
    pub fn name(&self) -> &'static str {
        match self {
            // Traditional
            Primitive::Line => "line",
            Primitive::Triangle => "triangle",
            Primitive::Polygon => "polygon",

            // Quadratic Beziers
            Primitive::BezierCurve => "quadratic_bezier_curve",
            Primitive::Bezigon => "bezigon",

            // Cubic Beziers
            Primitive::CubicBezierCurve => "cubic_bezier_curve",
            Primitive::CubicBezigon => "cubic_bezigon",
        }
    }

    pub fn vertices(&self) -> usize {
        match self {
            // Traditional
            Primitive::Line => 2,
            Primitive::Triangle => 3,
            Primitive::Polygon => 4,

            // Quadratic Beziers
            Primitive::BezierCurve => 3,
            Primitive::Bezigon => 5,

            // Cubic Beziers
            Primitive::CubicBezierCurve => 4,
            Primitive::CubicBezigon => 7,
        }
    }

    pub fn path_data_template(&self) -> String {
        let mut data = String::new();
        match self {
            Primitive::Line => {
                data.push_str(MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(LINE_TO);
                data.push_str("{} {} ");
                data.push_str(CLOSE_PATH);
            }
            Primitive::Triangle => {
                data.push_str(MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(LINE_TO);
                data.push_str("{} {} ");
                data.push_str(LINE_TO);
                data.push_str("{} {} ");
                data.push_str(CLOSE_PATH);
            }
            Primitive::Polygon => {
                data.push_str(MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(LINE_TO);
                data.push_str("{} {} ");
                data.push_str(LINE_TO);
                data.push_str("{} {} ");
                data.push_str(LINE_TO);
                data.push_str("{} {} ");
                data.push_str(CLOSE_PATH);
            }
            Primitive::BezierCurve => {
                data.push_str(MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(QUADRATIC_CURVE_TO);
                data.push_str("{} {} {} {} ");
                data.push_str(CLOSE_PATH);
            }
            Primitive::Bezigon => {
                data.push_str(MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(QUADRATIC_CURVE_TO);
                data.push_str("{} {} {} {} ");
                data.push_str(QUADRATIC_CURVE_TO);
                data.push_str("{} {} {} {} ");
                data.push_str(CLOSE_PATH);
            }
            Primitive::CubicBezierCurve => {
                data.push_str(MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(CUBIC_CURVE_TO);
                data.push_str("{} {} {} {} {} {} ");
                data.push_str(CLOSE_PATH);
            }
            Primitive::CubicBezigon => {
                data.push_str(MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(CUBIC_CURVE_TO);
                data.push_str("{} {} {} {} {} {} ");
                data.push_str(CUBIC_CURVE_TO);
                data.push_str("{} {} {} {} {} {} ");
                data.push_str(CLOSE_PATH);
            }
        }
        data
    }

    pub fn unit(&self) -> Vec<(f32, f32)> {
        let mut r = Vec::<(f32, f32)>::new();
        match self {
            Primitive::Line => {
                let xy_1 = (-0.5f32, -0.5f32);
                let xy_2 = (0.5f32, 0.5f32);
                r.push(xy_1);
                r.push(xy_2);
            }
            Primitive::Triangle => {
                let xy_1 = (0f32, -0.5f32);
                let xy_2 = (-0.5f32, 0.5f32);
                let xy_3 = (0.5f32, 0.5f32);
                r.push(xy_1);
                r.push(xy_2);
                r.push(xy_3);
            }
            Primitive::Polygon => {
                let xy_1 = (-0.5f32, -0.5f32);
                let xy_2 = (-0.5f32, 0.5f32);
                let xy_3 = (0.5f32, 0.5f32);
                let xy_4 = (0.5f32, -0.5f32);
                r.push(xy_1);
                r.push(xy_2);
                r.push(xy_3);
                r.push(xy_4);
            }
            Primitive::BezierCurve => {
                let xy_1 = (-0.5f32, -0.5f32);
                let xy_2 = (0.5f32, -0.5f32);
                let xy_3 = (0.5f32, 0.5f32);
                r.push(xy_1);
                r.push(xy_2);
                r.push(xy_3);
            }
            Primitive::Bezigon => {
                // Curve for BR quadrant
                let xy_1 = (-0.5f32, -0.5f32);
                let xy_2 = (0.5f32, -0.5f32);
                let xy_3 = (0.5f32, 0.5f32);
                // Origin
                r.push(xy_1);
                // Expand
                r.push(xy_2);
                r.push(xy_3);
                // Reflect
                r.push((xy_2.0 * -1f32, xy_2.1 * -1f32));
                r.push((xy_3.0 * -1f32, xy_3.1 * -1f32));
            }
            Primitive::CubicBezierCurve => {
                let xy_1 = (-0.5f32, -0.5f32);
                let xy_2 = (0.5f32, -0.5f32);
                let xy_3 = (-0.5f32, 0.5f32);
                let xy_4 = (0.5f32, 0.5f32);
                r.push(xy_1);
                r.push(xy_2);
                r.push(xy_3);
                r.push(xy_4);
            }
            Primitive::CubicBezigon => {
                // Curve for BR quadrant
                let xy_1 = (-0.5f32, -0.5f32);
                let xy_2 = (0.5f32, -0.5f32);
                let xy_3 = (-0.5f32, 0.5f32);
                let xy_4 = (0.5f32, 0.5f32);
                // Origin
                r.push(xy_1);
                // Expand
                r.push(xy_2);
                r.push(xy_3);
                r.push(xy_4);
                // Reflect
                r.push((xy_3.0 * -1f32, xy_3.1 * -1f32));
                r.push((xy_2.0 * -1f32, xy_2.1 * -1f32));
                r.push((xy_4.0 * -1f32, xy_4.1 * -1f32));
            }
        };
        r
    }
}

pub fn all() -> Vec<Primitive> {
    let mut primitives: Vec<Primitive> = Vec::new();
    primitives.push(Primitive::Line);
    primitives.push(Primitive::Triangle);
    primitives.push(Primitive::Polygon);
    primitives.push(Primitive::BezierCurve);
    primitives.push(Primitive::Bezigon);
    primitives.push(Primitive::CubicBezierCurve);
    primitives.push(Primitive::CubicBezigon);
    primitives
}

pub fn default() -> Vec<Primitive> {
    let mut primitives: Vec<Primitive> = Vec::new();
    primitives.push(Primitive::Triangle);
    primitives.push(Primitive::BezierCurve);
    primitives.push(Primitive::CubicBezierCurve);
    primitives
}
