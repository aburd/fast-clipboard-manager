use crate::components::{Dialog, DialogMsg};

use fast_clipboard::entry::Entry;

use jsonrpsee::{
    core::client::{Client, ClientT},
    rpc_params,
};

use relm4::{
    factory::FactoryVecDeque,
    gtk::{
        self,
        glib::{clone, MainContext},
        prelude::*,
    },
    prelude::*,
    ComponentParts, ComponentSender, MessageBroker, RelmApp, RelmWidgetExt, SimpleComponent,
};

use log::info;

static DIALOG_BROKER: MessageBroker<Dialog> = MessageBroker::new();

const APPLICATION_ID: &str = "com.github.aburd.fast-clipboard-manager";

#[derive(Debug)]
struct Task {
    entry: Entry,
}

#[derive(Debug)]
enum TaskInput {
    Selected,
}

#[derive(Debug)]
enum TaskOutput {
    Delete(DynamicIndex),
}

#[relm4::factory]
impl FactoryComponent for Task {
    type Init = Entry;
    type Input = TaskInput;
    type Output = TaskOutput;
    type CommandOutput = ();
    type ParentInput = AppMsg;
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,

            #[name(label)]
            gtk::Label {
                set_label: &self.entry.to_string(),
                set_hexpand: true,
                set_halign: gtk::Align::Start,
                set_margin_all: 12,
            },

            gtk::Button {
                set_icon_name: "edit-delete",
                set_margin_all: 12,

                connect_clicked[sender, index] => move |_| {
                    sender.output(TaskOutput::Delete(index.clone()));
                }
            }
        }
    }

    // fn pre_view() {
    //     let attrs = widgets.label.attributes().unwrap_or_default();
    //     attrs.change(gtk::pango::AttrInt::new_strikethrough(self.completed));
    //     widgets.label.set_attributes(Some(&attrs));
    // }

    fn output_to_parent_input(output: Self::Output) -> Option<AppMsg> {
        Some(match output {
            TaskOutput::Delete(index) => AppMsg::DeleteEntry(index),
        })
    }

    fn init_model(entry: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { entry }
    }
}

struct AppInit {
    client: Result<Client, AppErr>,
}

#[derive(Debug)]
pub enum AppErr {
    CantConnectToDaemon(String),
}

#[derive(Debug)]
enum AppMsg {
    DeleteEntry(DynamicIndex),
    AddEntry(Entry),
    Error(AppErr),
    Noop,
}

struct App {
    tasks: FactoryVecDeque<Task>,
    dialog: Controller<Dialog>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = AppInit;
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_width_request: 360,
            set_title: Some("To-Do"),

            gtk::Label {},

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 12,
                set_spacing: 6,

                gtk::Entry {
                    connect_activate[sender] => move |entry| {
                        // let buffer = entry.buffer();
                        // sender.input(AppMsg::AddEntry(buffer.text()));
                        // buffer.delete_text(0, None);
                    }
                },

                gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_min_content_height: 360,
                    set_vexpand: true,

                    #[local_ref]
                    task_list_box -> gtk::ListBox {}
                }
            }

        }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::DeleteEntry(index) => {
                self.tasks.guard().remove(index.current_index());
            }
            AppMsg::AddEntry(entry) => {
                self.tasks.guard().push_back(entry);
            }
            AppMsg::Error(e) => {
                let msg = format!("{:?}", e);
                DIALOG_BROKER.send(DialogMsg::Show(msg));
            }
            AppMsg::Noop => {}
        }
    }
    fn init(
        app_init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let dialog = Dialog::builder()
            .transient_for(root)
            .launch_with_broker((), &DIALOG_BROKER)
            .forward(sender.input_sender(), |()| AppMsg::Noop);
        let model = App {
            dialog,
            tasks: FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender()),
        };
        let task_list_box = model.tasks.widget();
        let widgets = view_output!();

        match app_init.client {
            Ok(client) => {
                let sender_clone = sender.clone();
                let main_context = MainContext::default();
                main_context.spawn(async move {
                    let s: String = client.request("get_entries", rpc_params!()).await.unwrap();
                    let entries: Vec<Entry> = serde_json::from_str(&s).unwrap();
                    for entry in entries {
                        sender_clone.input(AppMsg::AddEntry(entry));
                    }
                });
            }
            Err(e) => {
                sender.input(AppMsg::Error(e));
            }
        }

        ComponentParts { model, widgets }
    }
}

pub async fn run_app(client: Result<Client, AppErr>) {
    info!("connecting client");
    let app_init = AppInit { client };
    let app = RelmApp::new(APPLICATION_ID);
    app.run::<App>(app_init);
}
