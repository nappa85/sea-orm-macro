
use chrono::NaiveDateTime;

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: u64,
    pub email: String,
    pub encrypted_password: String,
    pub reset_password_token: Option<String>,
    pub reset_password_sent_at: Option<NaiveDateTime>,
    pub remember_created_at: Option<NaiveDateTime>,
    pub sign_in_count: u64,
    pub current_sign_in_at: Option<NaiveDateTime>,
    pub last_sign_in_at: Option<NaiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_ip: Option<String>,
    pub confirmation_token: Option<String>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub confirmation_sent_at: Option<NaiveDateTime>,
    pub unconfirmed_email: Option<String>,
    pub failed_attempts: u64,
    pub unlock_token: Option<String>,
    pub locked_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub salt: Option<String>,
    pub login: String,
    pub sl_key: Option<String>,
    pub promoter_id: Option<u64>,
    pub fb_user_id: Option<String>,
    pub last_address1: Option<String>,
    pub last_address2: Option<String>,
    pub has_accepted_current_tos: Option<bool>,
    pub api_key: Option<String>,
    pub last_activity_at: Option<NaiveDateTime>,
    pub tw_user_id: Option<String>,
    pub address3: Option<String>,
    pub gg_user_id: Option<String>,
    pub trust_level: Option<u16>,
    pub pin: Option<String>,
    pub last_address4: Option<String>,
    pub last_address5: Option<String>,
    pub last_address6: Option<String>,
    pub discount: Option<u64>,
    pub last_email: Option<String>,
    pub permanent_address: Option<String>,
    pub parent_id: Option<u64>,
    pub temp_pin: Option<String>,
    pub temp_pin_duration: Option<u64>,
    pub temp_pin_expires_at: Option<NaiveDateTime>,
    pub gauth_secret: Option<String>,
    pub gauth_enabled: Option<bool>,
    pub gauth_tmp: Option<String>,
    pub gauth_tmp_datetime: Option<NaiveDateTime>,
    pub unique_session_id: Option<String>,
    pub is_company: Option<bool>,
    pub grants: u64,
    pub auth_steps: u64,
    pub verification_email_sent_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Email,
    EncryptedPassword,
    ResetPasswordToken,
    ResetPasswordSentAt,
    RememberCreatedAt,
    SignInCount,
    CurrentSignInAt,
    LastSignInAt,
    CurrentSignInIp,
    LastSignInIp,
    ConfirmationToken,
    ConfirmedAt,
    ConfirmationSentAt,
    UnconfirmedEmail,
    FailedAttempts,
    UnlockToken,
    LockedAt,
    CreatedAt,
    UpdatedAt,
    Salt,
    Login,
    SlKey,
    PromoterId,
    FbUserId,
    LastAddress1,
    LastAddress2,
    HasAcceptedCurrentTos,
    ApiKey,
    LastActivityAt,
    TwUserId,
    Address3,
    GgUserId,
    TrustLevel,
    Pin,
    LastAddress4,
    LastAddress5,
    LastAddress6,
    Discount,
    LastEmail,
    PermanentAddress,
    ParentId,
    TempPin,
    TempPinDuration,
    TempPinExpiresAt,
    GauthSecret,
    GauthEnabled,
    GauthTmp,
    GauthTmpDatetime,
    UniqueSessionId,
    IsCompany,
    Grants,
    AuthSteps,
    VerificationEmailSentAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Email => ColumnType::String(None).def(),
            Self::EncryptedPassword => ColumnType::String(None).def(),
            Self::ResetPasswordToken => ColumnType::String(None).def().null(),
            Self::ResetPasswordSentAt => ColumnType::DateTime.def().null(),
            Self::RememberCreatedAt => ColumnType::DateTime.def().null(),
            Self::SignInCount => ColumnType::Integer.def(),
            Self::CurrentSignInAt => ColumnType::DateTime.def().null(),
            Self::LastSignInAt => ColumnType::DateTime.def().null(),
            Self::CurrentSignInIp => ColumnType::String(None).def().null(),
            Self::LastSignInIp => ColumnType::String(None).def().null(),
            Self::ConfirmationToken => ColumnType::String(None).def().null(),
            Self::ConfirmedAt => ColumnType::DateTime.def().null(),
            Self::ConfirmationSentAt => ColumnType::DateTime.def().null(),
            Self::UnconfirmedEmail => ColumnType::String(None).def().null(),
            Self::FailedAttempts => ColumnType::Integer.def(),
            Self::UnlockToken => ColumnType::String(None).def().null(),
            Self::LockedAt => ColumnType::DateTime.def().null(),
            Self::CreatedAt => ColumnType::DateTime.def(),
            Self::UpdatedAt => ColumnType::DateTime.def(),
            Self::Salt => ColumnType::String(None).def().null(),
            Self::Login => ColumnType::String(None).def(),
            Self::SlKey => ColumnType::String(None).def().null(),
            Self::PromoterId => ColumnType::Integer.def().null(),
            Self::FbUserId => ColumnType::String(None).def().null(),
            Self::LastAddress1 => ColumnType::String(None).def().null(),
            Self::LastAddress2 => ColumnType::String(None).def().null(),
            Self::HasAcceptedCurrentTos => ColumnType::Boolean.def().null(),
            Self::ApiKey => ColumnType::String(None).def().null(),
            Self::LastActivityAt => ColumnType::DateTime.def().null(),
            Self::TwUserId => ColumnType::String(None).def().null(),
            Self::Address3 => ColumnType::String(None).def().null(),
            Self::GgUserId => ColumnType::String(None).def().null(),
            Self::TrustLevel => ColumnType::Integer.def().null(),
            Self::Pin => ColumnType::String(None).def().null(),
            Self::LastAddress4 => ColumnType::String(None).def().null(),
            Self::LastAddress5 => ColumnType::String(None).def().null(),
            Self::LastAddress6 => ColumnType::String(None).def().null(),
            Self::Discount => ColumnType::Integer.def().null(),
            Self::LastEmail => ColumnType::String(None).def().null(),
            Self::PermanentAddress => ColumnType::String(None).def().null(),
            Self::ParentId => ColumnType::Integer.def().null(),
            Self::TempPin => ColumnType::String(None).def().null(),
            Self::TempPinDuration => ColumnType::Integer.def().null(),
            Self::TempPinExpiresAt => ColumnType::DateTime.def().null(),
            Self::GauthSecret => ColumnType::String(None).def().null(),
            Self::GauthEnabled => ColumnType::Boolean.def().null(),
            Self::GauthTmp => ColumnType::String(None).def().null(),
            Self::GauthTmpDatetime => ColumnType::DateTime.def().null(),
            Self::UniqueSessionId => ColumnType::String(None).def().null(),
            Self::IsCompany => ColumnType::Boolean.def().null(),
            Self::Grants => ColumnType::Integer.def(),
            Self::AuthSteps => ColumnType::Integer.def(),
            Self::VerificationEmailSentAt => ColumnType::DateTime.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        unreachable!()
    }
}

impl ActiveModelBehavior for ActiveModel {}

fn main() {
    println!("Hello, world!");
}
