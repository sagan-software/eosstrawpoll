use crate::types::*;
use crate::constants::*;
use eosio::*;

#[eosio_action]
pub fn transfer(from: AccountName, to: AccountName, quantity: Asset, memo: String) {
    let _self = AccountName::receiver();

    if to != _self {
        return;
    }

    let donated_symbol = quantity.symbol;
    if donated_symbol != SYS && donated_symbol != EOS && donated_symbol != TLOS {
        return;
    }

    let donors = Donor::table(_self, _self);
    let cursor = donors.find(from);
    match cursor {
        Some(cursor) => {
            let mut donor = cursor.get().assert("read");
            donor.donated += quantity.amount;
            cursor.modify(None, &donor).assert("write");
        }
        None => {
            let donor = Donor {
                account: from,
                donated: quantity.amount,
            };
            donors.emplace(_self, &donor).assert("write");
        }
    }

    let donations = Donation::table(_self, _self);
    let id = donations.available_primary_key().assert("error getting next available primary key"); 
    eosio_print!("!!! ", id);
    let donation = Donation {
        id,
        account: from,
        donated: quantity.amount,
        memo,
        create_time: Time::now()
    };
    donations.emplace(_self, &donation).assert("write");
}
