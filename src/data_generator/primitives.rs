#![allow(dead_code)]

use crate::commands;

#[derive(Debug)]
pub enum Primitive {
    Line,
    Triangle,
    Polygon,
    Circle,
    Ellipsoid,
    QuadraticBezigon,
    CubicBezigon,
}

impl Primitive {
    pub fn vertices(&self) -> usize {
        match self {
            Primitive::Line => 2,
            Primitive::Triangle => 3,
            Primitive::Polygon => 4,
            Primitive::Circle => todo!(),
            Primitive::Ellipsoid => todo!(),
            Primitive::QuadraticBezigon => todo!(),
            Primitive::CubicBezigon => todo!(),
        }
    }

    pub fn path_data_template(&self) -> String {
        let mut data = String::new();
        match self {
            Primitive::Line => {
                // Start line
                data.push_str(commands::MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::LINE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::CLOSE_PATH);
            }
            Primitive::Triangle => {
                // Start line
                data.push_str(commands::MOVE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::LINE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::LINE_TO);
                data.push_str("{} {} ");
                data.push_str(commands::CLOSE_PATH);
            }
            Primitive::Polygon => {
                // Start line
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
            Primitive::Circle => todo!(),
            Primitive::Ellipsoid => todo!(),
            Primitive::QuadraticBezigon => todo!(),
            Primitive::CubicBezigon => todo!(),
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
            Primitive::Circle => todo!(),
            Primitive::Ellipsoid => todo!(),
            Primitive::QuadraticBezigon => todo!(),
            Primitive::CubicBezigon => todo!(),
        };
        r
    }
}
