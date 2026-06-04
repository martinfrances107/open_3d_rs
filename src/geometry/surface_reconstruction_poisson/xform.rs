#![feature(generic_const_exprs)]
#![allow(unused, incomplete_features)]

use std::cmp::max;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::io::Point;

#[derive(Debug)]
pub struct XForm<REAL, const DIM: usize>
where
    REAL: Copy,
{
    min: Point<REAL, DIM>,
    max: Point<REAL, DIM>,
}

impl<REAL, const DIM: usize> Default for XForm<REAL, DIM>
where
    REAL: Copy + std::default::Default,
{
    fn default() -> Self {
        Self {
            min: [REAL::default(); DIM],
            max: [REAL::default(); DIM],
        }
    }
}

impl<REAL, const DIM: usize> XForm<REAL, DIM>
where
    REAL: Copy
        + Add<REAL, Output = REAL>
        + Div<REAL, Output = REAL>
        + Ord
        + Mul<REAL, Output = REAL>
        + MulAssign<REAL>
        + Sub<REAL, Output = REAL>,
    // + SubAsign<REAL, Output = REAL>,
    [REAL; DIM]: Add<[REAL; DIM], Output = [REAL; DIM]>,
    [REAL; DIM]: Div<f32, Output = [REAL; DIM]>,
{
    pub fn get_bounding_box_x_form_with_scale_factor<const DIMPLUSONE: usize>(
        min_p: Point<REAL, DIM>,
        max_p: Point<REAL, DIM>,
        scale_factor: REAL,
    ) -> XForm<REAL, DIMPLUSONE>
    where
        REAL: SubAssign<REAL> + Default,
        REAL: From<i32>,
        XForm<REAL, DIMPLUSONE>: Mul<Output = XForm<REAL, DIMPLUSONE>>,
        usize: From<REAL>,
    {
        let mut center = (max_p + min_p) / 2_f32;
        let mut scale = max_p[0] - min_p[0];
        for d in 1..DIM {
            scale = max(scale, max_p[d] - min_p[d]);
        }
        scale *= scale_factor;

        for i in 0..DIM {
            center[i] -= scale / REAL::from(2i32);
        }

        let mut t_xform = XForm::<REAL, DIMPLUSONE>::default();
        let mut s_xform = XForm::<REAL, DIMPLUSONE>::default();

        // Undertand to the code here.
        unimplemented!();

        s_xform * t_xform
    }

    pub fn get_bounding_box_x_form_with_width_scale_factor_depth<
        const DIMPLUSONE: usize,
    >(
        min_p: Point<REAL, DIM>,
        max_p: Point<REAL, DIM>,
        width: REAL,
        scale_factor: REAL,
        depth: usize,
    ) -> XForm<REAL, DIMPLUSONE>
    where
        REAL: SubAssign<REAL> + Default,
        REAL: From<i32>,
        XForm<REAL, DIMPLUSONE>: Mul<Output = XForm<REAL, DIMPLUSONE>>,
        usize: From<REAL>,
    {
        let mut resolution = (max_p[0] - min_p[0]) / width;
        for d in 1..DIM {
            resolution = max(resolution, (max_p[d] - min_p[d]) / width);
        }
        resolution *= scale_factor;

        let resolution = usize::from(resolution);
        let mut depth = 0usize;
        loop {
            if (1 << depth) < resolution {
                break;
            }
            depth += 1usize;
        }

        let mut center = (max_p + min_p) / 2_f32;
        let scale = REAL::from(1 << depth) * width;

        for i in 0..DIM {
            center[i] -= scale / REAL::from(2i32);
        }

        let mut t_xform = XForm::<REAL, DIMPLUSONE>::default();
        let mut s_xform = XForm::<REAL, DIMPLUSONE>::default();

        // Undertand to the code here.
        unimplemented!();

        s_xform * t_xform
    }

    pub fn get_point_x_form_with_width_scale_depth<const DIMPLUSONE: usize>(
        // stream: &InputPointStream<f32, DIM>,
        width: REAL,
        scale_factor: REAL,
        depth: i16,
    ) -> XForm<REAL, { DIMPLUSONE }> {
        todo!();
    }
}

impl<REAL: Copy, const DIM: usize> XForm<REAL, DIM> {
    pub fn get_point_x_form_with_scale<const DIMPLUSONE: usize>(
        // stream: &InputPointStream<f32, DIM>,
        scale_factor: REAL,
    ) -> XForm<REAL, { DIMPLUSONE }> {
        todo!();
    }
}
