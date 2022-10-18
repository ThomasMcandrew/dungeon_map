use druid::widget::
    {
        Align, 
        Container,
        
    };
use druid::
    {
        AppLauncher, 
        Data, 
        Lens, 
        LocalizedString, 
        Widget, 
        WindowDesc, 
        Color,
        ImageBuf,
        im::Vector, 
    };

use druid::piet::ImageFormat;
use imagesize::size;

const WINDOW_TITLE: LocalizedString<ApplicationState> = 
    LocalizedString::new("Hello World!");

mod presentation;
mod delegate;

#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    pub name: String,
    pub scenes: Vector<presentation::Scene>,
    pub current_scene: presentation::Scene,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));
    // create the initial app state
    let initial_state = ApplicationState {
        name: "World".into(),
        current_scene: presentation::Scene{
                    id: -1,
                    name: "default".into(),
                    created_date: "cd".into(),
                    updated_date: "ud".into(),
                    full_image: None,
                },
        scenes: Vector::from(vec!(
                presentation::Scene{
                    id: 0,
                    name: "foo".into(),
                    created_date: "cd".into(),
                    updated_date: "ud".into(),
                    full_image: None,
                },
                presentation::Scene{
                    id: 1,
                    name: "foo1".into(),
                    created_date: "cd1".into(),
                    updated_date: "ud1".into(),
                    full_image: Some(load_image()),
                },
                presentation::Scene{
                    id: 02,
                    name: "foo2".into(),
                    created_date: "cd2".into(),
                    updated_date: "ud2".into(),
                    full_image: None,
                },
                presentation::Scene{
                    id: 3,
                    name: "foo3".into(),
                    created_date: "cd3".into(),
                    updated_date: "ud3".into(),
                    full_image: None,
                },
                presentation::Scene{
                    id: 3,
                    name: "foo3".into(),
                    created_date: "cd3".into(),
                    updated_date: "ud3".into(),
                    full_image: None,
                },
                presentation::Scene{
                    id: 3,
                    name: "foo3".into(),
                    created_date: "cd3".into(),
                    updated_date: "ud3".into(),
                    full_image: None,
                },
                presentation::Scene{
                    id: 3,
                    name: "foo3".into(),
                    created_date: "cd3".into(),
                    updated_date: "ud3".into(),
                    full_image: None,
                },
                presentation::Scene{
                    id: 3,
                    name: "foo3".into(),
                    created_date: "cd3".into(),
                    updated_date: "ud3".into(),
                    full_image: None,
                },
                presentation::Scene{
                    id: 3,
                    name: "foo3".into(),
                    created_date: "cd3".into(),
                    updated_date: "ud3".into(),
                    full_image: None,
                },
                presentation::Scene{
                    id: 3,
                    name: "foo3".into(),
                    created_date: "cd3".into(),
                    updated_date: "ud3".into(),
                    full_image: None,
                },
                presentation::Scene{
                    id: 3,
                    name: "foo3".into(),
                    created_date: "cd3".into(),
                    updated_date: "ud3".into(),
                    full_image: None,
                },
            )),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .delegate(delegate::Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<ApplicationState> {
    let container = Container::new(presentation::build_presentation())
        .border(Color::RED, 5.);

    Align::centered(container)
}
fn load_image() -> ImageBuf {
    //let image_bytes = image::open("/home/thomas/dungeon_map/maps/ex1.jpg")
    //    .unwrap();
    //let bytes = image_bytes.as_bytes();
    let bytes = include_bytes!("/home/thomas/dungeon_map/maps/ex1.jpg");
    let (width, height) = match 
        size("/home/thomas/dungeon_map/maps/ex1.jpg") {
            Ok(dim) => (dim.width, dim.height),
            Err(_) => (0,0) 
    };
    let img = image::load_from_memory_with_format(
            bytes, 
            image::ImageFormat::Jpeg
        ).unwrap();
    ImageBuf::from_raw(
       img.into_bytes(),
       ImageFormat::Rgb,
       width,
       height)
}
