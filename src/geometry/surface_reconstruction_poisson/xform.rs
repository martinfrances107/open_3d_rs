use crate::io::Point;

#[derive(Debug)]
pub struct XForm<REAL: Copy, const DIM: usize> {
    min: Point<REAL, DIM>,
    max: Point<REAL, DIM>,
}

impl<REAL, const DIM: usize> Default for XForm<REAL, DIM>
where
    REAL: Copy + Default,
{
    fn default() -> Self {
        Self {
            min: [REAL::default(); DIM],
            max: [REAL::default(); DIM],
        }
    }
}

impl<REAL: Copy, const DIM: usize> XForm<REAL, DIM> {
    fn get_point_x_form<const DIMPLUSONE: usize>(
        // stream: &InputPointStream<f32, DIM>,
        width: REAL,
        scale_factor: REAL,
        depth: i16,
    ) -> XForm<REAL, { DIMPLUSONE }> {
        todo!();
    }
}
