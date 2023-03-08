use relm4::component::*;
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::*;
use relm4::prelude::*;
use adw::prelude::*;


#[derive(Debug)]
pub struct Builder{
    is_active: bool,
    label: String,
}

#[derive(Debug)]
pub enum BuilderMsg{
    Show,
    Build,
    CloseDialog,
}
#[derive(Debug)]
pub enum BuilderMsgOutput{
    FailedBuild,
    SuccessFullBuild,
}

#[relm4::component(pub)]
impl SimpleComponent for Builder {
    type Init = ();
    type Input = BuilderMsg;
    type Output = BuilderMsgOutput;
    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self>{

        let mut model = Self {
            is_active:false,
            label: "Hello world!".to_string(),
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>){
        match msg{
            BuilderMsg::Show => {
                self.is_active = true;
            }
            BuilderMsg::Build => println!("how did you call this?"),
            BuilderMsg::CloseDialog => {
                self.is_active=false;
                println!("Closed");
            },

        }
    }

    view! {
        window = adw::Window{
            #[watch]
            set_visible: model.is_active,
            set_modal: true,
            set_title: Some("Kernel Manager"),
            set_default_size: (800, 600),
            set_vexpand: true,

            connect_close_request[sender] => move |_| {
                sender.input(BuilderMsg::CloseDialog);
                gtk::Inhibit(true)
            },

            gtk::Box{
                set_valign: gtk::Align::Center,
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 32,
                
                gtk::Label{
                    set_label: &format!("{}",model.label),
                    add_css_class: &"title-1",
                }
            }
            

        }

    }
}