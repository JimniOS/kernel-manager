
use relm4::component::*;
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::*;
use relm4::prelude::*;

use adw::prelude::*;

pub static mut random_counter:f32 = 6.0;
#[derive(Debug, Clone)]
pub enum GeneralAppMessages {
    Add,
    Remove(DynamicIndex),
}

#[derive(Debug)]
struct KernelListComponent {
    kernel_version: String,
    installed: bool,
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for KernelListComponent {
    type Init = (String, bool);
    type Input = GeneralAppMessages;
    type Output = GeneralAppMessages;
    type CommandOutput = ();
    type ParentInput = GeneralAppMessages;
    type ParentWidget = adw::PreferencesGroup;

    view! {
        root = adw::ComboRow{
            #[watch]
            set_title: &format!("{}",self.kernel_version),
            set_subtitle: "Amogus",
            set_sensitive: true,
            add_suffix = &gtk::Button{
                set_icon_name: "window-new-suffix",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,
                connect_clicked[sender,index] => move |_|{

                    sender.input(GeneralAppMessages::Remove(index.clone()));
                }
            }
        }
    }

    fn output_to_parent_input(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        Self {
            kernel_version: init.0,
            installed: init.1,
        }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncFactorySender<Self>) {
        sender.output(msg);
    }
}

#[derive(Debug)]
pub struct GeneralApp {
    kernel_list: AsyncFactoryVecDeque<KernelListComponent>,
}

#[relm4::component(pub)]
impl SimpleComponent for GeneralApp {
    type Init = ();
    type Input = GeneralAppMessages;
    type Output = ();
    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self>{

        let mut model = Self {
            kernel_list: AsyncFactoryVecDeque::new(adw::PreferencesGroup::new(), sender.input_sender()),
        };
        // testing vec
        let list = vec![("Linux 5.15".to_string(),false),("Linux 5.16".to_string(),true)];
        for (name,managed) in list{
            model.kernel_list.guard().push_back((name.clone(),managed));
        }

        let kernel_list = model.kernel_list.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>){
        match msg{
            GeneralAppMessages::Add => {
                unsafe{
                    random_counter+=0.1;
                    self.kernel_list.guard().push_back((format!("Kernel {:#0.1}",random_counter),false));
                }   
            }
            GeneralAppMessages::Remove(index) =>{
                if self.kernel_list.get(index.current_index()).unwrap().installed{
                    self.kernel_list.guard().remove(index.current_index());
                }else{
                    println!("Unimplemented");
                }
            }
        }
    }

    view! {
        window = adw::Window{
            set_title: Some("Kernel Manager"),
            set_default_size: (700, 560),
            set_vexpand: true,

            gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 32,
                adw::PreferencesPage {
                    set_title: &"Kernel List",
                    set_icon_name: Some("document-properties-symbolic-suffix"),
                    
                    add = &adw::PreferencesGroup{
                        #[local_ref]
                        add = kernel_list -> adw::PreferencesGroup {}
                    }
                    
                },
                gtk::Box{
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Start,
                    set_vexpand: true,


                    gtk::Button {
                        set_label: "Add Kernel",
                        set_css_classes: &["suggested-action", "pill"],    
                        set_valign: gtk::Align::Center,

                        connect_clicked => GeneralAppMessages::Add,
                    }
                }
            }
            

        }

    }
}