use speedy2d::color::Color;
use speedy2d::Graphics2D;
use crate::assets::Assets;

struct DrawScaled<'a, 'b> {
    grph: &'a mut  Graphics2D,
    assets: &'b Assets,
}

impl <'a, 'b>DrawScaled<'a, 'b> {
    pub fn new(graphics2d: &'a mut  Graphics2D, assets: &'b Assets) -> DrawScaled<'a, 'b> {
        DrawScaled { grph: graphics2d, assets }
    }
    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: &Color) {
        
    }
}