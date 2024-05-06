use slint::ComponentHandle;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

// slint::slint! {
//     import {FancyText} from "fancy_text.slint";
// }

slint::slint! {
    import { Button } from "std-widgets.slint";
    import { FancyText } from "ui/fancy_text.slint";
export component Window1 inherits Window {
        width: 300px;
        height: 300px;
        callback button-clicked <=> button.clicked;
        callback other-button-clicked <=> other_button.clicked;
        VerticalLayout {
            Text { text: "Hello from Window 1"; }
            button := Button {
                text: "Change text in Window 2";
            }
            other_button := Button {
                text: "Also Change text in Window 2";
            }
            FancyText {}
        }
    }
}

slint::slint! {
    export component Window2 inherits Window {
        width: 300px;
        height: 300px;
        in property<string> text_property_with_default: "Default property text";
        in property<string> text_property;
        // in property<string> title <=> title_text;
        VerticalLayout {
        title_text := Text { text: "Hello from Window 2";} // Static text
        Text { text:text_property_with_default;} // Text from a property default
        Text { text:text_property;} // Text set from a rust callback
        }
    }
}


fn main() -> Result<(), slint::PlatformError> {
    let window1 = Window1::new()?;
    // let window2 = Window2::new()?;
    let window2 = Rc::new(RefCell::new(Window2::new()?));
    // let window3 = Window2::new()?; // a second instance of Window2
    window2.borrow_mut().set_text_property("Text from set_text".into());

    window1.show()?;
    window2.borrow_mut().show()?;
    // window3.show()?;

    let window2_ = window2.clone();
    window1.on_button_clicked(move ||{
        println!("Button Clicked!");
        window2_.borrow_mut().set_text_property("First Button Clicked!".into());
    });

    let window2_ = window2.clone();
    window1.on_other_button_clicked(move ||{
        println!("Other Button Clicked!");
        window2_.borrow_mut().set_text_property("Second Button Clicked!".into());
    });

    slint::run_event_loop()
}