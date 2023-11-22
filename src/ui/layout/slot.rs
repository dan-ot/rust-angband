
pub struct SizeBox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

pub enum HorizontalAlignment {
    Fill,
    Left,
    Center,
    Right
}

pub enum VerticalAlignment {
    Fill,
    Top,
    Center,
    Bottom
}

pub struct Slot {
    pub padding: SizeBox,
    pub horizontal_alignment: Option<HorizontalAlignment>,
    pub vertical_alignment: Option<VerticalAlignment>
}