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

fn idk() -> impl IsA<Widget> {
    let draw = DrawingArea::new();
    draw.set_draw_func(|da: &DrawingArea,c: &Context,w: i32,h: i32|{
        c.set_line_width(1.);

        let pbx = Pixbuf::from_file("sky-background.png").expect("");
        let bytes = pbx.pixel_bytes().expect("");
        for x in 0..pbx.width() {
            for y in 0..pbx.height() {
                let color = bytes[(x + (y * pbx.width())) as usize];
                
                let red = (color.rotate_right(16)) & 0xFF;
                let green = (color.rotate_right(8)) & 0xFF;
                let blue = (color) & 0xFF;
                c.set_source_rgb(red as f64,green as f64,blue as f64);
                c.rectangle(x as f64,y as f64,1.,1.);
                c.fill().expect("Failed to paint");
            }
        }
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
