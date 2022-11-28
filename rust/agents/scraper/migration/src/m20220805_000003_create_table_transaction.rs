use sea_orm_migration::prelude::*;

use crate::l20220805_types::*;
use crate::m20220805_000002_create_table_block::Block;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Transaction::TimeCreated)
                            .timestamp()
                            .not_null()
                            .default("NOW()"),
                    )
                    .col(
                        ColumnDef::new_with_type(Transaction::Hash, Hash)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Transaction::BlockId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Transaction::GasLimit).double().not_null())
                    .col(ColumnDef::new(Transaction::MaxPriorityFeePerGas).double())
                    .col(ColumnDef::new(Transaction::MaxFeePerGas).double())
                    .col(ColumnDef::new(Transaction::GasPrice).double())
                    .col(ColumnDef::new(Transaction::EffectiveGasPrice).double())
                    .col(ColumnDef::new(Transaction::Nonce).big_unsigned().not_null())
                    .col(ColumnDef::new_with_type(Transaction::Sender, Address).not_null())
                    .col(&mut ColumnDef::new_with_type(
                        Transaction::Recipient,
                        Address,
                    ))
                    .col(ColumnDef::new(Transaction::GasUsed).double().not_null())
                    .col(
                        ColumnDef::new(Transaction::CumulativeGasUsed)
                            .double()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(Transaction::BlockId)
                            .to(Block::Table, Block::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Transaction::Table)
                    .name("transaction_hash_idx")
                    .col(Transaction::Hash)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Transaction::Table)
                    .name("transaction_sender_idx")
                    .col(Transaction::Sender)
                    .index_type(IndexType::Hash)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Transaction::Table)
                    .name("transaction_block_idx")
                    .col(Transaction::BlockId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Transaction {
    Table,
    /// Unique database ID
    Id,
    /// Time of record creation
    TimeCreated,
    /// The transaction hash
    Hash,
    /// Block this transaction was included in
    BlockId,
    /// Amount of gas which was allocated for running the transaction
    GasLimit,
    MaxPriorityFeePerGas,
    MaxFeePerGas,
    /// Price paid for gas on this txn. Null for type 2 txns.
    GasPrice,
    EffectiveGasPrice,
    /// Nonce of this transaction by the sneder
    Nonce,
    /// Transaction signer
    Sender,
    /// Recipient or contract
    Recipient,
    /// Amount of gas used by this transaction
    GasUsed,
    /// Cumulative gas used within the block after this was executed
    CumulativeGasUsed,
}
