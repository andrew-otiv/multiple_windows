use slint::ComponentHandle;

slint::slint! {
    import { Button } from "std-widgets.slint";
export component Window1 inherits Window {
        width: 300px;
        height: 300px;
        callback button-clicked <=> button.clicked;
        Text { text: "Hello from Window 1"; }
        button := Button {
            text: "Change text in Window 2";
        }
    }
}

slint::slint! {
    export component Window2 inherits Window {
        width: 300px;
        height: 300px;
        in property<string> text_property_with_default: "Default property text";
        in property<string> text_property;
        VerticalLayout {
        Text { text: "Hello from Window 2";} // Static text
        Text { text:text_property_with_default;} // Text from a property default
        Text { text:text_property;} // Text set from a rust callback
        }
    }
}


fn main() -> Result<(), slint::PlatformError> {
    let window1 = Window1::new()?;
    let window2 = Window2::new()?;
    window2.set_text_property("Text from set_text".into());

    window1.show()?;
    window2.show()?;

    window1.on_button_clicked(move ||{
        println!("Button Clicked!");
        window2.set_text_property("Text via rust callback".into());
    });

    slint::run_event_loop()
}