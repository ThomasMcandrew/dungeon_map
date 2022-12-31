use gtk4 as gtk;
use gtk::prelude::*;
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
        Image, 
        DrawingArea,
        
    };

fn idk() {
    let draw = DrawingArea::new();
    let test = Image::from_file("sky-background.png"); 
    
}

fn map_screen() -> impl IsA<Widget> {
    let test = Image::from_file("sky-background.png"); 

    let scroll = ScrolledWindow::new();
    scroll.set_child(Some(&test));
    
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
