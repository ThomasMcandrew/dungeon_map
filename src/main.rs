use druid::widget::Container;
use druid::
    {
        AppLauncher, 
        Data, 
        Lens, 
        LocalizedString, 
        Widget, 
        WindowDesc, 
        ImageBuf,
        im::Vector, 
    };

use druid::piet::ImageFormat;
use imagesize::size;
use std::path::Path;


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
    let container = Container::new(presentation::build_presentation());
    container
}
/*
 * we want to make a save and load in the scene struct
 * then make a button that adds one so we can see if they are 
 * loading and saving correctly, then adding one switches the root
 * widget to the builder and right now just has name and save button 
 * stuff
 * */
pub fn load_image() -> ImageBuf {
    let path = Path::new("/home/thomas/dungeon_map/maps/ex2.jpg");
    
    let image_bytes = image::io::Reader::open(path)
        .unwrap()
        .decode()
        .unwrap();
    
    let bytes = image_bytes.as_bytes();
    
    let (width, height) = match 
        size(path) {
            Ok(dim) => (dim.width, dim.height),
            Err(_) => (0,0) 
    };
    ImageBuf::from_raw(
        bytes,
        ImageFormat::Rgb,
        width,
        height
    )
}
