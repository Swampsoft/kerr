use ggez::graphics::{DrawParam, Point2};

pub fn fix_sprite(mut dp: DrawParam) -> DrawParam {
    dp.dest = Point2::new(dp.dest.x - dp.offset.x, dp.dest.y - dp.offset.y);
    dp
}
