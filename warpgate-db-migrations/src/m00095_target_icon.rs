use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

use crate::m00007_targets_and_roles::target;

/// Custom icon shown next to the target name on the gateway dashboard, stored
/// as a `data:image/…` URL. `NULL` means no custom icon.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut column = ColumnDef::new(Alias::new("icon"));
        // MySQL's TEXT stops at 64 KB, below the 4 MiB the API accepts.
        if manager.get_database_backend() == DbBackend::MySql {
            column.custom(Alias::new("MEDIUMTEXT"));
        } else {
            column.text();
        }
        manager
            .alter_table(
                Table::alter()
                    .table(target::Entity)
                    .add_column(column.null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(target::Entity)
                    .drop_column(Alias::new("icon"))
                    .to_owned(),
            )
            .await
    }
}
