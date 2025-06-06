use anyhow::{bail, Result};
use schemars::JsonSchema;
use sov_modules_api::macros::UniversalWallet;
use sov_modules_api::{Address, Context, SafeString, Spec, TxState, WorkingSet};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use crate::user::User;
use crate::{post::Post, subreddit::SubReddit, Reddit};



#[derive(
    BorshDeserialize,
    BorshSerialize,
    Debug,
    PartialEq,
    Eq,
    JsonSchema,
    Clone,
      UniversalWallet,
    Serialize,
    Deserialize,
)]
#[schemars(bound = "S::Address: ::schemars::JsonSchema", rename = "CallMessage")]
#[serde(rename_all = "snake_case")]
pub enum CallMessage<S: Spec> {

    CreateUser {
        username: SafeString,
    },

    CreateSubReddit {
        user_address: S::Address,
        subname: SafeString,
        description: SafeString,
    },

    CreatePost {
        title: SafeString,
        flair: SafeString,
        content: SafeString,
        subaddress: S::Address
    }
}


#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RedditModels<S: Spec> {
    pub user: Option<User<S>>,
     pub sub_reddit: Option<SubReddit<S>>,
        pub post: Option<Post<S>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RedditResponse<S: Spec> {
    pub response: RedditModels<S>,
}




impl<S: Spec> Reddit<S> {


    pub(crate) fn create_new_user(
        &mut self,
        username: &str,
        context: &Context<S>,
        state: &mut impl TxState<S>,
    ) -> RedditResponse<S> {
        let (new_user_address , new_user) = User::new(username, &self.user_collections, context, state).unwrap();

        
            //self.user_address_collections.set(context.sender(), &new_user_address, state);
        self.user_collections.set(&new_user_address, &new_user, state);

            RedditResponse {
                response: RedditModels {
                    user: Some(new_user),
                    post: None,
                    sub_reddit: None
                },
            }

    }


    pub(crate) fn create_new_subreddit(
        &mut self,
        subname: &str,
        description: &str,
        context: &Context<S>,
        state: &mut impl TxState<S>,
    ) -> RedditResponse<S> {


           let (new_sub_address , new_sub) = SubReddit::new(subname, description, &self.sub_collections, context, state).unwrap();

        self.sub_collections.set(&new_sub_address, &new_sub, state);


                RedditResponse {
                response: RedditModels {
                    user: None,
                    post: None,
                    sub_reddit: Some(new_sub)
                },
            }

        


    }


      pub(crate) fn create_new_post(
        &mut self,
        title: &str,
        flair: &str,
        content: &str,
        subaddress: S::Address,
        context: &Context<S>,
        state: &mut impl TxState<S>,
    ) -> RedditResponse<S> {


           let (new_post_address , new_post) = Post::new(title, flair, content, subaddress , context, state).unwrap();

        self.post_collections.set(&new_post_address, &new_post, state);


                RedditResponse {
                response: RedditModels {
                    user: None,
                    post: Some(new_post) ,
                    sub_reddit: None
                },
            }

        


    }






}