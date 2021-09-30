use lyon::math::Point;
use lyon::path::PathEvent;
use lyon::tessellation::geometry_builder::*;
use lyon::tessellation::{self, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator};
use usvg::{NodeExt, Tree, ViewBox};

use std::error::Error;
use std::f64::NAN;

use crate::artifacts::{TessellationData, TessellationProfile};
use crate::renderer::types::{GpuPrimitive, GpuTransform, GpuVertex};
use crate::targets::SVGDocument;
use crate::Tessellator;

pub const FALLBACK_COLOR: usvg::Color = usvg::Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 0,
};

#[derive(Clone)]
pub struct LyonState {
    rtree: Tree,
    #[allow(dead_code)]
    view_box: ViewBox,
    prev_transform: usvg::Transform,
}
pub struct LyonTessellator {
    state: Option<LyonState>,
}

impl LyonTessellator {
    pub fn new() -> LyonTessellator {
        Self { state: None }
    }
}

impl Tessellator for LyonTessellator {
    fn name(&self) -> &'static str {
        "Lyon"
    }

    fn init(&mut self, t: &SVGDocument) {
        let opt = usvg::Options::default();
        let file_data = t.content.as_bytes();

        let rtree = usvg::Tree::from_data(&file_data, &opt.to_ref()).unwrap();
        let view_box = rtree.svg_node().view_box;

        let prev_transform = usvg::Transform {
            a: NAN,
            b: NAN,
            c: NAN,
            d: NAN,
            e: NAN,
            f: NAN,
        };

        let state = LyonState {
            rtree,
            view_box,
            prev_transform,
        };
        self.state = Some(state);
    }

    fn tessellate(&mut self) -> Result<TessellationProfile, Box<dyn Error>> {
        let data = self.get_tessellate_data()?;
        Ok(TessellationProfile {
            vertices: data.mesh.vertices.len() as u32,
            indices: data.mesh.indices.len() as u32,
        })
    }

    fn get_tessellate_data(&mut self) -> Result<Box<TessellationData>, Box<dyn Error>> {
        // Create vertex buffer
        let mut fill_tess = FillTessellator::new();
        let mut stroke_tess = StrokeTessellator::new();
        let mut mesh: VertexBuffers<_, u32> = VertexBuffers::new();

        let mut transforms = Vec::new();
        let mut primitives = Vec::new();

        for node in self.state.as_ref().unwrap().rtree.root().descendants() {
            if let usvg::NodeKind::Path(ref p) = *node.borrow() {
                let t = node.transform();
                if t != self.state.as_ref().unwrap().prev_transform {
                    transforms.push(GpuTransform {
                        data0: [t.a as f32, t.b as f32, t.c as f32, t.d as f32],
                        data1: [t.e as f32, t.f as f32, 0.0, 0.0],
                    });
                }
                self.state.as_mut().unwrap().prev_transform = t;

                let transform_idx = transforms.len() as u32 - 1;

                if let Some(ref fill) = p.fill {
                    // fall back to always use color fill
                    // no gradients (yet?)
                    let color = match fill.paint {
                        usvg::Paint::Color(c) => c,
                        _ => FALLBACK_COLOR,
                    };

                    primitives.push(GpuPrimitive::new(
                        transform_idx,
                        color,
                        fill.opacity.value() as f32,
                    ));

                    fill_tess
                        .tessellate(
                            convert_path(p),
                            &FillOptions::tolerance(0.01),
                            &mut BuffersBuilder::new(
                                &mut mesh,
                                VertexCtor {
                                    prim_id: primitives.len() as u32 - 1,
                                },
                            ),
                        )
                        .expect("Error during tesselation!");
                }

                if let Some(ref stroke) = p.stroke {
                    let (stroke_color, stroke_opts) = convert_stroke(stroke);
                    primitives.push(GpuPrimitive::new(
                        transform_idx,
                        stroke_color,
                        stroke.opacity.value() as f32,
                    ));
                    let _ = stroke_tess.tessellate(
                        convert_path(p),
                        &stroke_opts.with_tolerance(0.01),
                        &mut BuffersBuilder::new(
                            &mut mesh,
                            VertexCtor {
                                prim_id: primitives.len() as u32 - 1,
                            },
                        ),
                    );
                }
            }
        }

        let data = TessellationData {
            mesh,
            transforms,
            primitives,
        };

        // Return result
        Ok(Box::new(data))
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuGlobals {
    pub zoom: [f32; 2],
    pub pan: [f32; 2],
    pub aspect_ratio: f32,
    pub _pad: f32,
}

pub struct VertexCtor {
    pub prim_id: u32,
}

impl FillVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            prim_id: self.prim_id,
        }
    }
}

impl StrokeVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::StrokeVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            prim_id: self.prim_id,
        }
    }
}
/// Some glue between usvg's iterators and lyon's.

fn point(x: &f64, y: &f64) -> Point {
    Point::new((*x) as f32, (*y) as f32)
}

pub struct PathConvIter<'a> {
    iter: std::slice::Iter<'a, usvg::PathSegment>,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
}

impl<'l> Iterator for PathConvIter<'l> {
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }

        let next = self.iter.next();
        match next {
            Some(usvg::PathSegment::MoveTo { x, y }) => {
                if self.needs_end {
                    let last = self.prev;
                    let first = self.first;
                    self.needs_end = false;
                    self.prev = point(x, y);
                    self.deferred = Some(PathEvent::Begin { at: self.prev });
                    self.first = self.prev;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    self.first = point(x, y);
                    self.needs_end = true;
                    Some(PathEvent::Begin { at: self.first })
                }
            }
            Some(usvg::PathSegment::LineTo { x, y }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point(x, y);
                Some(PathEvent::Line {
                    from,
                    to: self.prev,
                })
            }
            Some(usvg::PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point(x, y);
                Some(PathEvent::Cubic {
                    from,
                    ctrl1: point(x1, y1),
                    ctrl2: point(x2, y2),
                    to: self.prev,
                })
            }
            Some(usvg::PathSegment::ClosePath) => {
                self.needs_end = false;
                self.prev = self.first;
                Some(PathEvent::End {
                    last: self.prev,
                    first: self.first,
                    close: true,
                })
            }
            None => {
                if self.needs_end {
                    self.needs_end = false;
                    let last = self.prev;
                    let first = self.first;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub fn convert_path(p: &usvg::Path) -> PathConvIter {
    PathConvIter {
        iter: p.data.iter(),
        first: Point::new(0.0, 0.0),
        prev: Point::new(0.0, 0.0),
        deferred: None,
        needs_end: false,
    }
}

pub fn convert_stroke(s: &usvg::Stroke) -> (usvg::Color, StrokeOptions) {
    let color = match s.paint {
        usvg::Paint::Color(c) => c,
        _ => FALLBACK_COLOR,
    };
    let linecap = match s.linecap {
        usvg::LineCap::Butt => tessellation::LineCap::Butt,
        usvg::LineCap::Square => tessellation::LineCap::Square,
        usvg::LineCap::Round => tessellation::LineCap::Round,
    };
    let linejoin = match s.linejoin {
        usvg::LineJoin::Miter => tessellation::LineJoin::Miter,
        usvg::LineJoin::Bevel => tessellation::LineJoin::Bevel,
        usvg::LineJoin::Round => tessellation::LineJoin::Round,
    };

    let opt = StrokeOptions::tolerance(0.01)
        .with_line_width(s.width.value() as f32)
        .with_line_cap(linecap)
        .with_line_join(linejoin);

    (color, opt)
}

unsafe impl bytemuck::Pod for GpuGlobals {}
unsafe impl bytemuck::Zeroable for GpuGlobals {}
unsafe impl bytemuck::Pod for GpuPrimitive {}
unsafe impl bytemuck::Zeroable for GpuPrimitive {}
unsafe impl bytemuck::Pod for GpuTransform {}
unsafe impl bytemuck::Zeroable for GpuTransform {}
