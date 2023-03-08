use super::builder;
use adw::prelude::*;
use builder::kernelbuilder::Builder;
use builder::kernelbuilder::BuilderMsg;
use builder::kernelbuilder::BuilderMsgOutput;
use builder::lib::kernel::*;
use relm4::component::*;
use relm4::factory::AsyncFactoryVecDeque;
use relm4::factory::*;
use relm4::prelude::*;
use std::rc::Rc;

static mut BUILDER_DIALOG: Option<Rc<Controller<Builder>>> = None;

#[derive(Debug, Clone)]
pub enum GeneralAppMessages {
    Add,
    Remove(DynamicIndex),
    OpenBuilder(DynamicIndex),
    ChildFailedBuild,
    ChildSuccessBuild,
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
                #[watch]
                set_icon_name: match self.installed{
                    false => "document-save-symbolic",
                    true => "user-trash-symbolic"
                },
                add_css_class: "flat",
                set_valign: gtk::Align::Center,
                connect_clicked[sender,index] => move |_|{
                    sender.input(GeneralAppMessages::Remove(index.clone()));
                }
            },

            add_suffix = &gtk::Button{
                set_icon_name: "emblem-system-symbolic",
                #[watch]
                set_sensitive: match self.installed{
                    false => false,
                    true => true
                },

                add_css_class: "flat",
                set_valign: gtk::Align::Center,
                connect_clicked[sender,index] => move |_|{

                    sender.input(GeneralAppMessages::OpenBuilder(index.clone()));
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
    kernel_vec: Vec<Kernel>,
    kernel_list: AsyncFactoryVecDeque<KernelListComponent>,
    // builder_dialog: Controller<Builder>,
}

#[relm4::component(pub)]
impl SimpleComponent for GeneralApp {
    type Init = Vec<Kernel>;
    type Input = GeneralAppMessages;
    type Output = ();

    view! {
        window = adw::Window{
            set_title: Some("Kernel Manager"),
            set_default_size: (700, 560),
            set_vexpand: true,

            gtk::Box{
                set_valign: gtk::Align::Center,
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
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_vexpand: true,
    
                    gtk::Button {
                        set_label: "Add dummy kernel!",
                        set_css_classes: &["suggested-action", "pill"],
    
                        set_valign: gtk::Align::Center,
    
                        connect_clicked => GeneralAppMessages::Add
                    }
                }
            },
            
        }

    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = Self {
            kernel_list: AsyncFactoryVecDeque::new(
                adw::PreferencesGroup::new(),
                sender.input_sender(),
            ),
            kernel_vec: _init.clone(),
        };

        model.kernel_vec.iter().for_each(|object| {
            model
                .kernel_list
                .guard()
                .push_back((object.version.clone(), false));
        });

        unsafe {
            BUILDER_DIALOG = Some(Rc::new(
                Builder::builder()
                    .transient_for(root)
                    .launch((_init.clone(), 0))
                    .forward(sender.input_sender(), convert_alert_response),
            ));
        }
        let kernel_list = model.kernel_list.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            GeneralAppMessages::Add => {
                unsafe{
                    BUILDER_DIALOG.clone().unwrap().emit(BuilderMsg::Add("dummy".to_string(),false));                    
                }
                self.kernel_list
                    .guard()
                    .push_back(("dummy".to_string(), false));
            }
            GeneralAppMessages::Remove(index) => {
                if self
                    .kernel_list
                    .get(index.current_index())
                    .unwrap()
                    .installed
                {
                    // self.kernel_list.guard().remove(index.current_index());
                    self.kernel_list
                        .guard()
                        .get_mut(index.current_index())
                        .unwrap()
                        .installed = false;
                    println!("Unimplemented");
                } else {
                    self.kernel_list
                        .guard()
                        .get_mut(index.current_index())
                        .unwrap()
                        .installed = true;
                    println!("Unimplemented");
                }
            }

            GeneralAppMessages::OpenBuilder(index) => unsafe {
                BUILDER_DIALOG
                    .clone()
                    .unwrap()
                    .emit(BuilderMsg::Show(index));
            },

            GeneralAppMessages::ChildFailedBuild => {
                println!("Failed to build");
            }

            GeneralAppMessages::ChildSuccessBuild => {
                println!("Build success");
            }
        }
    }
}

fn convert_alert_response(response: BuilderMsgOutput) -> GeneralAppMessages {
    match response {
        BuilderMsgOutput::FailedBuild => GeneralAppMessages::ChildFailedBuild,
        BuilderMsgOutput::SuccessFullBuild => GeneralAppMessages::ChildSuccessBuild,
    }
}
