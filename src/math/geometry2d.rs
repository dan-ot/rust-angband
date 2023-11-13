use super::{Vector2f, LayoutTransform2d, transform2d::RenderTransform2d};

pub struct Geometry2d {
    accumulated_render_transform: Option<RenderTransform2d>,

    pub size: Vector2f,
    pub scale: f32,
    pub absolute_position: Vector2f,
    pub position: Vector2f
}

impl Geometry2d {
    pub fn new_size_layout_render_parent(
        local_size: Vector2f,
        local_layout: LayoutTransform2d,
        local_render: RenderTransform2d,
        local_render_pivot: Vector2f,
        parent_accumulated_layout: LayoutTransform2d,
        parent_accumulated_render: RenderTransform2d
    ) -> Geometry2d {
        let pivot_to_local = -(local_size * local_render_pivot);
        let render_to_local = concatenate_vtor(pivot_to_local, local_render);
        let translate_pivot = concatenate_rtov(render_to_local, local_size * local_render_pivot);
        let applied_layout = translate_pivot.concatenate(local_layout.into());
        let parent_to_root = applied_layout.concatenate(parent_accumulated_render);

        let accumulated_layout = local_layout.concatenate(parent_accumulated_layout);
        Geometry2d { 
            accumulated_render_transform: Option::Some(parent_to_root),
            size: local_size, 
            scale: accumulated_layout.scale, 
            absolute_position: accumulated_layout.translation, 
            position: local_layout.translation
        }
    }

}

pub fn concatenate_vtor(v: Vector2f, rt: RenderTransform2d) -> RenderTransform2d {
    RenderTransform2d { 
        matrix: rt.matrix, 
        translation: rt.matrix.transform(v) + rt.translation
    }
}

pub fn concatenate_rtov(rt: RenderTransform2d, v: Vector2f) -> RenderTransform2d {
    RenderTransform2d { 
        matrix: rt.matrix, 
        translation: rt.translation + v
    }
}
