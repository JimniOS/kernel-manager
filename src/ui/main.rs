use gtk::prelude::*;
use relm4::prelude::*;

pub struct App{
    state:String,
}



#[derive(Debug)]
pub enum AppMsg {
    Hello
}


#[relm4::component(pub)]
impl SimpleComponent for App{
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view!{
        window = gtk::Window{
            set_title: Some("Kernel Manager"),
            set_default_size: (700,500),

            gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                gtk::HeaderBar,

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_halign: gtk::Align::Center,
                    set_vexpand: true,
                    set_margin_top: 10,
                    gtk::Label{
                        #[watch]
                        set_label: &format!("{}",model.state),
                        set_valign: gtk::Align::Center,
                        set_justify: gtk::Justification::Center,
                        set_margin_top:32,
                        add_css_class:"title-1",
                        
                    },
                    
                    gtk::Button{
                        set_label: "Hello!",
                        set_css_classes: &["suggested-action", "pill"],
                        set_valign: gtk::Align::Center,
                        connect_clicked => AppMsg::Hello,
                        set_margin_top:50,
                    }
                }
            }
        }
    }


    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self{
            state: "Kernel manager reporting!".to_string(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Hello => {
                self.state = "Hello to you too!".to_string();
            }
        }
    }
}