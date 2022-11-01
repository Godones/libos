#![no_std]

mod buddy;
mod link_list;




#[cfg(feature = "buddy")]
pub type Allocator = buddy::BuddyAllocator;

#[cfg(feature = "linked-list")]
pub type Allocator = link_list::LinkAllocator;
