use std::path::Path;

use relm4::component::*;
use relm4::prelude::*;
use adw::prelude::*;

#[derive(Debug)]
pub struct CreatorDialog{
    file_path: String,
    is_active:bool,
}

#[derive(Debug)]
pub enum CreatorDialogMessages{
    SetPath(String),
    Show,
    CloseDialog,
}

#[derive(Debug)]
pub enum CreatorDialogMessagesOutput{
    CloseWithPath(String),
}


#[relm4::component(pub)]
impl SimpleComponent for CreatorDialog{
    type Init = (); 
    type Input = CreatorDialogMessages;
    type Output = CreatorDialogMessagesOutput;
    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self>{

        let model = Self {
            file_path: "".to_string(),
            is_active:false,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }


    view!{
        window = adw::Window{
            #[watch]
            set_visible: model.is_active,
            set_modal: true,
            set_title: Some("Kernel Manager"),
            set_default_size: (500, 150),
            set_vexpand: true,

            connect_close_request[sender] => move |_| {
                sender.input(CreatorDialogMessages::CloseDialog);
                gtk::Inhibit(true)
            },
            gtk::Box{
                set_halign:gtk::Align::Center,
                adw::PreferencesGroup{
                    set_title: "Kernel",
                    gtk::Box{
                        // set_valign: gtk::Align::Center,
                        set_orientation: gtk::Orientation::Vertical,
                        set_halign:gtk::Align::Center,
                        set_spacing: 32,
                        adw::EntryRow{
                            // set_focused: false,
                            set_title: "File Path",
                            set_text: &model.file_path,
                            connect_changed: move |row|{
                                sender.clone().input(CreatorDialogMessages::SetPath(row.text().to_string()));
                            }
                        },
                        gtk::Box{
                            gtk::Button{
                                set_label: "Save",
                                // set_icon_name: "emblem-system-symbolic",
                                #[watch]
                                set_sensitive: match Path::new(&model.file_path).exists(){
                                    false => false,
                                    true => true
                                },
                
                                add_css_class: "flat",
                                set_valign: gtk::Align::Center,
                                connect_clicked[sender] => move |_|{
                                    sender.input(CreatorDialogMessages::CloseDialog);
                                }
                            },
                        }
                        
                    }
                }
            },
            
            
        }
    }
    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>){
        match msg{
            CreatorDialogMessages::SetPath(path) =>{
                self.file_path=path;
            }
            CreatorDialogMessages::CloseDialog => {
                self.is_active=false;
                sender.output(CreatorDialogMessagesOutput::CloseWithPath(self.file_path.clone())).unwrap();
            }
            CreatorDialogMessages::Show => {
                self.is_active= true;
            }
        }
    }
}