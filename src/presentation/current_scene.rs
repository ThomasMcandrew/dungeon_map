use druid::widget::
    {
        Align, 
        Label, 
        Container,
        Painter,
   };
use druid::
    {
        Widget, 
        WidgetExt,
        Env,
        ImageBuf,
        RenderContext,
    };

use druid::piet::InterpolationMode;

use crate::ApplicationState;
use crate::presentation::Scene;

pub fn build_current_scene() -> impl Widget<ApplicationState> {
    let container = Container::new(build_scene())
        .lens(ApplicationState::current_scene);
    Align::centered(container)
}

pub fn build_scene() -> impl Widget<Scene> {
    let label = Label::new(|data: &Scene, _env: &Env| data.name.clone());
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
    let container = Container::new(label)
        .background(painter);

    Align::centered(container)
}
