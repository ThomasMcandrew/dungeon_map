use druid::widget::
    {
        Align, 
        Flex, 
        Label, 
   };
use druid::
    {
       Widget, 
    };

use crate::ApplicationState;


pub fn build_current_scene() -> impl Widget<ApplicationState> {
    let label = Label::
        new("scenes");

    Align::centered(label)


}
