use relm4::factory::FactoryView;
use relm4::gtk::traits::{ListBoxRowExt, OrientableExt, WidgetExt};
use relm4::gtk::{self};
use relm4::prelude::{DynamicIndex, FactoryComponent};
use relm4::{adw, Component, ComponentController, Controller, FactorySender, RelmWidgetExt};
use relm4_components::simple_combo_box::SimpleComboBox;
use relm4_icons::icon_name;

use super::profile::ProfileInput;

#[derive(Debug)]
pub struct ProfileItemFan {
    fan_idx: u8,
    combo_box: Controller<SimpleComboBox<String>>,
}

pub struct ProfileItemFanInit {
    pub fan_idx: u8,
    pub fan_profiles: Vec<String>,
    pub index: usize,
}

#[relm4::factory(pub)]
impl FactoryComponent for ProfileItemFan {
    type CommandOutput = ();
    type Init = ProfileItemFanInit;
    type Input = ();
    type Output = u8;
    type ParentInput = ProfileInput;
    type ParentWidget = adw::ExpanderRow;

    view! {
        #[root]
        gtk::ListBoxRow {
            set_activatable: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_all: 6,

                gtk::Image {
                    set_icon_name: Some(icon_name::SPEEDOMETER),
                    set_margin_all: 6,
                },

                gtk::Label {
                    set_label: &format!("Fan {}", self.fan_idx + 1),
                    set_margin_all: 6,
                },
                gtk::Box {
                    set_hexpand: true,
                },
                #[local_ref]
                fan_box -> gtk::ComboBoxText {},
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let ProfileItemFanInit {
            fan_idx,
            fan_profiles,
            index,
        } = init;
        let combo_box = SimpleComboBox::builder()
            .launch(SimpleComboBox {
                variants: fan_profiles,
                active_index: Some(index),
            })
            .forward(sender.output_sender(), |output| output as u8);
        Self { fan_idx, combo_box }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let fan_box = self.combo_box.widget();
        let widgets = view_output!();
        widgets
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        Some(ProfileInput::UpdateProfile)
    }
}

impl ProfileItemFan {
    pub fn get_profile_name(&self) -> String {
        self.combo_box
            .state()
            .get()
            .model
            .get_active_elem()
            .unwrap()
            .clone()
    }
}
