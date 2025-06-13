use std::ops::Sub;

// use address::{PostAddress, SubAddress, UserAddress};
use call::CallMessage;
use post::Post;
use serde::{Deserialize, Serialize};
use sov_modules_api::{Context, Error, GenesisState, Module, ModuleId, ModuleInfo, ModuleRestApi, Spec, StateMap, TxState, WorkingSet};
use subreddit::SubReddit;
use user::User;
use sov_modules_api::EventEmitter;
use crate::event::Event;

pub mod call;
pub mod user;
pub mod utils;
pub mod subreddit;
pub mod post;
pub mod event;

#[cfg(feature = "native")]
pub mod query;



#[derive(Clone, ModuleInfo, ModuleRestApi)]
pub struct Reddit<S: Spec> {
    #[id]
    pub address: ModuleId,


    // #[state]
    // pub user_address_collections:  StateMap<C::Address , UserAddress<C>>,

    #[state]
    pub user_collections: StateMap<S::Address, User<S>>,

    #[state]
    pub sub_collections: StateMap<S::Address, SubReddit<S>>,

    #[state]
    pub post_collections: StateMap<S::Address , Post<S>>
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct RedditConfig {}


impl<S: Spec> Module for Reddit<S> {
     type Spec = S;

    type Config = RedditConfig;

    type CallMessage = CallMessage<S>;

    type Event = Event;


      fn genesis(
             &mut self,
        _header: &<S::Da as sov_modules_api::DaSpec>::BlockHeader,
        _config: &Self::Config,
        _state: &mut impl GenesisState<S>,
    ) -> Result<(), Error> {
        Ok(())
    }


        fn call(
       &mut self,
        msg: Self::CallMessage,
        context: &Context<Self::Spec>,
        state: &mut impl TxState<S>,
    ) -> Result<(), Error> {
      match msg {
            CallMessage::CreateUser {
                username,
            } => {
                
               let rsp =  self.create_new_user(&username.to_string(), context, state) ;
                self.emit_event(state , Event::UserCreatedEvent {
                    username: username.to_string().clone(),
                    user_address: rsp.response.user.unwrap().get_user_address().to_string(),
                });

            
            },
            CallMessage::CreateSubReddit { user_address , subname , description } => {
               let rsp =  self.create_new_subreddit(&subname.to_string(), &description.to_string(), context , state);

                let sub_rsp = rsp.response.sub_reddit.unwrap();

                     self.emit_event(state , Event::SubRedditCreatedEvent {
                        subname: sub_rsp.get_sub_name().to_string(),
                        description: sub_rsp.get_sub_description().to_string(),
                        subaddress: sub_rsp.get_sub_address().to_string(),
                                        mods: sub_rsp.get_mods().into_iter()
                    .map(|addr| addr.to_string())
                    .collect(),
                    });


            }
            CallMessage::CreatePost {
               title,
               flair,
               content,
               subaddress
            } => {
            let rsp =     self.create_new_post(
                &title.to_string(),
                &flair.to_string(),
                &content.to_string(),
                subaddress,
                context,
                state,
            );

            let post_rsp = rsp.response.post.unwrap();

            self.emit_event(state , Event::PostCreatedEvent { 
                title: post_rsp.get_post_title().to_string(), 
                content: post_rsp.get_post_content().to_string(), 
                subaddress: post_rsp.get_sub_address().to_string(), 
                post_address: post_rsp.get_post_address().to_string(),
                flair: post_rsp.get_post_flair().to_string(),
                user_address: post_rsp.get_user_address().to_string(), 
            });

            }

        };

        Ok(())
    }
    
        

}



