use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // CREATE TABLE - USERS
        manager.create_table(Table::create().if_not_exists().table(User::Table)
            .col(ColumnDef::new(User::Id).uuid().primary_key())
            .to_owned()
        )
            .await?;

        // CREATE TABLE - CREDENTIALS
        manager.create_table(Table::create().if_not_exists().table(Credential::Table)
            // UserID is unique -> One user can have only one credential
            .col(ColumnDef::new(Credential::UserId).uuid().not_null().unique_key())
            .col(ColumnDef::new(Credential::Login).string_len(31).not_null().unique_key())
            .col(ColumnDef::new(Credential::Password).string_len(127).not_null())
            .to_owned()
        )
            .await?;

        // CREATE TABLE - SESSIONS
        manager.create_table(Table::create().if_not_exists().table(Session::Table)
            // UserID is not unique -> One user can have multiply sessions.
            .col(ColumnDef::new(Session::UserId).uuid().not_null())
            .col(ColumnDef::new(Session::Token).string().not_null())
            .to_owned()
        )
            .await?;
        
        // CREATE RELATIONSHIP - USER 1:1 CREDENTIALS
        manager.create_foreign_key(ForeignKey::create()
            .name("fk_user")
            .from(Credential::Table, Credential::UserId)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned()
        )
            .await?;

        // CREATE RELATIONSHIP - USER 1:M SESSIONS
        manager.create_foreign_key(ForeignKey::create()
            .name("fk_user")
            .from(Session::Table, Session::UserId)
            .to(User::Table, User::Id)
            .to_owned()
        )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        // DROP RELATIONSHIP - USER 1:M SESSIONS
        manager.drop_foreign_key(ForeignKey::drop()
            .name("fk_user")
            .table(Session::Table)
            .to_owned()
        )
            .await?;

        // DROP RELATIONSHIP - USER 1:1 CREDENTIAL
        manager.drop_foreign_key(ForeignKey::drop()
            .name("fk_user")
            .table(Credential::Table)
            .to_owned()
        )
            .await?;

        // DROP TABLE - SESSIONS
        manager.drop_table(Table::drop().if_exists()
            .table(Session::Table)
            .to_owned()
        )
            .await?;

        // DROP TABLE - CREDENTIALS
        manager.drop_table(Table::drop().if_exists()
            .table(Credential::Table)
            .to_owned()
        )
            .await?;

        // DROP TABLE - USERS
        manager.drop_table(Table::drop().if_exists()
            .table(User::Table)
            .to_owned()
        )
            .await
    }
}


#[derive(DeriveIden)]
enum Credential {
    Table,
    UserId,
    Login,
    Password,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    UserId,
    Token,
}