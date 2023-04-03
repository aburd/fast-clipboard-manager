use gtk::prelude::{GtkWindowExt, WidgetExt};
use relm4::{gtk, MessageBroker};
use relm4::{ComponentParts, ComponentSender, SimpleComponent};

static DIALOG_BROKER: MessageBroker<Dialog> = MessageBroker::new();

pub struct Dialog {
    visible: bool,
    message: String,
}

#[derive(Debug)]
pub enum DialogMsg {
    Show(String),
    Hide,
}

#[relm4::component(pub)]
impl SimpleComponent for Dialog {
    type Init = ();
    type Input = DialogMsg;
    type Output = ();

    view! {
        dialog = gtk::Dialog {
            #[watch]
            set_visible: model.visible,
            set_modal: true,

            #[wrap(Some)]
            set_child = &gtk::Label {
                set_width_request: 200,
                set_height_request: 80,
                set_halign: gtk::Align::Center,
                set_valign: gtk::Align::Center,
                #[watch]
                set_label: &model.message,
            },

            connect_close_request[sender] => move |_| {
                sender.input(DialogMsg::Hide);
                gtk::Inhibit(false)
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Dialog {
            visible: false,
            message: "".to_string(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            DialogMsg::Show(message) => {
                self.visible = true;
                self.message = message;
            }
            DialogMsg::Hide => self.visible = false,
        }
    }
}
