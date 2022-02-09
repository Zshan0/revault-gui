use iced::{Command, Element};
use std::sync::Arc;

use crate::{
    app::{
        context::Context,
        error::Error,
        message::{Message, VaultMessage},
        state::cmd::{get_onchain_txs, revault},
        view::vault::{RevaultVaultView, VaultModal, VaultOnChainTransactionsPanel, VaultView},
    },
    daemon::{
        model::{self, outpoint, VaultStatus, VaultTransactions},
        Daemon,
    },
};

#[derive(Debug)]
pub struct VaultListItem<T> {
    pub vault: model::Vault,
    view: T,
}

impl<T: VaultView> VaultListItem<T> {
    pub fn new(vault: model::Vault) -> Self {
        Self {
            vault,
            view: T::new(),
        }
    }

    pub fn view(&mut self, ctx: &Context) -> Element<Message> {
        self.view.view(ctx, &self.vault)
    }
}

/// SelectedVault is a widget displaying information of a vault
/// and handling user action on it.
#[derive(Debug)]
pub struct Vault {
    pub vault: model::Vault,
    warning: Option<Error>,
    section: VaultSection,
    view: VaultModal,
}

impl Vault {
    pub fn new(vault: model::Vault) -> Self {
        Self {
            vault,
            section: VaultSection::Unloaded,
            view: VaultModal::new(),
            warning: None,
        }
    }

    pub fn update(&mut self, ctx: &Context, message: VaultMessage) -> Command<VaultMessage> {
        match message {
            VaultMessage::ListOnchainTransaction => {
                return Command::perform(
                    get_onchain_txs(ctx.revaultd.clone(), outpoint(&self.vault)),
                    VaultMessage::OnChainTransactions,
                );
            }
            VaultMessage::OnChainTransactions(res) => match res {
                Ok(txs) => self.section = VaultSection::new_onchain_txs_section(txs),
                Err(e) => self.warning = Error::from(e).into(),
            },
            VaultMessage::SelectRevault => {
                self.section = VaultSection::new_revault_section();
            }
            _ => {
                return self
                    .section
                    .update(ctx.revaultd.clone(), &mut self.vault, message);
            }
        };
        Command::none()
    }

    pub fn view(&mut self, ctx: &Context) -> Element<Message> {
        self.view.view(
            ctx,
            &self.vault,
            self.warning.as_ref(),
            self.section.title(&self.vault),
            self.section.view(ctx, &self.vault),
        )
    }

    pub fn load(&self, revaultd: Arc<dyn Daemon + Send + Sync>) -> Command<VaultMessage> {
        Command::perform(
            get_onchain_txs(revaultd, outpoint(&self.vault)),
            VaultMessage::OnChainTransactions,
        )
    }
}

#[derive(Debug)]
pub enum VaultSection {
    Unloaded,
    OnchainTransactions {
        txs: VaultTransactions,
        view: VaultOnChainTransactionsPanel,
    },
    /// Revault action ask the user if the vault that is unvaulting
    /// should be revaulted and executes the revault command after
    /// confirmation from the user.
    Revault {
        processing: bool,
        success: bool,
        warning: Option<Error>,
        view: RevaultVaultView,
    },
}

impl VaultSection {
    pub fn title(&self, vault: &model::Vault) -> &'static str {
        match self {
            Self::Unloaded => "",
            Self::OnchainTransactions { .. } => match vault.status {
                VaultStatus::Funded | VaultStatus::Unconfirmed => "Deposit details",
                _ => "Vault details",
            },
            Self::Revault { .. } => "Revault funds",
        }
    }

    pub fn new_onchain_txs_section(txs: VaultTransactions) -> Self {
        Self::OnchainTransactions {
            txs,
            view: VaultOnChainTransactionsPanel::new(),
        }
    }

    pub fn new_revault_section() -> Self {
        Self::Revault {
            processing: false,
            success: false,
            view: RevaultVaultView::new(),
            warning: None,
        }
    }

    fn update(
        &mut self,
        revaultd: Arc<dyn Daemon + Send + Sync>,
        vault: &mut model::Vault,
        message: VaultMessage,
    ) -> Command<VaultMessage> {
        match message {
            VaultMessage::Revault => {
                if let Self::Revault {
                    processing,
                    warning,
                    ..
                } = self
                {
                    *processing = true;
                    *warning = None;
                    return Command::perform(
                        revault(revaultd.clone(), outpoint(vault)),
                        VaultMessage::Revaulted,
                    );
                }
            }
            VaultMessage::Revaulted(res) => {
                if let Self::Revault {
                    processing,
                    success,
                    warning,
                    ..
                } = self
                {
                    *processing = false;
                    match res {
                        Ok(()) => {
                            *success = true;
                            *warning = None;
                            vault.status = VaultStatus::Canceling;
                        }
                        Err(e) => *warning = Error::from(e).into(),
                    }
                }
            }
            _ => {}
        };
        Command::none()
    }

    pub fn view(&mut self, ctx: &Context, vault: &model::Vault) -> Element<Message> {
        match self {
            Self::Unloaded => iced::Container::new(iced::Column::new()).into(),
            Self::OnchainTransactions { txs, view } => view.view(ctx, &vault, &txs),
            Self::Revault {
                processing,
                success,
                warning,
                view,
            } => view.view(ctx, vault, &processing, &success, warning.as_ref()),
        }
    }
}
