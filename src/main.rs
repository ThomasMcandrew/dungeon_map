use gtk4 as gtk;
use gtk::prelude::*;
use gtk::gdk::cairo::*;
use gtk::
    {
        Application,
        ApplicationWindow,
        Widget,
        ListBox,
        ScrolledWindow,
        ListBoxRow,
        Button,
        FlowBox,
        FlowBoxChild,
        DrawingArea,
        
        Image,
    };
use gtk::gdk_pixbuf::*;

struct SimpleImage {
    area: DrawingArea,
    width: i32,
    height: i32,
    sprite_width: i32,
    sprite_height: i32,
    im_vec: Vec<Pixbuf>,
}
impl SimpleImage {
    pub fn from_image(image: Pixbuf, w: i32, h: i32) -> Self {
        let sprite_w = image.width() / w;
        let sprite_h = image.height() / h;
        let mut images_vec : Vec<Pixbuf> = Vec::new();
        for x in 0..w {
            for y in 0..h {
                let i = image.new_subpixbuf(x,y,sprite_w,sprite_h).expect("invalid subpicture");
                images_vec.push(i);
            }
        }
        
        let draw = DrawingArea::new();
        draw.set_draw_func(|da: &DrawingArea,c: &Context,w: i32,h: i32|{
            c.set_line_width(1.);
            
            for i in 0..images_vec.len() {
                let si = images_vec[i];
                c.set_source_pixbuf(&si,
                    (sprite_w * (i as i32 / w)).into(),
                    (sprite_h * (i as i32 / h)).into());
            }
    
    
            c.paint().expect("Failed to paint");
        });
        SimpleImage {
            area: DrawingArea::new(),
            width: w,
            height: h,
            sprite_width: sprite_w,
            sprite_height: sprite_h,
        }
    }
    pub fn new() -> Self {
        SimpleImage {
            area: DrawingArea::new(),
            width: 0,
            height: 0,
            sprite_width: 0,
            sprite_height: 0,
        }
    }
}

fn idk() -> impl IsA<Widget> {
    let draw = DrawingArea::new();
    draw.set_draw_func(|da: &DrawingArea,c: &Context,w: i32,h: i32|{
        c.set_line_width(1.);

        let pbx = Pixbuf::from_file("sky-background.png").expect("");
        let foo = pbx.new_subpixbuf(32,32,32,32).expect("failed to get subpix");

        c.set_source_pixbuf(&foo,0.,32.);

        c.paint().expect("Failed to paint");
    });
    
    let display = gtk::gdk::Display::default().expect("No Default display");
    let im = ImageSurface::create(Format::ARgb32,32,32).expect("Invalid surface");
    draw
}

fn map_screen() -> impl IsA<Widget> {
    // let test = Image::from_file("sky-background.png"); 

    // let scroll = ScrolledWindow::new();
    // scroll.set_child(Some(&test));
    // 
    // scroll
    idk()
}

fn root() -> impl IsA<Widget> {
    let fbx = FlowBox::new();
    
    fbx.set_max_children_per_line(2);

    let scenes = scenes_panel();
    let scenes2 = map_screen();
   
    let fbc = FlowBoxChild::new();
    fbc.set_child(Some(&scenes));
    fbx.insert(&fbc,1);
   
    let fbc2 = FlowBoxChild::new();
    fbx.insert(&fbc2,1);
    fbc2.set_child(Some(&scenes2));
  
    fbx
}

fn scenes_panel() -> impl IsA<Widget> {
    let list = ListBox::new();
    let scroll = ScrolledWindow::new();

    for _ in 0..80 {
        let button = Button::with_label("Click me!");
        let lbr = ListBoxRow::new();
        lbr.set_child(Some(&button));
        list.append(&lbr);
    }
    scroll.set_child(Some(&list));
    scroll
}
fn main() {
    let application = Application::builder()
        .application_id("com.example.dungeon_map")
        .build();
    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("dmap")
            .build();
        let root = root();
        window.set_child(Some(&root));
        window.show();
    });
    application.run();
}
