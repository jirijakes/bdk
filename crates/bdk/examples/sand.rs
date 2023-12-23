use std::convert::Infallible;
use std::fmt::Debug;
use std::str::FromStr;

use bdk::bitcoin::Network::*;
use bdk::wallet::AddressIndex::New;
use bdk::Wallet;
use bdk_chain::{BlockId, ConfirmationTime, PersistBackend};
use bitcoin::hashes::Hash;
use bitcoin::{Address, BlockHash, OutPoint, Transaction, TxIn, TxOut, Txid};

pub fn main() {
    let mut wallet = mk_wallet();

    let addr = Address::from_str("tb1qcweafyulu7samj8qa3cdnnr6axtp6c6wpdt985")
        .unwrap()
        .assume_checked();

    let mut builder = wallet.build_tx();
    builder.add_recipient(addr, 10000);
    let mut psbt = builder.finish().unwrap();

    println!("{:#?}", psbt);

    // let x = wallet.sign(&mut psbt, Default::default());
    // println!("{:?}", x);
}

#[rustfmt::skip]
fn mk_wallet() -> Wallet<()> {
    let mut wallet = Wallet::new(
        // bitcoin::bip32::ExtendedPrivKey::new_master(Signet, &[0]).unwrap()
        "wpkh(tprv8ZgxMBicQKsPexv3jfDTHG3s59Gh6VwRcgpFzaDG125BnAFyU2MU1h2NbNFqgkgyTds9jy1kxA4tZYcoVh5M1rbLR2aWeUto9iX99XsBmT9/*)",
        None,
        (),
        Signet,
    )
    .unwrap();

    let change_address = wallet.get_address(New).address;
    let sendto_address = Address::from_str("tb1qk2clpn6qht0d5vu4ddqtxw9ddx2jlartgxtync")
        .expect("address")
        .require_network(Signet)
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
    wallet.insert_tx(tx1, ConfirmationTime::Confirmed {height: 2_000, time: 200,},).unwrap();

    wallet.commit().unwrap();
    
    wallet
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
