use sov_modules_api::{Address, Context, Spec, TxState, WorkingSet};
use serde::{Deserialize , Serialize};
use crate::{user::User, utils::get_post_address};




#[derive(borsh::BorshDeserialize, borsh::BorshSerialize,  Deserialize , Serialize, Debug, PartialEq, Clone , Eq)]
/// Defines an nft collection
pub struct Post<S: Spec> {
    post_address: S::Address,
    user_address: S::Address,
    subaddress: S::Address,
    post_title: String,
    flair: String,
    content: String,
    status: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostStatus{
    ACTIVE,
    ARCHIVED,
    DELETED
}

impl PostStatus {


        fn to_string(&self) -> String {
        match self {
            PostStatus::ACTIVE => "ACTIVE".to_string(),
            PostStatus::ARCHIVED => "ARCHIVED".to_string(),
            PostStatus::DELETED => "DELETED".to_string(),
        }
    }

    // Convert string to enum
    fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "ACTIVE" => Ok(PostStatus::ACTIVE),
            "ARCHIVED" => Ok(PostStatus::ARCHIVED),
            "DELETED" => Ok(PostStatus::DELETED),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }

}

impl<S: Spec> Post<S> {
    pub fn new(
        title: &str,
        flair: &str,
        content: &str,
        sub_address: S::Address,
    context: &Context<S>,
        state: &mut impl TxState<S>,
 ) -> anyhow::Result<(S::Address , Post<S>)> {


        let creator = context.sender();

    let post_address = get_post_address::<S>(creator.as_ref() ,
     sub_address.as_ref() );

    Ok(  

        (post_address.clone() , Post {
        post_address,
        user_address: creator.clone(),
        subaddress: sub_address,
        post_title: title.to_string(),
        flair: flair.to_string(),
        content: content.to_string(),
        status: PostStatus::ACTIVE.to_string(),
    })

    )
    
 }

 #[allow(dead_code)]
 pub fn get_post_address(&self) -> &S::Address {
    &self.post_address
 }


  #[allow(dead_code)]
 pub fn get_user_address(&self) -> &S::Address {
    &self.user_address
 }

  #[allow(dead_code)]
 pub fn get_sub_address(&self) -> &S::Address {
    &self.subaddress
 }
 
 
  #[allow(dead_code)]
 pub fn get_post_title(&self) -> &str {
    &self.post_title
 } 
 
 
 #[allow(dead_code)]
 pub fn get_post_flair(&self) -> &str {
    &self.flair
 } 
 
 
 #[allow(dead_code)]
 pub fn get_post_content(&self) -> &str {
    &self.content
 } 
 
 #[allow(dead_code)]
 pub fn get_post_status(&self) -> &str {
    &self.status
 }


}