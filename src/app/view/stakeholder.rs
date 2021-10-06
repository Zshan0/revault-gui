use iced::{
    scrollable,
    tooltip::{self, Tooltip},
    Align, Column, Container, Element, Length, QRCode, Row,
};

use revault_ui::{
    component::{
        button, card, scroll, separation, text::Text, ContainerBackgroundStyle, TooltipStyle,
    },
    icon,
};

use crate::{
    app::{context::Context, error::Error, menu::Menu, message::Message},
    daemon::client::Client,
};

#[derive(Debug)]
pub struct StakeholderCreateVaultsView {
    scroll: scrollable::State,
    qr_code: Option<iced::qr_code::State>,
    close_button: iced::button::State,
    copy_button: iced::button::State,
}

impl StakeholderCreateVaultsView {
    pub fn new() -> Self {
        StakeholderCreateVaultsView {
            qr_code: None,
            scroll: scrollable::State::new(),
            close_button: iced::button::State::new(),
            copy_button: iced::button::State::new(),
        }
    }

    // Address is loaded directly in the view in order to cache the created qrcode.
    pub fn load(&mut self, address: &bitcoin::Address) {
        self.qr_code = iced::qr_code::State::new(address.to_string()).ok();
    }

    pub fn view<'a, C: Client>(
        &'a mut self,
        _ctx: &Context<C>,
        deposits: Vec<Element<'a, Message>>,
        address: Option<&bitcoin::Address>,
    ) -> Element<'a, Message> {
        let mut content = Column::new()
            .max_width(1000)
            .push(Text::new("Create some vaults").bold().size(50))
            .spacing(20);

        if !deposits.is_empty() {
            content = content.push(Container::new(
                Column::new()
                    .push(Text::new(" Click on a deposit to create a vault:"))
                    .push(Column::with_children(deposits).spacing(5))
                    .spacing(20),
            ))
        } else {
            content = content.push(Container::new(Text::new("No deposits")).padding(5))
        }

        if let Some(qr_code) = self.qr_code.as_mut() {
            if let Some(addr) = address {
                content = content.push(separation().width(Length::Fill)).push(
                    card::white(Container::new(
                        Row::new()
                            .push(
                                Column::new()
                                    .push(Text::new(
                                        "Bitcoin deposits are needed in order to create a vault\n",
                                    ))
                                    .push(
                                        Column::new()
                                            .push(
                                                Text::new("Please, use this deposit address:")
                                                    .bold(),
                                            )
                                            .push(
                                                Row::new()
                                                    .push(Container::new(
                                                        Text::new(&addr.to_string()).bold().small(),
                                                    ))
                                                    .push(
                                                        button::clipboard(
                                                            &mut self.copy_button,
                                                            Message::Clipboard(addr.to_string()),
                                                        )
                                                        .width(Length::Shrink),
                                                    )
                                                    .align_items(Align::Center),
                                            ),
                                    )
                                    .spacing(30)
                                    .width(Length::Fill),
                            )
                            .push(
                                Container::new(QRCode::new(qr_code).cell_size(5))
                                    .width(Length::Shrink),
                            )
                            .spacing(10),
                    ))
                    .width(Length::Fill),
                );
            }
        }

        let col = Column::new()
            .push(
                Row::new().push(Container::new(
                            Tooltip::new(
                                Row::new()
                                    .push(icon::tooltip_icon().size(20))
                                    .push(Text::new(" Help")),
                                "A vault is a deposit with revocation transactions\nsigned and shared between stakeholders",
                                tooltip::Position::Right,
                            )
                            .gap(5)
                            .size(20)
                            .padding(10)
                            .style(TooltipStyle),
                        )
                        .width(Length::Fill)).push(
                    Container::new(
                        button::close_button(
                            &mut self.close_button,
                        )
                        .on_press(Message::Menu(Menu::Home)),
                    )
                    .width(Length::Shrink),
                ),
            ).push(content).align_items(Align::Center).spacing(50);

        Container::new(scroll(&mut self.scroll, Container::new(col)))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(ContainerBackgroundStyle)
            .padding(20)
            .into()
    }
}

#[derive(Debug)]
pub struct StakeholderDelegateFundsView {
    scroll: scrollable::State,
    close_button: iced::button::State,
}

impl StakeholderDelegateFundsView {
    pub fn new() -> Self {
        StakeholderDelegateFundsView {
            scroll: scrollable::State::new(),
            close_button: iced::button::State::default(),
        }
    }

    pub fn view<'a, C: Client>(
        &'a mut self,
        ctx: &Context<C>,
        active_balance: &u64,
        activating_balance: &u64,
        vaults: Vec<Element<'a, Message>>,
        warning: Option<&Error>,
    ) -> Element<'a, Message> {
        let mut col = Column::new();
        if let Some(error) = warning {
            col = col.push(card::alert_warning(Container::new(Text::new(&format!(
                "{}",
                error
            )))))
        }

        col = col
            .push(
                Column::new()
                    .push(
                        Text::new("Delegate funds to your manager team")
                            .bold()
                            .size(50),
                    )
                    .spacing(5),
            )
            .push(
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                Text::new(&ctx.converter.converts(*active_balance).to_string())
                                    .bold()
                                    .size(30),
                            )
                            .push(Text::new(&format!(
                                " {} are allocated to managers",
                                ctx.converter.unit
                            )))
                            .align_items(Align::Center),
                    )
                    .push(
                        Row::new()
                            .push(
                                Text::new(&format!(
                                    "+ {}",
                                    ctx.converter.converts(*activating_balance)
                                ))
                                .bold()
                                .size(20),
                            )
                            .push(Text::new(&format!(
                                " {} are waiting for other stakeholders' approval",
                                ctx.converter.unit
                            )))
                            .align_items(Align::Center),
                    ),
            );
        if !vaults.is_empty() {
            col = col.push(Container::new(
                Column::new()
                    .push(Text::new(" Click on the vaults to delegate:"))
                    .push(Column::with_children(vaults).spacing(5))
                    .spacing(20),
            ))
        } else {
            col = col.push(
                Container::new(Text::new("No more funds can be delegated to managers")).padding(5),
            )
        }

        let modal = Column::new()
            .push(
                Row::new()
                    .push(
                        Container::new(
                            Tooltip::new(
                                Row::new()
                                    .push(icon::tooltip_icon().size(20))
                                    .push(Text::new(" Help").small()),
                                "By delegating you allow managers to spend the funds,\n but you can still revert any undesired transaction.",
                                tooltip::Position::Right,
                            )
                            .gap(5)
                            .size(20)
                            .padding(10)
                            .style(TooltipStyle),
                        )
                        .width(Length::Fill),
                    )
                    .push(
                        Container::new(
                            button::close_button(
                                &mut self.close_button,
                            )
                            .on_press(Message::Menu(Menu::Home)),
                        )
                        .width(Length::Shrink),
                    )
                    .align_items(Align::Center),
            )
            .push(
                Container::new(col.spacing(30).max_width(1000))
                    .width(Length::Fill)
                    .align_x(Align::Center),
            )
            .spacing(50);

        Container::new(scroll(&mut self.scroll, Container::new(modal)))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(ContainerBackgroundStyle)
            .padding(20)
            .into()
    }
}
