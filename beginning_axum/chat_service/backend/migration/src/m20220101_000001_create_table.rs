use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string_uniq(Users::Username))
                    .col(string(Users::Password))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(pk_auto(Room::Id))
                    .col(string(Room::Participants))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(pk_auto(Chat::Id))
                    .col(
                        timestamp_with_time_zone(Chat::Timestamp)
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(Chat::SenderId))
                    .col(text(Chat::Message))
                    .col(integer(Chat::RoomId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_sender_id")
                            .from(Chat::Table, Chat::SenderId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_room_id")
                            .from(Chat::Table, Chat::RoomId)
                            .to(Room::Table, Room::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chat_room_id_timestamp")
                    .table(Chat::Table)
                    .col(Chat::RoomId)
                    .col(Chat::Timestamp)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Password,
}

#[derive(DeriveIden)]
enum Room {
    Table,
    Id,
    Participants,
}

#[derive(DeriveIden)]
enum Chat {
    Table,
    Id,
    Timestamp,
    SenderId,
    Message,
    RoomId,
}
