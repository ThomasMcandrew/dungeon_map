use druid::widget::
    {
        Align, 
        Split,
    };
use druid::
    {
        Widget,
        Data,
        Lens,
    };


mod scenes;
mod current_scene;

use crate::ApplicationState;

#[derive(Clone, Data, Lens)]
pub struct Scene
{
    pub id: i32,
    pub name: String,
    pub created_date: String,
    pub updated_date: String,
}

pub fn build_presentation() -> impl Widget<ApplicationState> {
    let scenes_window = scenes::build_scenes_widgit();
    let current_scene = current_scene::build_current_scene();

    let split = Split::columns(
                scenes_window,
                current_scene
            )
        .draggable(true);

    Align::centered(split)
}
