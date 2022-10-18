use druid::widget::
    {
        Align, 
        Flex, 
        Label,
        Container,
        Click,
        ControllerHost,
        List,
        Painter,
        Scroll,
    };
use druid::
    {
        Widget,
        WidgetExt,
        Color,
        Env,
        ImageBuf,
        RenderContext,
    };
use druid::piet::InterpolationMode;

use crate::ApplicationState;
use crate::presentation::Scene;
use crate::delegate::SWITCH;

pub fn build_scenes_widgit() -> impl Widget<ApplicationState> {
    let list = List::new(build_scene_widgit)
        .lens(ApplicationState::scenes);
    let container = Container::new(list)
        .border(Color::BLACK, 5.);
    Align::centered(container)
}


pub fn build_scene_widgit() -> impl  Widget<Scene> {
    let name = Label::
        new(|data: &Scene, _env: &Env| data.name.clone());
    let created_date = Label::
        new(|data: &Scene, _env: &Env| data.created_date.clone());
    let updated_date = Label::
        new(|data: &Scene, _env: &Env| data.updated_date.clone())
        .align_right();

    let row_date = Flex::row()
        .must_fill_main_axis(true)
        .with_child(created_date)
        .with_child(updated_date);

    let row_name = Flex::row()
        .must_fill_main_axis(true)
        .with_child(name);
    
    let layout = Flex::column()
        .must_fill_main_axis(true)
        .with_child(row_name)
        .with_default_spacer()
        .with_default_spacer()
        .with_default_spacer()
        .with_default_spacer()
        .with_child(row_date);

    let click = Click::new(|_ctx, data: &mut Scene, _env|{
        _ctx.submit_command(SWITCH.with(data.clone()));
    });

    let painter = Painter::new(
        |ctx, data: &Scene, _env| {
             let rect = ctx.size().to_rect();
             let img = if let Some(im) = data.full_image.clone() {
                im
             } else {
                ImageBuf::empty()
             };
             let img = img.to_image(ctx.render_ctx);
             ctx.with_save(|ctx| {
                 ctx.draw_image(
                     &img, 
                     rect, 
                     InterpolationMode::Bilinear);
             });
        });

    let container = Container::new(layout)
        .background(painter)
        .border(Color::PURPLE, 5.);
    
    let controller = ControllerHost::new(container,click);
    Align::centered(controller)
}
