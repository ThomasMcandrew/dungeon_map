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
}

pub fn get_drawing_area_from_image(image: Pixbuf, width: i32, height: i32) -> DrawingArea {
    let sprite_w = image.width() / width;
    let sprite_h = image.height() / height;
    let mut images_vec : Vec<Pixbuf> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let i = image.new_subpixbuf(
                    x * sprite_w,
                    y * sprite_h,
                    sprite_w,
                    sprite_h)
                .expect("invalid subpicture");
            images_vec.push(i);
        }
    }
    
    let draw = DrawingArea::new();

    draw.set_draw_func(move |_da: &DrawingArea,c: &Context,w: i32,h: i32|{
        c.set_line_width(1.);
        
        for y in 0..height {
            for x in 0..width {
                let si = images_vec[((y * width) + x) as usize].clone();
                c.set_source_pixbuf(&si,
                    (x * sprite_w).into(),
                    (y * sprite_h).into());

                c.paint().expect("Failed to paint");
            }
        }
    });
    draw
}

fn map_screen() -> impl IsA<Widget> {
    let pbx = Pixbuf::from_file("cr.jpg").expect("");
    let scroll = ScrolledWindow::new();
    let si = get_drawing_area_from_image(pbx,10,10);
    scroll.set_child(Some(&si));
    scroll
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
