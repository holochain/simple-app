#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Item {
    content: String,
}

pub fn handle_share_item(item: Item) -> ZomeApiResult<Address> {
    let post_entry = Entry::App("item".into(), item.into());
    let address = hdk::commit_entry(&post_entry)?;
    Ok(address)
}

pub fn handle_get_item(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(address)
}

fn definition() -> ValidatingEntryType {
    entry!(
        name: "item",
        description: "an item to share",
        sharing: Sharing::Public,
        native_type: Item,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_item: Item, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}
define_zome! {
    entries: [
       definition()
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            share_item: {
                inputs: |item: Item|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: handle_share_item
            }
            get_item: {
                inputs: |address: Address|,
                outputs: |result: ZomeApiResult<Option<Entry>>|,
                handler: handle_get_item
            }
        }
    }
}
