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
            presentation::Scene::new("one".into()),
            presentation::Scene::new("two".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("three".into()),
            presentation::Scene::new("four".into()),
            presentation::Scene::new("five".into())
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
pub fn load_image() -> ImageBuf {
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
