use druid::widget::
    {
        Flex, 
        Label,
        Container,
        Click,
        ControllerHost,
        List,
        Painter,
        Scroll,
        Padding,
    };
use druid::
    {
        Widget,
        WidgetExt,
        Color,
        Env,
        ImageBuf,
        RenderContext,
        FontFamily,
        FontDescriptor,
        Insets,
    };
use druid::piet::InterpolationMode;

use crate::ApplicationState;
use crate::presentation::Scene;
use crate::delegate::SWITCH;

pub fn build_scenes_widgit() -> impl Widget<ApplicationState> {
    let list = List::new(build_scene_widgit)
        .lens(ApplicationState::scenes);

    Scroll::new(list)
}


pub fn build_scene_widgit() -> impl  Widget<Scene> {
    let name = Label::
        new(|data: &Scene, _env: &Env| data.name.clone())
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE));
    let created_date = Label::
        new(|data: &Scene, _env: &Env| data.created_date.clone())
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE));
    let updated_date = Label::
        new(|data: &Scene, _env: &Env| data.updated_date.clone())
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE));

    let row_created_date = Flex::row()
        .with_child(
            Label::new("Created Date:")
                .with_font(
                    FontDescriptor::new(
                        FontFamily::MONOSPACE
                )))
        .with_default_spacer()
        .with_child(created_date);

    let row_updated_date = Flex::row()
        .with_child(
            Label::new("Updated Date:")
                .with_font(
                    FontDescriptor::new(
                        FontFamily::MONOSPACE
                )))
        .with_default_spacer()
        .with_child(updated_date);
    

    let layout = Flex::column()
        .with_child(name)
        .with_default_spacer()
        .with_default_spacer()
        .with_default_spacer()
        .with_child(row_created_date)
        .with_child(row_updated_date);

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

    let controller = ControllerHost::new(layout,click);
    let container = Container::new(controller)
        .background(painter)
        .rounded(5.)
        .border(Color::WHITE, 2.);
    Padding::new(Insets::uniform(5.),container) 
}
