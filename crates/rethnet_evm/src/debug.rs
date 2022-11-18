use auto_impl::auto_impl;
use rethnet_eth::{Address, H256, U256};
use revm::AccountInfo;

/// A trait for debug operation on a database.
#[auto_impl(Box)]
pub trait DatabaseDebug {
    /// The database's error type.
    type Error;

    /// Inserts an account with the specified `address`.
    fn insert_account(
        &mut self,
        address: Address,
        account_info: AccountInfo,
    ) -> Result<(), Self::Error>;

    /// Inserts a block with the specified `block_number` and `block_hash`.
    fn insert_block(&mut self, block_number: U256, block_hash: H256) -> Result<(), Self::Error>;

    /// Modifies the account at the specified address using the provided function.
    fn modify_account(
        &mut self,
        address: Address,
        modifier: Box<dyn Fn(&mut AccountInfo) + Send>,
    ) -> Result<(), Self::Error>;

    /// Retrieves the storage root of the database.
    fn storage_root(&mut self) -> Result<H256, Self::Error>;

    /// Creates a checkpoint that can be reverted to using [`revert`].
    fn checkpoint(&mut self) -> Result<(), Self::Error>;

    /// Reverts to the previous checkpoint, created using [`checkpoint`].
    fn revert(&mut self) -> Result<(), Self::Error>;
}

// /// A trait for objects that support [`DatabaseDebug`].
// pub trait HasDatabaseDebug {
//     /// The database's error type.
//     type Error;

//     /// Retrieves the owned `DatabaseDebug`.
//     fn db_debug(&mut self) -> &mut dyn DatabaseDebug<Error = Self::Error>;
// }

// impl<T: HasDatabaseDebug> DatabaseDebug for T {
//     type Error = <T as HasDatabaseDebug>::Error;

//     fn insert_account(
//         &mut self,
//         address: Address,
//         account_info: AccountInfo,
//     ) -> Result<(), Self::Error> {
//         self.db_debug().insert_account(address, account_info)
//     }

//     fn insert_block(&mut self, block_number: U256, block_hash: H256) -> Result<(), Self::Error> {
//         self.db_debug().insert_block(block_number, block_hash)
//     }

//     fn modify_account(
//         &mut self,
//         address: Address,
//         modifier: fn(&mut AccountInfo),
//     ) -> Result<(), Self::Error> {
//         self.db_debug().modify_account(address, modifier)
//     }

//     fn storage_root(&mut self) -> Result<H256, Self::Error> {
//         self.db_debug().storage_root()
//     }

//     fn checkpoint(&mut self) -> Result<(), Self::Error> {
//         self.db_debug().checkpoint()
//     }

//     fn revert(&mut self) -> Result<(), Self::Error> {
//         self.db_debug().revert()
//     }
// }
