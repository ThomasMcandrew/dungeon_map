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
        im::Vector, 
    };

const WINDOW_TITLE: LocalizedString<ApplicationState> = LocalizedString::new("Hello World!");

mod presentation;

#[derive(Clone, Data, Lens)]
pub struct ApplicationState {
    name: String,
    scenes: Vector<presentation::Scene>,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = ApplicationState {
        name: "World".into(),
        scenes: Vector::from(vec!(
                presentation::Scene{
                    id: 0,
                    name: "foo".into(),
                    created_date: "cd".into(),
                    updated_date: "ud".into(),
                },
                presentation::Scene{
                    id: 1,
                    name: "foo1".into(),
                    created_date: "cd1".into(),
                    updated_date: "ud1".into(),
                },
                presentation::Scene{
                    id: 02,
                    name: "foo2".into(),
                    created_date: "cd2".into(),
                    updated_date: "ud2".into(),
                },
                presentation::Scene{
                    id: 3,
                    name: "foo3".into(),
                    created_date: "cd3".into(),
                    updated_date: "ud3".into(),
                },
            )),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<ApplicationState> {
    let container = Container::new(presentation::build_presentation())
        .border(Color::RED, 5.);

    Align::centered(container)
}
