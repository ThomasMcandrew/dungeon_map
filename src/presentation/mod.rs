use druid::widget::
    {
        Align, 
        Split,
        Image,
        ViewSwitcher,
    };

use druid::
    {
        Widget,
        Data,
        Lens,
        ImageBuf,
    };
use chrono::Utc;

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
    pub full_image: Option<ImageBuf>,
}

impl Scene {
    pub fn new(name: String) -> Scene {
        let current_time = Utc::now();
        Scene { 
            id: 0,
            name,
            created_date: format!("{:?}{:?}",
                current_time.date(),current_time.time()),
            updated_date: Utc::now().to_string(),
            full_image: Some(crate::load_image())
        }
    }
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
