use std::collections::HashMap;
use anyhow::{anyhow, bail};
use sov_modules_api::{Context, Spec, StateMap, TxState, WorkingSet};
use serde::{Deserialize , Serialize};
use crate::{utils::get_sub_address};



#[derive(borsh::BorshDeserialize, borsh::BorshSerialize,  Deserialize , Serialize, Debug, PartialEq, Clone , Eq)]
/// Defines an nft collection
pub struct SubReddit<S: Spec> {
    subaddress: S::Address,
    subname: String,
    description: String,
    mods: Vec<S::Address>
}

impl<S: Spec> SubReddit<S> {


 pub fn new(
    subname: &str,
    description: &str,
    sub_collections: &StateMap<S::Address, SubReddit<S>>,
    context: &Context<S>,
        state: &mut impl TxState<S>,
 ) -> anyhow::Result<(S::Address, SubReddit<S>)> {

         let creator = context.sender();

    let sub_address = get_sub_address::<S>(subname);


    let sub_add = sub_collections.get(&sub_address, state)?;

    if sub_add.is_some() {
        Err(anyhow!( "Subreddit with subname={} already exists", subname ))
    } else {
        Ok(  
            (sub_address.clone() , SubReddit {
             subaddress: sub_address.clone(), 
            subname: subname.to_string(), 
            description: description.to_string(), 
            mods: vec![creator.clone()] })

         )
    }


    }


    #[allow(dead_code)]
    pub fn get_sub_name(&self) -> &str {
        &self.subname
    }


        #[allow(dead_code)]
    pub fn get_sub_description(&self) -> &str {
        &self.description
    }


        #[allow(dead_code)]
    pub fn get_sub_address(&self) -> &S::Address{
        &self.subaddress
    }


           #[allow(dead_code)]
    pub fn get_mods(&self) -> &Vec<S::Address> {
        &self.mods
    }


}