use druid::widget::
    {
        Container,
        ViewSwitcher,
        Image,
        Scroll,
   };
use druid::
    {
        Widget, 
        WidgetExt,
        LensExt,
        ImageBuf,

    };

use crate::ApplicationState;
use crate::presentation::Scene;

pub fn build_current_scene() -> impl Widget<ApplicationState> {
    let container = Container::new(get_background_image())
        .lens(ApplicationState::current_scene.then(Scene::full_image));
    container
}

pub fn get_background_image() -> impl Widget<Option<ImageBuf>> {
    Scroll::new(
    ViewSwitcher::new(
        |data: &Option<ImageBuf>, _env| data.is_some(),
            move |f, data: &Option<ImageBuf>, _env| {
                if *f {
                    Box::new(Image::new(data.clone().unwrap()))
                } else {
                    Box::new(Image::new(ImageBuf::empty()))
                }
    }))
}
