use iced::{
    button::State as Button, pick_list, scrollable, text_input, Alignment, Checkbox, Column,
    Container, Element, Length, Row,
};

use revault_ui::{
    component::{
        button, card, form, image::revault_colored_logo, scroll, text::Text,
        ContainerBackgroundStyle,
    },
    icon,
};

use crate::{
    installer::message::{self, Message},
    revault::Role,
};

const NETWORKS: [bitcoin::Network; 4] = [
    bitcoin::Network::Bitcoin,
    bitcoin::Network::Testnet,
    bitcoin::Network::Signet,
    bitcoin::Network::Regtest,
];

pub struct Welcome {
    network_input: pick_list::State<bitcoin::Network>,
    install_button: Button,
}

impl Welcome {
    pub fn new() -> Self {
        Self {
            network_input: pick_list::State::default(),
            install_button: Button::default(),
        }
    }

    pub fn render(&mut self, network: &bitcoin::Network) -> Element<Message> {
        Container::new(Container::new(
            Column::new()
                .push(Container::new(
                    revault_colored_logo()
                        .width(Length::Units(400))
                        .height(Length::Fill),
                ))
                .push(Container::new(
                    pick_list::PickList::new(
                        &mut self.network_input,
                        &NETWORKS[..],
                        Some(*network),
                        message::Message::Network,
                    )
                    .padding(10),
                ))
                .push(
                    button::primary(
                        &mut self.install_button,
                        button::button_content(None, "Install"),
                    )
                    .on_press(Message::Next)
                    .width(Length::Units(200)),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center),
        ))
        .center_y()
        .center_x()
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}

pub struct DefineRole {
    previous_button: Button,
    manager_button: Button,
    stakeholder_button: Button,
    stakeholder_manager_button: Button,
    scroll: scrollable::State,
}

impl DefineRole {
    pub fn new() -> Self {
        Self {
            previous_button: Button::new(),
            manager_button: Button::new(),
            stakeholder_button: Button::new(),
            stakeholder_manager_button: Button::new(),
            scroll: scrollable::State::new(),
        }
    }
    pub fn render(&mut self) -> Element<Message> {
        layout(
            &mut self.scroll,
            &mut self.previous_button,
            Column::new()
                .push(
                    Row::new()
                        .push(
                            button::white_card_button(
                                &mut self.stakeholder_button,
                                button::button_content(None, "Stakeholder"),
                            )
                            .on_press(Message::Role(&Role::STAKEHOLDER_ONLY)),
                        )
                        .push(
                            button::white_card_button(
                                &mut self.stakeholder_manager_button,
                                button::button_content(None, "Stakeholder & Manager"),
                            )
                            .on_press(Message::Role(&Role::STAKEHOLDER_AND_MANAGER)),
                        )
                        .push(
                            button::white_card_button(
                                &mut self.manager_button,
                                button::button_content(None, "Manager"),
                            )
                            .on_press(Message::Role(&Role::MANAGER_ONLY)),
                        )
                        .spacing(20),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center)
                .into(),
        )
    }
}

pub fn participant_xpub<'a>(
    xpub: &form::Value<String>,
    xpub_input: &'a mut text_input::State,
    delete_button: &'a mut Button,
) -> Element<'a, message::ParticipantXpub> {
    Container::new(
        Column::new()
            .push(
                Row::new()
                    .push(
                        form::Form::new(
                            xpub_input,
                            "Xpub",
                            xpub,
                            message::ParticipantXpub::XpubEdited,
                        )
                        .warning("Please enter a valid xpub according to the chosen network")
                        .size(20)
                        .padding(10)
                        .render(),
                    )
                    .push(
                        button::transparent(delete_button, Container::new(icon::trash_icon()))
                            .on_press(message::ParticipantXpub::Delete),
                    )
                    .spacing(5)
                    .align_items(Alignment::Center),
            )
            .spacing(10),
    )
    .into()
}

pub fn required_xpub<'a>(
    xpub: &form::Value<String>,
    xpub_input: &'a mut text_input::State,
) -> Element<'a, String> {
    Container::new(
        Column::new()
            .push(
                form::Form::new(xpub_input, "Xpub", xpub, |msg| msg)
                    .warning("Please enter a valid xpub according to the chosen network")
                    .size(20)
                    .padding(10)
                    .render(),
            )
            .spacing(10),
    )
    .into()
}

pub fn cosigner_key<'a>(
    key: &form::Value<String>,
    key_input: &'a mut text_input::State,
) -> Element<'a, String> {
    Container::new(
        Column::new()
            .push(
                Row::new()
                    .push(
                        form::Form::new(key_input, "Key", key, |msg| msg)
                            .warning("Please enter a valid key")
                            .size(20)
                            .padding(10)
                            .render(),
                    )
                    .spacing(5)
                    .align_items(Alignment::Center),
            )
            .spacing(10),
    )
    .into()
}

pub struct DefinePrivateNoiseKey {
    key_input: text_input::State,
    next_button: Button,
    previous_button: Button,
    scroll: scrollable::State,
}

impl DefinePrivateNoiseKey {
    pub fn new() -> Self {
        Self {
            key_input: text_input::State::new(),
            next_button: Button::new(),
            previous_button: Button::new(),
            scroll: scrollable::State::new(),
        }
    }

    pub fn render<'a>(&'a mut self, key: &form::Value<String>) -> Element<Message> {
        layout(
            &mut self.scroll,
            &mut self.previous_button,
            Column::new()
                .push(Text::new("Fill your private noise key:").bold().size(50))
                .push(
                    Column::new().spacing(10).push(
                        form::Form::new(&mut self.key_input, "", key, Message::PrivateNoiseKey)
                            .warning(
                                "Please enter a 32 bytes noise private key that is hex encoded",
                            )
                            .size(20)
                            .padding(10)
                            .render(),
                    ),
                )
                .push(
                    button::primary(&mut self.next_button, button::button_content(None, "Next"))
                        .on_press(Message::Next)
                        .width(Length::Units(200)),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center)
                .into(),
        )
    }
}

pub struct DefineStakeholderXpubsAsStakeholder {
    our_xpub_input: text_input::State,
    previous_button: Button,
    save_button: Button,
    add_xpub_button: Button,
    scroll: scrollable::State,
}

impl DefineStakeholderXpubsAsStakeholder {
    pub fn new() -> Self {
        Self {
            our_xpub_input: text_input::State::new(),
            add_xpub_button: Button::new(),
            scroll: scrollable::State::new(),
            previous_button: Button::new(),
            save_button: Button::new(),
        }
    }
    pub fn render<'a>(
        &'a mut self,
        our_xpub: &form::Value<String>,
        other_xpubs: Vec<Element<'a, Message>>,
        warning: Option<&String>,
    ) -> Element<'a, Message> {
        let mut content = Column::new()
            .push(Text::new("Stakeholders information").bold().size(50))
            .push(
                Column::new()
                    .push(Text::new("Your stakeholder xpub:").bold())
                    .push(
                        form::Form::new(
                            &mut self.our_xpub_input,
                            "Your stakeholder xpub",
                            our_xpub,
                            |msg| {
                                Message::DefineStakeholderXpubs(
                                    message::DefineStakeholderXpubs::OurXpubEdited(msg),
                                )
                            },
                        )
                        .warning("Please enter a valid xpub")
                        .size(20)
                        .padding(10)
                        .render(),
                    )
                    .spacing(10),
            )
            .push(
                Column::new()
                    .spacing(10)
                    .push(Text::new("Other stakeholders xpubs:").bold())
                    .push(Column::with_children(other_xpubs).spacing(10))
                    .push(
                        Container::new(
                            button::white_card_button(
                                &mut self.add_xpub_button,
                                button::button_content(Some(icon::plus_icon()), "Add stakeholder"),
                            )
                            .on_press(Message::DefineStakeholderXpubs(
                                message::DefineStakeholderXpubs::AddXpub,
                            )),
                        )
                        .width(Length::Fill),
                    ),
            )
            .push(
                Row::new()
                    .push(
                        button::primary(
                            &mut self.save_button,
                            button::button_content(None, "Next"),
                        )
                        .on_press(Message::Next)
                        .width(Length::Units(200)),
                    )
                    .align_items(Alignment::Center)
                    .spacing(20),
            );

        if let Some(error) = warning {
            content = content.push(card::alert_warning(Container::new(Text::new(error))));
        }

        layout(
            &mut self.scroll,
            &mut self.previous_button,
            content
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center)
                .into(),
        )
    }
}

pub fn define_stakeholder_xpubs_as_manager_only<'a>(
    add_xpub_button: &'a mut Button,
    stakeholder_xpubs: Vec<Element<'a, Message>>,
    scroll: &'a mut scrollable::State,
    previous_button: &'a mut Button,
    save_button: &'a mut Button,
    warning: Option<&String>,
) -> Element<'a, Message> {
    let mut row = Row::new().align_items(Alignment::Center).spacing(20);
    if stakeholder_xpubs.is_empty() {
        row = row.push(
            button::primary(save_button, button::button_content(None, "Next"))
                .width(Length::Units(200)),
        );
    } else {
        row = row.push(
            button::primary(save_button, button::button_content(None, "Next"))
                .on_press(Message::Next)
                .width(Length::Units(200)),
        );
    }

    let mut content = Column::new()
        .spacing(10)
        .push(Text::new("Stakeholders xpubs:").bold())
        .push(Column::with_children(stakeholder_xpubs).spacing(10))
        .push(
            Container::new(
                button::white_card_button(
                    add_xpub_button,
                    button::button_content(Some(icon::plus_icon()), "Add stakeholder"),
                )
                .on_press(Message::DefineStakeholderXpubs(
                    message::DefineStakeholderXpubs::AddXpub,
                )),
            )
            .width(Length::Fill),
        );

    if let Some(error) = warning {
        content = content.push(card::alert_warning(Container::new(Text::new(error))));
    }

    layout(
        scroll,
        previous_button,
        Column::new()
            .push(Text::new("Stakeholders information").bold().size(50))
            .push(content)
            .push(row)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .spacing(50)
            .align_items(Alignment::Center)
            .into(),
    )
}

pub struct ManagersThreshold {
    increment_button: Button,
    decrement_button: Button,
}

impl ManagersThreshold {
    pub fn new() -> Self {
        Self {
            increment_button: Button::new(),
            decrement_button: Button::new(),
        }
    }

    pub fn render(&mut self, managers_threshold: &form::Value<usize>) -> Container<Message> {
        let mut col = Column::new()
            .push(Text::new("Managers threshold:").bold())
            .push(
                Row::new()
                    .push(Text::new(&format!("{}", managers_threshold.value)).size(50))
                    .push(
                        Column::new()
                            .push(
                                button::transparent(
                                    &mut self.increment_button,
                                    Container::new(Text::new("+")),
                                )
                                .on_press(
                                    Message::DefineManagerXpubs(
                                        message::DefineManagerXpubs::ManagersThreshold(
                                            message::Action::Increment,
                                        ),
                                    ),
                                ),
                            )
                            .push(
                                button::transparent(
                                    &mut self.decrement_button,
                                    Container::new(Text::new("-")),
                                )
                                .on_press(
                                    Message::DefineManagerXpubs(
                                        message::DefineManagerXpubs::ManagersThreshold(
                                            message::Action::Decrement,
                                        ),
                                    ),
                                ),
                            )
                            .align_items(Alignment::Center),
                    )
                    .align_items(Alignment::Center)
                    .spacing(20),
            )
            .align_items(Alignment::Center)
            .spacing(10);

        if !managers_threshold.valid {
            col = col.push(card::alert_warning(Container::new(
                Text::new("Impossible threshold").small(),
            )))
        }
        Container::new(col)
    }
}

pub struct SpendingDelay {
    increment_button: Button,
    decrement_button: Button,
}

impl SpendingDelay {
    pub fn new() -> Self {
        Self {
            increment_button: Button::new(),
            decrement_button: Button::new(),
        }
    }

    pub fn render(&mut self, spending_delay: &form::Value<u32>) -> Container<Message> {
        let mut col = Column::new()
            .push(Text::new("Spending delay in blocks:").bold())
            .push(
                Row::new()
                    .push(Text::new(&format!("{}", spending_delay.value)).size(50))
                    .push(
                        Column::new()
                            .push(
                                button::transparent(
                                    &mut self.increment_button,
                                    Container::new(Text::new("+")),
                                )
                                .on_press(
                                    Message::DefineManagerXpubs(
                                        message::DefineManagerXpubs::SpendingDelay(
                                            message::Action::Increment,
                                        ),
                                    ),
                                ),
                            )
                            .push(
                                button::transparent(
                                    &mut self.decrement_button,
                                    Container::new(Text::new("-")),
                                )
                                .on_press(
                                    Message::DefineManagerXpubs(
                                        message::DefineManagerXpubs::SpendingDelay(
                                            message::Action::Decrement,
                                        ),
                                    ),
                                ),
                            )
                            .align_items(Alignment::Center),
                    )
                    .align_items(Alignment::Center)
                    .spacing(20),
            )
            .align_items(Alignment::Center)
            .spacing(10);
        if !spending_delay.valid {
            col = col.push(card::alert_warning(Container::new(
                Text::new("Spending delay cannot be equal to zero").small(),
            )))
        }
        Container::new(col)
    }
}

pub struct DefineManagerXpubsAsManager {
    managers_threshold: ManagersThreshold,
    spending_delay: SpendingDelay,
    add_xpub_button: Button,
    our_xpub_input: text_input::State,
    scroll: scrollable::State,
    previous_button: Button,
    save_button: Button,
}

impl DefineManagerXpubsAsManager {
    pub fn new() -> Self {
        Self {
            our_xpub_input: text_input::State::new(),
            add_xpub_button: Button::new(),
            scroll: scrollable::State::new(),
            previous_button: Button::new(),
            save_button: Button::new(),
            spending_delay: SpendingDelay::new(),
            managers_threshold: ManagersThreshold::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render<'a>(
        &'a mut self,
        managers_threshold: &form::Value<usize>,
        spending_delay: &form::Value<u32>,
        our_xpub: &form::Value<String>,
        other_xpubs: Vec<Element<'a, Message>>,
        cosigners: Vec<Element<'a, Message>>,
        cosigners_enabled: bool,
        warning: Option<&String>,
    ) -> Element<'a, Message> {
        let manager_xpub_col = Column::new()
            .push(Text::new("Your manager xpub:").bold())
            .push(
                form::Form::new(
                    &mut self.our_xpub_input,
                    "Your manager xpub",
                    our_xpub,
                    |msg| {
                        Message::DefineManagerXpubs(message::DefineManagerXpubs::OurXpubEdited(msg))
                    },
                )
                .warning("Please enter a valid xpub")
                .size(20)
                .padding(10)
                .render(),
            )
            .spacing(10);

        let mut content = Column::new()
            .push(Text::new("Define managers").bold().size(50))
            .push(
                Row::new()
                    .push(
                        Container::new(self.managers_threshold.render(managers_threshold))
                            .width(Length::FillPortion(1))
                            .center_x(),
                    )
                    .push(
                        Container::new(self.spending_delay.render(spending_delay))
                            .width(Length::FillPortion(1))
                            .center_x(),
                    )
                    .width(Length::Fill),
            )
            .push(manager_xpub_col)
            .push(
                Column::new()
                    .push(Text::new("Other Managers xpubs:").bold())
                    .push(Column::with_children(other_xpubs).spacing(10))
                    .push(
                        Container::new(
                            button::white_card_button(
                                &mut self.add_xpub_button,
                                button::button_content(Some(icon::plus_icon()), "Add manager"),
                            )
                            .on_press(Message::DefineManagerXpubs(
                                message::DefineManagerXpubs::AddXpub,
                            )),
                        )
                        .width(Length::Fill),
                    )
                    .spacing(10),
            )
            .push(
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                Container::new(Text::new("Cosigning servers keys:").bold())
                                    .width(Length::Fill),
                            )
                            .push(Container::new(Checkbox::new(
                                cosigners_enabled,
                                "Enable cosigners",
                                |msg| {
                                    Message::DefineManagerXpubs(
                                        message::DefineManagerXpubs::CosignersEnabled(msg),
                                    )
                                },
                            )))
                            .width(Length::Shrink),
                    )
                    .push(Column::with_children(cosigners).spacing(10))
                    .spacing(10),
            )
            .push(
                Row::new()
                    .push(
                        button::primary(
                            &mut self.save_button,
                            button::button_content(None, "Next"),
                        )
                        .on_press(Message::Next)
                        .width(Length::Units(200)),
                    )
                    .align_items(Alignment::Center)
                    .spacing(20),
            );

        if let Some(error) = warning {
            content = content.push(card::alert_warning(Container::new(Text::new(error))));
        }

        layout(
            &mut self.scroll,
            &mut self.previous_button,
            content
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center)
                .into(),
        )
    }
}

pub struct DefineManagerXpubsAsStakeholderOnly {
    managers_threshold: ManagersThreshold,
    spending_delay: SpendingDelay,
    add_xpub_button: Button,
    scroll: scrollable::State,
    previous_button: Button,
    save_button: Button,
}

impl DefineManagerXpubsAsStakeholderOnly {
    pub fn new() -> Self {
        Self {
            managers_threshold: ManagersThreshold::new(),
            spending_delay: SpendingDelay::new(),
            scroll: scrollable::State::new(),
            previous_button: Button::new(),
            save_button: Button::new(),
            add_xpub_button: Button::new(),
        }
    }
    pub fn render<'a>(
        &'a mut self,
        managers_threshold: &form::Value<usize>,
        spending_delay: &form::Value<u32>,
        manager_xpubs: Vec<Element<'a, Message>>,
        cosigners: Vec<Element<'a, Message>>,
        cosigners_enabled: bool,
        warning: Option<&String>,
    ) -> Element<'a, Message> {
        let mut row = Row::new().align_items(Alignment::Center).spacing(20);
        if manager_xpubs.is_empty() {
            row = row.push(
                button::primary(&mut self.save_button, button::button_content(None, "Next"))
                    .width(Length::Units(200)),
            );
        } else {
            row = row.push(
                button::primary(
                    &mut self.save_button,
                    button::button_content(None, "Next").width(Length::Fill),
                )
                .on_press(Message::Next)
                .width(Length::Units(200)),
            );
        }

        let mut content = Column::new()
            .push(Text::new("Fund management configuration").bold().size(50))
            .push(
                Row::new()
                    .push(
                        Container::new(self.managers_threshold.render(managers_threshold))
                            .width(Length::FillPortion(1))
                            .center_x(),
                    )
                    .push(
                        Container::new(self.spending_delay.render(spending_delay))
                            .width(Length::FillPortion(1))
                            .center_x(),
                    )
                    .width(Length::Fill),
            )
            .push(
                Column::new()
                    .spacing(10)
                    .push(Text::new("Managers xpubs:").bold())
                    .push(Column::with_children(manager_xpubs).spacing(10))
                    .push(
                        Container::new(
                            button::white_card_button(
                                &mut self.add_xpub_button,
                                button::button_content(Some(icon::plus_icon()), "Add manager"),
                            )
                            .on_press(Message::DefineManagerXpubs(
                                message::DefineManagerXpubs::AddXpub,
                            )),
                        )
                        .width(Length::Fill),
                    ),
            )
            .push(
                Column::new()
                    .spacing(10)
                    .push(
                        Row::new()
                            .push(
                                Container::new(Text::new("Cosigning servers keys:").bold())
                                    .width(Length::Fill),
                            )
                            .push(Container::new(Checkbox::new(
                                cosigners_enabled,
                                "Enable cosigners",
                                |msg| {
                                    Message::DefineManagerXpubs(
                                        message::DefineManagerXpubs::CosignersEnabled(msg),
                                    )
                                },
                            )))
                            .width(Length::Shrink),
                    )
                    .push(Column::with_children(cosigners).spacing(10)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .spacing(50)
            .align_items(Alignment::Center);

        if let Some(error) = warning {
            content = content.push(card::alert_warning(Container::new(Text::new(error))));
        }

        layout(
            &mut self.scroll,
            &mut self.previous_button,
            content.push(row).into(),
        )
    }
}

pub struct DefineCpfpDescriptorView {
    scroll: scrollable::State,
    previous_button: Button,
    save_button: Button,
}

impl DefineCpfpDescriptorView {
    pub fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
            previous_button: Button::new(),
            save_button: Button::new(),
        }
    }

    pub fn render<'a>(
        &'a mut self,
        manager_xpubs: Vec<Element<'a, Message>>,
        warning: Option<&String>,
    ) -> Element<'a, Message> {
        let mut row = Row::new().align_items(Alignment::Center).spacing(20);
        if manager_xpubs.is_empty() {
            row = row.push(
                button::primary(&mut self.save_button, button::button_content(None, "Next"))
                    .width(Length::Units(200)),
            );
        } else {
            row = row.push(
                button::primary(&mut self.save_button, button::button_content(None, "Next"))
                    .on_press(Message::Next)
                    .width(Length::Units(200)),
            );
        }

        let mut content = Column::new()
            .spacing(10)
            .push(Text::new("Managers CPFP xpubs:").bold())
            .push(Column::with_children(manager_xpubs).spacing(10));

        if let Some(error) = warning {
            content = content.push(card::alert_warning(Container::new(Text::new(error))));
        }

        layout(
            &mut self.scroll,
            &mut self.previous_button,
            Column::new()
                .push(Text::new("Fill in the managers CPFP xpubs").bold().size(50))
                .push(content)
                .push(row)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center)
                .into(),
        )
    }
}

pub struct DefineCoordinator {
    host_input: text_input::State,
    noise_key_input: text_input::State,
    scroll: scrollable::State,
    previous_button: Button,
    save_button: Button,
}

impl DefineCoordinator {
    pub fn new() -> Self {
        Self {
            host_input: text_input::State::new(),
            noise_key_input: text_input::State::new(),
            scroll: scrollable::State::new(),
            previous_button: Button::new(),
            save_button: Button::new(),
        }
    }
    pub fn render<'a>(
        &'a mut self,
        host: &form::Value<String>,
        noise_key: &form::Value<String>,
    ) -> Element<'a, Message> {
        layout(
            &mut self.scroll,
            &mut self.previous_button,
            Column::new()
                .push(Text::new("Fill in coordinator information").bold().size(50))
                .push(
                    Column::new()
                        .push(Text::new("Host:").bold())
                        .push(
                            form::Form::new(&mut self.host_input, "Host", host, |msg| {
                                Message::DefineCoordinator(message::DefineCoordinator::HostEdited(
                                    msg,
                                ))
                            })
                            .warning("Incorrect format for a socket address")
                            .size(20)
                            .padding(10)
                            .render(),
                        )
                        .spacing(10),
                )
                .push(
                    Column::new()
                        .push(Text::new("Noise key:").bold())
                        .push(
                            form::Form::new(
                                &mut self.noise_key_input,
                                "Noise key",
                                noise_key,
                                |msg| {
                                    Message::DefineCoordinator(
                                        message::DefineCoordinator::NoiseKeyEdited(msg),
                                    )
                                },
                            )
                            .warning("Key must be a 64 characters hex encoded string")
                            .size(20)
                            .padding(10)
                            .render(),
                        )
                        .spacing(10),
                )
                .push(
                    button::primary(&mut self.save_button, button::button_content(None, "Next"))
                        .on_press(Message::Next)
                        .width(Length::Units(200)),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center)
                .into(),
        )
    }
}

pub struct DefineEmergencyAddress {
    address_input: text_input::State,
    scroll: scrollable::State,
    previous_button: Button,
    save_button: Button,
}

impl DefineEmergencyAddress {
    pub fn new() -> Self {
        Self {
            address_input: text_input::State::new(),
            scroll: scrollable::State::new(),
            previous_button: Button::new(),
            save_button: Button::new(),
        }
    }
    pub fn render<'a>(
        &'a mut self,
        address: &form::Value<String>,
        warning: Option<&String>,
    ) -> Element<'a, Message> {
        let mut row = Row::new().align_items(Alignment::Center).spacing(20);
        if !address.valid {
            row = row.push(
                button::primary(&mut self.save_button, button::button_content(None, "Next"))
                    .width(Length::Units(200)),
            );
        } else {
            row = row.push(
                button::primary(&mut self.save_button, button::button_content(None, "Next"))
                    .on_press(Message::Next)
                    .width(Length::Units(200)),
            );
        }
        let mut col = Column::new()
            .push(Text::new("Address:").bold())
            .push(
                form::Form::new(
                    &mut self.address_input,
                    "",
                    address,
                    Message::DefineEmergencyAddress,
                )
                .warning("Please enter a valid address")
                .size(20)
                .padding(10)
                .render(),
            )
            .spacing(10);

        if let Some(error) = warning {
            col = col.push(card::alert_warning(Container::new(Text::new(error))));
        }

        layout(
            &mut self.scroll,
            &mut self.previous_button,
            Column::new()
                .push(Text::new("Fill in emergency information").bold().size(50))
                .push(col)
                .push(row)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center)
                .into(),
        )
    }
}

pub struct Cosigner {
    noise_key_input: text_input::State,
    host_input: text_input::State,
}

impl Cosigner {
    pub fn new() -> Self {
        Self {
            noise_key_input: text_input::State::new(),
            host_input: text_input::State::new(),
        }
    }
    pub fn render(
        &mut self,
        host: &form::Value<String>,
        noise_key: &form::Value<String>,
    ) -> Element<message::DefineCosigner> {
        Container::new(
            Row::new()
                .push(
                    form::Form::new(
                        &mut self.host_input,
                        "Host",
                        host,
                        message::DefineCosigner::HostEdited,
                    )
                    .warning("Please enter a valid host")
                    .size(20)
                    .padding(10)
                    .render(),
                )
                .push(
                    form::Form::new(
                        &mut self.noise_key_input,
                        "Noise key",
                        noise_key,
                        message::DefineCosigner::NoiseKeyEdited,
                    )
                    .warning("Key must be a 64 characters hex encoded string")
                    .size(20)
                    .padding(10)
                    .render(),
                )
                .spacing(5)
                .align_items(Alignment::Center),
        )
        .into()
    }
}

pub struct DefineCosigners {
    scroll: scrollable::State,
    previous_button: Button,
    save_button: Button,
}

impl DefineCosigners {
    pub fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
            previous_button: Button::new(),
            save_button: Button::new(),
        }
    }
    pub fn render<'a>(&'a mut self, cosigners: Vec<Element<'a, Message>>) -> Element<'a, Message> {
        layout(
            &mut self.scroll,
            &mut self.previous_button,
            Column::new()
                .push(
                    Text::new("Fill in cosigning servers information")
                        .bold()
                        .size(50),
                )
                .push(
                    Column::new()
                        .push(
                            Container::new(Text::new("The cosigning servers:").bold())
                                .width(Length::Fill),
                        )
                        .push(Column::with_children(cosigners).spacing(10))
                        .spacing(10),
                )
                .push(
                    button::primary(&mut self.save_button, button::button_content(None, "Next"))
                        .on_press(Message::Next)
                        .width(Length::Units(200)),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center)
                .into(),
        )
    }
}

pub struct DefineBitcoind {
    address_input: text_input::State,
    cookie_path_input: text_input::State,
    scroll: scrollable::State,
    previous_button: Button,
    save_button: Button,
}

impl DefineBitcoind {
    pub fn new() -> Self {
        Self {
            address_input: text_input::State::new(),
            cookie_path_input: text_input::State::new(),
            scroll: scrollable::State::new(),
            previous_button: Button::new(),
            save_button: Button::new(),
        }
    }
    pub fn render<'a>(
        &'a mut self,
        address: &form::Value<String>,
        cookie_path: &form::Value<String>,
    ) -> Element<'a, Message> {
        let col_address = Column::new()
            .push(Text::new("Address:").bold())
            .push(
                form::Form::new(&mut self.address_input, "Address", address, |msg| {
                    Message::DefineBitcoind(message::DefineBitcoind::AddressEdited(msg))
                })
                .warning("Please enter correct address")
                .size(20)
                .padding(10)
                .render(),
            )
            .spacing(10);

        let col_cookie = Column::new()
            .push(Text::new("Cookie path:").bold())
            .push(
                form::Form::new(
                    &mut self.cookie_path_input,
                    "Cookie path",
                    cookie_path,
                    |msg| Message::DefineBitcoind(message::DefineBitcoind::CookiePathEdited(msg)),
                )
                .warning("Please enter correct path")
                .size(20)
                .padding(10)
                .render(),
            )
            .spacing(10);

        layout(
            &mut self.scroll,
            &mut self.previous_button,
            Column::new()
                .push(
                    Text::new("Set up connection to the Bitcoin full node")
                        .bold()
                        .size(50),
                )
                .push(col_address)
                .push(col_cookie)
                .push(
                    button::primary(&mut self.save_button, button::button_content(None, "Next"))
                        .on_press(Message::Next)
                        .width(Length::Units(200)),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(100)
                .spacing(50)
                .align_items(Alignment::Center)
                .into(),
        )
    }
}

pub struct Final {
    scroll: scrollable::State,
    previous_button: Button,
    action_button: Button,
}

impl Final {
    pub fn new() -> Self {
        Self {
            scroll: scrollable::State::new(),
            previous_button: Button::new(),
            action_button: Button::new(),
        }
    }

    pub fn render(
        &mut self,
        generating: bool,
        config_path: Option<&std::path::PathBuf>,
        warning: Option<&String>,
    ) -> Element<Message> {
        let mut col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .spacing(50)
            .align_items(Alignment::Center);

        if let Some(error) = warning {
            col = col.push(card::alert_warning(Container::new(Text::new(error))));
        }

        if generating {
            col = col.push(
                button::primary(
                    &mut self.action_button,
                    button::button_content(None, "Installing ..."),
                )
                .width(Length::Units(200)),
            )
        } else if let Some(path) = config_path {
            col = col.push(card::border_success(
                Container::new(
                    Column::new()
                        .push(Container::new(Text::new("Installed !")))
                        .push(Container::new(
                            button::primary(
                                &mut self.action_button,
                                button::button_content(None, "Start"),
                            )
                            .on_press(Message::Exit(path.clone()))
                            .width(Length::Units(200)),
                        ))
                        .align_items(Alignment::Center)
                        .spacing(20),
                )
                .padding(50)
                .width(Length::Fill)
                .center_x(),
            ));
        } else {
            col = col.push(
                button::primary(
                    &mut self.action_button,
                    button::button_content(None, "Finalize installation"),
                )
                .on_press(Message::Install)
                .width(Length::Units(200)),
            );
        }

        layout(&mut self.scroll, &mut self.previous_button, col.into())
    }
}

fn layout<'a>(
    scroll_state: &'a mut scrollable::State,
    previous_button: &'a mut Button,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    Container::new(scroll(
        scroll_state,
        Container::new(
            Column::new()
                .push(
                    button::transparent(
                        previous_button,
                        button::button_content(None, "< Previous"),
                    )
                    .on_press(Message::Previous),
                )
                .push(Container::new(content).width(Length::Fill).center_x()),
        ),
    ))
    .style(ContainerBackgroundStyle)
    .center_x()
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}
