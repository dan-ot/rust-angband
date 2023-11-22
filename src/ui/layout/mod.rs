pub mod geometry2d;
pub mod transform2d;
pub mod slot;

pub use geometry2d::Geometry2d;
pub use transform2d::LayoutTransform2d;
pub use transform2d::RenderTransform2d;
use crate::math::Vector2f;

pub struct WidgetPersistentState { }

pub struct ArrangedWidgetChildren { }

pub trait LayoutWidget {
    fn needs_prepass() -> bool;
    // fn prepass_update_caches(layout_scale_multiplier: f32);
    fn desired_size(layout_scale_multiplier: f32) -> Vector2f;
    fn persistent_state() -> WidgetPersistentState;
    fn assign_parent() -> ();
    fn relative_layout_scale(child_index: i32, layout_scale_multiplier: f32) -> f32;
    fn arrange_children(allotted_geometry: Geometry2d, update_attributes: bool) -> ArrangedWidgetChildren;
}