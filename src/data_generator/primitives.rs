#![allow(dead_code)]

use crate::commands;

#[derive(Debug)]
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
    pub fn vertices(&self) -> usize {
        match self {
            // Traditional
            Primitive::Line => 2,
            Primitive::Triangle => 3,
            Primitive::Polygon => 4,

            // Quadratic Beziers
            Primitive::BezierCurve => 3,
            Primitive::Bezigon => 9,

            // Cubic Beziers
            Primitive::CubicBezierCurve => 4,
            Primitive::CubicBezigon => 13,
        }
    }

    pub fn path_data_template(&self) -> String {
        let mut data = String::new();
        match self {
            Primitive::Line => {
                data.push_str(commands::MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::LINE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::CLOSE_PATH);
            }
            Primitive::Triangle => {
                data.push_str(commands::MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::LINE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::LINE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::CLOSE_PATH);
            }
            Primitive::Polygon => {
                data.push_str(commands::MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::LINE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::LINE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::LINE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::CLOSE_PATH);
            }
            Primitive::BezierCurve => {
                data.push_str(commands::MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::QUADRATIC_BEZIER_CURVE_TO);
                data.push_str("{} {} {} {} ");
                data.push_str(commands::CLOSE_PATH);
            }
            Primitive::Bezigon => {
                data.push_str(commands::MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::QUADRATIC_BEZIER_CURVE_TO);
                data.push_str("{} {} {} {} ");
                data.push_str(commands::QUADRATIC_BEZIER_CURVE_TO);
                data.push_str("{} {} {} {} ");
                data.push_str(commands::QUADRATIC_BEZIER_CURVE_TO);
                data.push_str("{} {} {} {} ");
                data.push_str(commands::QUADRATIC_BEZIER_CURVE_TO);
                data.push_str("{} {} {} {} ");
                data.push_str(commands::CLOSE_PATH);
            }
            Primitive::CubicBezierCurve => {
                data.push_str(commands::MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::CURVE_TO);
                data.push_str("{} {} {} {} {} {} ");
                data.push_str(commands::CLOSE_PATH);
            }
            Primitive::CubicBezigon => {
                data.push_str(commands::MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::CURVE_TO);
                data.push_str("{} {} {} {} {} {} ");
                data.push_str(commands::CURVE_TO);
                data.push_str("{} {} {} {} {} {} ");
                data.push_str(commands::CURVE_TO);
                data.push_str("{} {} {} {} {} {} ");
                data.push_str(commands::CURVE_TO);
                data.push_str("{} {} {} {} {} {} ");
                data.push_str(commands::CLOSE_PATH);
            }
        }
        data
    }

    pub fn unit_primitive(&self) -> Vec<(f32, f32)> {
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
                let xy_1 = (-0.5f32, 0.5f32);
                let xy_2 = (0f32, -0.5f32);
                let xy_3 = (0.5f32, 0.5f32);
                r.push(xy_1);
                r.push(xy_2);
                r.push(xy_3);
            }
            Primitive::Bezigon => {
                // Curve for BR quadrant
                let xy_1 = (0.5f32, 0f32);
                let xy_2 = (0.5f32, 0.5f32);
                let xy_3 = (0.0f32, 0.5f32);
                // Origin
                r.push(xy_1);
                // Bottom right quadrant
                r.push(xy_2);
                r.push(xy_3);
                // Bottom left quadrant
                r.push((xy_2.0 * -1f32, xy_2.1));
                r.push((xy_1.0 * -1f32, xy_1.1));
                // Top left quadrant
                r.push((xy_2.0 * -1f32, xy_2.1 * -1f32));
                r.push((xy_3.0 * -1f32, xy_3.1 * -1f32));
                // Top right quadrant
                r.push((xy_2.0, xy_2.1 * -1f32));
                r.push((xy_1.0, xy_1.1 * -1f32));
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
                let xy_1 = (0.5f32, 0f32);
                let xy_2 = (0.5f32, 0.33f32);
                let xy_3 = (0.0f32, 0.33f32);
                // Origin
                r.push(xy_1);
                // Bottom right quadrant
                r.push(xy_1);
                r.push(xy_2);
                r.push(xy_3);
                // Bottom left quadrant
                r.push((xy_3.0 * -1f32, xy_3.1));
                r.push((xy_2.0 * -1f32, xy_2.1));
                r.push((xy_1.0 * -1f32, xy_1.1));
                // Top left quadrant
                r.push((xy_1.0 * -1f32, xy_1.1 * -1f32));
                r.push((xy_2.0 * -1f32, xy_2.1 * -1f32));
                r.push((xy_3.0 * -1f32, xy_3.1 * -1f32));
                // Top right quadrant
                r.push((xy_3.0, xy_3.1 * -1f32));
                r.push((xy_2.0, xy_2.1 * -1f32));
                r.push((xy_1.0, xy_1.1 * -1f32));
            }
        };
        r
    }
}
