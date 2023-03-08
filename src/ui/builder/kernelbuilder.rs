

use relm4::component::*;

use relm4::factory::*;
use relm4::prelude::*;
use adw::prelude::*;

use super::lib::kernel::Kernel;


#[derive(Debug)]
pub struct Builder{
    index:u16,
    kernel_list: Vec<Kernel>,
    is_active: bool,
    label: String,
}

#[derive(Debug)]
pub enum BuilderMsg{
    Remove(usize),
    Add(String),
    Show(DynamicIndex),
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
    type Init = (Vec<Kernel>,u16);
    type Input = BuilderMsg;
    type Output = BuilderMsgOutput;
    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self>{

        let model = Self {
            kernel_list: _init.0,
            index: _init.1,
            is_active:false,
            label: "Hello world!".to_string(),
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>){
        match msg{
            BuilderMsg::Show(index) => {
                self.index = index.current_index() as u16;
                self.label=  self.kernel_list[self.index as usize].version.clone();
        
                self.is_active = true;
            }
            BuilderMsg::Build => println!("how did you call this?"),
            BuilderMsg::CloseDialog => {
                self.is_active=false;
                println!("Closed");
            },
            BuilderMsg::Add(name) =>{
                let kernel = Kernel{
                    version: name,
                    url: None,
                    path: Some("./kernel".to_string()),
                };
                self.kernel_list.push(kernel);
            }
            BuilderMsg::Remove(index) => {
                self.kernel_list.remove(index);
            }

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
                    #[watch]
                    set_label: &format!("{}",model.label),
                    add_css_class: &"title-1",
                }
            }
            

        }

    }
}