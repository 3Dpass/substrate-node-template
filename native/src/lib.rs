#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::runtime_interface;

#[runtime_interface]
pub trait HashableObject {
    fn calc() {
        println!("123456")
    }
}