use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;
use iced::{Command, Element};
use std::sync::Arc;

use crate::{
    app::{
        error::Error,
        message::{Message, VaultMessage},
        state::{
            cmd::{
                get_onchain_txs, get_revocation_txs, get_unvault_tx, revault, set_revocation_txs,
                set_unvault_tx,
            },
            sign::{RevocationTransactionsTarget, Signer, UnvaultTransactionTarget},
        },
        view::{
            vault::{
                DelegateVaultView, RevaultVaultView, SecureVaultView, VaultModal,
                VaultOnChainTransactionsPanel, VaultView,
            },
            Context,
        },
    },
    revaultd::{
        model::{self, RevocationTransactions, VaultStatus, VaultTransactions},
        RevaultD,
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

    pub fn update(
        &mut self,
        revaultd: Arc<RevaultD>,
        message: VaultMessage,
    ) -> Command<VaultMessage> {
        match message {
            VaultMessage::ListOnchainTransaction => {
                return Command::perform(
                    get_onchain_txs(revaultd, self.vault.outpoint()),
                    VaultMessage::OnChainTransactions,
                );
            }
            VaultMessage::OnChainTransactions(res) => match res {
                Ok(txs) => self.section = VaultSection::new_onchain_txs_section(txs),
                Err(e) => self.warning = Error::from(e).into(),
            },
            VaultMessage::UnvaultTransaction(res) => match res {
                Ok(tx) => self.section = VaultSection::new_delegate_section(tx.unvault_tx),
                Err(e) => self.warning = Error::from(e).into(),
            },
            VaultMessage::RevocationTransactions(res) => match res {
                Ok(tx) => self.section = VaultSection::new_ack_section(tx),
                Err(e) => self.warning = Error::from(e).into(),
            },
            VaultMessage::SelectRevault => {
                self.section = VaultSection::new_revault_section();
            }
            VaultMessage::SelectDelegate => {
                return Command::perform(
                    get_unvault_tx(revaultd, self.vault.outpoint()),
                    VaultMessage::UnvaultTransaction,
                );
            }
            VaultMessage::SelectSecure => {
                return Command::perform(
                    get_revocation_txs(revaultd, self.vault.outpoint()),
                    VaultMessage::RevocationTransactions,
                );
            }
            _ => {
                return self.section.update(revaultd, &mut self.vault, message);
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

    pub fn load(&self, revaultd: Arc<RevaultD>) -> Command<VaultMessage> {
        Command::perform(
            get_onchain_txs(revaultd, self.vault.outpoint()),
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
    Delegate {
        signer: Signer<UnvaultTransactionTarget>,
        view: DelegateVaultView,
        warning: Option<Error>,
    },
    Secure {
        signer: Signer<RevocationTransactionsTarget>,
        view: SecureVaultView,
        warning: Option<Error>,
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
            Self::Delegate { .. } => "Delegate vault",
            Self::Secure { .. } => "Create vault",
            Self::Revault { .. } => "Revault funds",
        }
    }

    pub fn new_onchain_txs_section(txs: VaultTransactions) -> Self {
        Self::OnchainTransactions {
            txs,
            view: VaultOnChainTransactionsPanel::new(),
        }
    }

    pub fn new_delegate_section(unvault_tx: Psbt) -> Self {
        Self::Delegate {
            signer: Signer::new(UnvaultTransactionTarget { unvault_tx }),
            view: DelegateVaultView::new(),
            warning: None,
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

    pub fn new_ack_section(txs: RevocationTransactions) -> Self {
        Self::Secure {
            signer: Signer::new(RevocationTransactionsTarget {
                emergency_tx: txs.emergency_tx,
                emergency_unvault_tx: txs.emergency_unvault_tx,
                cancel_tx: txs.cancel_tx,
            }),
            view: SecureVaultView::new(),
            warning: None,
        }
    }

    fn update(
        &mut self,
        revaultd: Arc<RevaultD>,
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
                        revault(revaultd, vault.outpoint()),
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
            VaultMessage::Delegate(msg) => {
                if let VaultSection::Delegate {
                    signer, warning, ..
                } = self
                {
                    *warning = None;
                    signer.update(msg);
                    if signer.signed() {
                        return Command::perform(
                            set_unvault_tx(
                                revaultd,
                                vault.outpoint(),
                                signer.target.unvault_tx.clone(),
                            ),
                            VaultMessage::Delegated,
                        );
                    }
                }
            }
            VaultMessage::Delegated(res) => {
                if let Self::Delegate { warning, .. } = self {
                    match res {
                        Ok(()) => {
                            *warning = None;
                        }
                        Err(e) => {
                            *warning = Some(Error::RevaultDError(e));
                        }
                    }
                }
            }
            VaultMessage::Secured(res) => {
                if let VaultSection::Secure { warning, .. } = self {
                    match res {
                        Ok(()) => {
                            *warning = None;
                        }
                        Err(e) => {
                            *warning = Some(Error::RevaultDError(e));
                        }
                    }
                }
            }
            VaultMessage::Secure(msg) => {
                if let VaultSection::Secure {
                    signer, warning, ..
                } = self
                {
                    *warning = None;
                    signer.update(msg);
                    return Command::perform(
                        set_revocation_txs(
                            revaultd,
                            vault.outpoint(),
                            signer.target.emergency_tx.clone(),
                            signer.target.emergency_unvault_tx.clone(),
                            signer.target.cancel_tx.clone(),
                        ),
                        VaultMessage::Secured,
                    );
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
            Self::Delegate {
                signer,
                view,
                warning,
                ..
            } => view.view(ctx, &vault, warning.as_ref(), signer.view(ctx)),
            Self::Secure {
                warning,
                view,
                signer,
            } => view
                .view(
                    ctx,
                    warning.as_ref(),
                    vault,
                    signer.view(ctx).map(VaultMessage::Secure),
                )
                .map(Message::Vault),
            Self::Revault {
                processing,
                success,
                warning,
                view,
            } => view.view(ctx, vault, &processing, &success, warning.as_ref()),
        }
    }
}
