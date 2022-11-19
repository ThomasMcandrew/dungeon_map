//All these examples are in one file so all the imports are here
//you may not need some, or need different ones. consult the 
//compiler for help with that. :)

use std::path::Path;
use druid::widget::
{
    Image,
    ViewSwitcher,
};
use druid::piet::ImageFormat;
use druid::
{
    WindowDesc,
    AppLauncher,
    Widget,
    Data,
    ImageBuf,
    WidgetExt,
    Lens,
};

use imagesize::size;

pub fn image_widget() -> impl Widget<()> {
	let image_buf = ImageBuf::empty();
	let image = Image::new(image_buf);
    image
}

pub fn load_image() -> ImageBuf {
    let bytes = include_bytes!("/home/thomas/dungeon_map/maps/ex2.jpg");
    let (width, height) = match 
        size("/home/ex1.jpg") {
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

pub fn image_widget_with_image() -> impl Widget<()> {
    let image_buf = load_image();
    let image = Image::new(image_buf);
    image
}

pub fn load_image_by_path(path_to_file: &str) -> ImageBuf {
    let path = Path::new(path_to_file);
    
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

pub fn image_widget_with_image_from_path() -> impl Widget<()> {
    let image_buf = load_image_by_path("/home/file.jpg");
    let image = Image::new(image_buf);
    image
}

#[derive(Data,Clone,Lens)]
pub struct AppState {
    pub image_buf: Option<ImageBuf>,
}

impl AppState {
    pub fn new(path_to_file: &str) -> Self {
        AppState { image_buf: load_image_by_path_new(path_to_file) }
    }
}

pub fn load_image_by_path_new(path_to_file: &str) -> Option<ImageBuf> {
    let path = Path::new(path_to_file);
    
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
    Some(
        ImageBuf::from_raw(
        bytes,
        ImageFormat::Rgb,
        width,
        height
    ))
}

pub fn root_widget() -> impl Widget<AppState> {
    image_widget_from_state()
}

pub fn image_widget_from_state() -> impl Widget<AppState> {
    let image = ViewSwitcher::new(
        |data: &AppState, _env| data.image_buf.is_some(),
        move |f, data: &AppState, _env| {
            if *f {
                Box::new(Image::new(
                        data.image_buf
                            .clone()
                            .unwrap()))
            } else {
                Box::new(Image::new(ImageBuf::empty()))
            }
    });
    image
}

pub fn widget_with_lens() -> impl Widget<AppState> {
    image_widget_from_image_option()
        .lens(AppState::image_buf)
}

pub fn image_widget_from_image_option() -> impl Widget<Option<ImageBuf>> {
    let image = ViewSwitcher::new(
        |data: &Option<ImageBuf>, _env| data.is_some(),
        move |f, data: &Option<ImageBuf>, _env| {
            if *f {
                Box::new(Image::new(
                        data.clone()
                            .unwrap()))
            } else {
                Box::new(Image::new(ImageBuf::empty()))
            }
    });
    image
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(root_widget())
        .window_size((400.0, 400.0));
    let app_state = AppState {
        image_buf: load_image_by_path_new("/home/thomas/dungeon_map/maps/ex2.jpg")
    };
    // start the application
    AppLauncher::with_window(main_window)
        .launch(app_state)
        .expect("Failed to launch application");
}
