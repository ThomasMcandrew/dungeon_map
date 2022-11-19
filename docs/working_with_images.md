# Working with Images in Druid


## Imports
``` rs
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
```

## Initial example
This is the base example for an empty image widget.
We want to expand upon thim more, I focus on loading
Images from the file system in this guide, but I think
drawing images is a different story. 
``` rs
pub fn image_widget() -> impl Widget<()> {
	let image_buf = ImageBuf::empty();
	let image = Image::new(image_buf);
    image
}
```

## Image from file system
To load in an image there is the macro include_bytes! and 
this seems to be easier to work with but has the disadvantage
of the file string needs to be hard coded and the file needs 
to exist at compile time. So next we will try to abstract 
that ability next.
``` rs
pub fn load_image() -> ImageBuf {
    let bytes = include_bytes!("/home/ex2.jpg");
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
```

## Images from path
Keeping everything pretty much the same we can pass in 
the path and use the reader rather than the include_bytes! 
macro.
``` rs
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
```

## Working with Data!
Define the AppState or struct for this image.
I prefer to wrap the ImageBuf in an Option
so that if the loading of the image is unsuccessful 
we can move on from it better.

``` rs
#[derive(Data,Clone,Lens)] //we dont need Lens right now but it is later
pub struct AppState {
    pub image_buf: Option<ImageBuf>,
}
impl AppState {
    pub fn new(path_to_file: &str) -> Self {
        AppState { image_buf: load_image_by_path_new(path_to_file) }
    }
}
```

We want to update our loading method so that we can account 
for the option response better.
``` rs
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
```

The best way to get the image buffer from the AppState
is through the ViewSwitcher, and this integrates with
our Option Setting so we can return a empty buffer 
and keep our app running.
``` rs
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

pub fn root_widget() -> impl Widget<AppState> {
    image_widget_from_state()
}
```

Then Finally if we want to we can make a widget that is 
specific to an image with a Lens. and attach it that way.
``` rs
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

pub fn widget_with_lens() -> impl Widget<AppState> {
    image_widget_from_image_option()
        .lens(AppState::image_buf)
}
```