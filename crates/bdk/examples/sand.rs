use std::convert::Infallible;
use std::fmt::Debug;
use std::str::FromStr;

use bdk::bitcoin::Network::*;
use bdk::template::{Bip86, P2TR};
use bdk::wallet::AddressIndex::New;
use bdk::{KeychainKind::*, Wallet};
use bdk_chain::{BlockId, ConfirmationTime, PersistBackend};
use bitcoin::consensus::Encodable;
use bitcoin::hashes::Hash;
use bitcoin::{Address, BlockHash, OutPoint, Transaction, TxIn, TxOut, Txid};

pub fn main() {
    // let key =
    //     bitcoin::bip32::ExtendedPrivKey::new_master(Network::Signet, &[0xf0, 0x0d, 0xba, 0xbe])
    //         .unwrap();

    // let mut wallet = Wallet::new(
    //     Bip86(key, External),
    //     Some(Bip86(key, Internal)),
    //     Whisper,
    //     Network::Signet,
    // )
    // .unwrap();

    let (mut w, tx) = mk_wallet();

    println!("{:?}", w.get_address(New));

    println!("{:?}", tx.output[0].script_pubkey.is_v0_p2wpkh());

    let mut t = vec![];
    let _ = tx.consensus_encode(&mut t);

    println!("{}", hex::encode(&t));
}

#[rustfmt::skip]
fn mk_wallet() -> (Wallet<Whisper>, Transaction) {
    let mut wallet = Wallet::new(
        "wpkh(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW)",
        None,
        Whisper,
        Regtest,
    )
    .unwrap();

    let change_address = wallet.get_address(New).address;
    let sendto_address = Address::from_str("bcrt1q3qtze4ys45tgdvguj66zrk4fu6hq3a3v9pfly5")
        .expect("address")
        .require_network(Regtest)
        .unwrap();

    let tx0 = Transaction {
        version: 1,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint { txid: Txid::all_zeros(), vout: 0 },
            script_sig: Default::default(),
            sequence: Default::default(),
            witness: Default::default(),
        }],
        output: vec![
            TxOut { value: 76_000, script_pubkey: change_address.script_pubkey() }
        ],
    };

    let tx1 = Transaction {
        version: 1,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint { txid: tx0.txid(), vout: 0 },
            script_sig: Default::default(),
            sequence: Default::default(),
            witness: Default::default(),
        }],
        output: vec![
            TxOut { value: 50_000, script_pubkey: change_address.script_pubkey() },
            TxOut { value: 25_000, script_pubkey: sendto_address.script_pubkey() },
        ],
    };

    wallet.insert_checkpoint(BlockId {height: 1_000, hash: BlockHash::all_zeros(),}).unwrap();
    wallet.insert_checkpoint(BlockId {height: 2_000, hash: BlockHash::all_zeros(),}).unwrap();

    wallet.insert_tx(tx0, ConfirmationTime::Confirmed {height: 1_000, time: 100,},).unwrap();
    wallet.insert_tx(tx1.clone(), ConfirmationTime::Confirmed {height: 2_000, time: 200,},).unwrap();

    (wallet, tx1)
}

pub struct Whisper;

impl<C: Debug> PersistBackend<C> for Whisper {
    type WriteError = Infallible;

    type LoadError = Infallible;

    fn write_changes(&mut self, changeset: &C) -> Result<(), Self::WriteError> {
        println!("\u{001B}[90m{:#?}\u{001B}[39m", changeset);
        Ok(())
    }

    fn load_from_persistence(&mut self) -> Result<Option<C>, Self::LoadError> {
        Ok(None)
    }
}
