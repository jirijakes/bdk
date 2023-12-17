use std::convert::Infallible;
use std::fmt::Debug;
use std::str::FromStr;

use bdk::bitcoin::{Network, PrivateKey};
use bdk::template::{Bip86, P2TR};
use bdk::wallet::AddressIndex::New;
use bdk::{KeychainKind, Wallet};
use bdk_chain::PersistBackend;

pub fn main() {
    let key =
        bitcoin::bip32::ExtendedPrivKey::new_master(Network::Signet, &[0xf0, 0x0d, 0xba, 0xbe])
            .unwrap();

    let mut wallet = Wallet::new(
        Bip86(key, KeychainKind::External),
        Some(Bip86(key, KeychainKind::Internal)),
        Whisper,
        Network::Signet,
    )
    .unwrap();

    println!("{:#?}", wallet.keychains());

    println!("{:?}", wallet.get_address(New));
    // println!("{:?}", wallet.get_address(New));
    // println!("{:?}", wallet.get_address(New));
    // println!("{:?}", wallet.get_address(New));

    // assert_eq!(wallet.public_descriptor(KeychainKind::External).unwrap().to_string(), "tr([c55b303f/86'/1'/0']tpubDCiHofpEs47kx358bPdJmTZHmCDqQ8qw32upCSxHrSEdeeBs2T5Mq6QMB2ukeMqhNBiyhosBvJErteVhfURPGXPv3qLJPw5MVpHUewsbP2m/0/*)#dkgvr5hm");

    // println!("{:?}", wallet.get_address(New).to_string());
}

pub struct Whisper;

impl<C: Debug> PersistBackend<C> for Whisper {
    type WriteError = Infallible;

    type LoadError = Infallible;

    fn write_changes(&mut self, changeset: &C) -> Result<(), Self::WriteError> {
        println!("\u{001B}[90m>>> {:#?}\u{001B}[39m", changeset);
        Ok(())
    }

    fn load_from_persistence(&mut self) -> Result<Option<C>, Self::LoadError> {
        Ok(None)
    }
}
