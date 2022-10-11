use druid::widget::
    {
        Align, 
        Flex, 
        Label,
        Container,
        Click,
        ControllerHost,
        List,
    };
use druid::
    {
        Widget,
        WidgetExt,
        Color,
        Env,
    };

use crate::ApplicationState;
use crate::presentation::Scene;

pub fn build_scenes_widgit() -> impl Widget<ApplicationState> {
    let list = List::new(build_scene_widgit).lens(ApplicationState::scenes);
        
    let container = Container::new(list)
        .border(Color::BLACK, 5.);

    Align::centered(container)
}


pub fn build_scene_widgit() -> impl  Widget<Scene> {
    let name = Label::
        new(|data: &Scene, _env: &Env| data.name.clone());
    let created_date = Label::
        new("10-3-3");
    let updated_date = Label::
        new("10-3-3");
    let layout = Flex::row()
        .must_fill_main_axis(true)
        .with_child(name)
        .with_child(created_date)
        .with_child(updated_date);

    let click = Click::new(|_ctx, data: &mut Scene, _env|{
        println!("Click")
    });

    let container = Container::new(layout)
        .border(Color::PURPLE, 5.);
    
    let controller = ControllerHost::new(container,click);

    Align::centered(controller)
}
