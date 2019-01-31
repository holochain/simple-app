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
    holochain_wasm_utils::api_serialization::{
        get_links::GetLinksResult,
    },
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Item {
    content: String,
}

pub fn handle_share_item(item: Item) -> ZomeApiResult<Address> {
    hdk::debug(format!("sharing {:?}", item));
    let post_entry = Entry::App("item".into(), item.into());
    let address = hdk::commit_entry(&post_entry)?;
    Ok(address)
}

pub fn handle_get_item(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

pub fn handle_add_link(base: Address, target: Address) -> ZomeApiResult<()> {
    let address = hdk::link_entries(&base,&target,"the_tag")?;
    Ok(())
}

pub fn handle_get_links(base: Address) -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(&base, "the_tag")
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
        },

        links: [
            from!(
                "item",
                tag: "the_tag",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData | {
                    Ok(())
                }
            )
        ]

    )
}

define_zome! {
    entries: [
       definition()
    ]

    genesis: || { Ok(()) }

    functions: [
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
        add_link: {
            inputs: |base: Address, target: Address|,
            outputs: |result: ZomeApiResult<()>|,
            handler: handle_add_link
        }
        get_links: {
            inputs: |base: Address|,
            outputs: |result: ZomeApiResult<GetLinksResult>|,
            handler: handle_get_links
        }
    ]

    capabilities: {
        public (Public) [share_item, get_item, add_link, get_links]
    }
}
