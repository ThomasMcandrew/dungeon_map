use druid::widget::
    {
        Flex, 
        Label,
        Container,
        Click,
        ControllerHost,
        List,
        Scroll,
        ViewSwitcher,
        Padding,
        Image,
        FillStrat,
    };
use druid::
    {
        Widget,
        WidgetExt,
        Color,
        Env,
        ImageBuf,
        FontFamily,
        FontDescriptor,
        Insets,
    };

use crate::ApplicationState;
use crate::presentation::Scene;
use crate::delegate::SWITCH;

const PREVIEW_WIDTH : f64 = 300.;
const PREVIEW_HEIGHT : f64 = 175.;

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
        .with_child(scene_image())
        .with_child(row_created_date)
        .with_child(row_updated_date);

    let click = Click::new(|_ctx, data: &mut Scene, _env|{
        _ctx.submit_command(SWITCH.with(data.clone()));
    });

    let controller = ControllerHost::new(layout,click);

    let container = Container::new(controller)
        .rounded(5.)
        .fix_size(PREVIEW_WIDTH,PREVIEW_HEIGHT)
        .border(Color::WHITE, 2.);

    Padding::new(Insets::uniform(5.),container) 
}


pub fn scene_image() -> impl Widget<Scene> {
    let image = ViewSwitcher::new(
        |data: &Scene, _env| data.full_image.is_some(),
        move |f, data: &Scene, _env| {
            if *f {
                Box::new(Image::new(
                        data.full_image
                            .clone()
                            .unwrap())
                    .fill_mode(FillStrat::Fill)
                    .fix_size(PREVIEW_WIDTH,PREVIEW_HEIGHT - 50.))
            } else {
                Box::new(Image::new(ImageBuf::empty()))
            }
    });
    image
}
