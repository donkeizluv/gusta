use cursive::{align::HAlign, view::Nameable, View};
use cursive_table_view::{TableView, TableViewItem};
use std::cmp::Ordering;

use crate::client::OnlineUser;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum UserColumn {
    Id,
    Name,
    UserType,
    LoginTime,
    ClientAddress,
}

impl UserColumn {
    pub fn as_str(&self) -> &str {
        match *self {
            UserColumn::Id => "Id",
            UserColumn::Name => "Name",
            UserColumn::UserType => "UserType",
            UserColumn::LoginTime => "Login",
            UserColumn::ClientAddress => "IP",
        }
    }
}

impl TableViewItem<UserColumn> for OnlineUser {
    fn to_column(&self, column: UserColumn) -> String {
        match column {
            UserColumn::Id => self.id.to_string(),
            UserColumn::Name => self.name.clone(),
            UserColumn::UserType => self.user_type.clone(),
            UserColumn::LoginTime => self.login_time.clone(),
            UserColumn::ClientAddress => self.client_address.ip_address.clone(),
        }
    }

    fn cmp(&self, other: &Self, column: UserColumn) -> std::cmp::Ordering
    where
        Self: Sized,
    {
        match column {
            UserColumn::Id => self.id.cmp(&other.id),
            UserColumn::Name => self.name.cmp(&other.name),
            UserColumn::UserType => self.user_type.cmp(&other.user_type),
            UserColumn::LoginTime => self.login_time.cmp(&other.login_time),
            UserColumn::ClientAddress => self
                .client_address
                .ip_address
                .cmp(&other.client_address.ip_address),
        }
    }
}

pub fn build_table(name: &str) -> impl View {
    let mut table = TableView::<OnlineUser, UserColumn>::new()
        // .column(UserColumn::Id, UserColumn::Id.as_str(), |c| {c})
        // .column(UserColumn::UserType, UserColumn::UserType.as_str(), |c| {c})
        .column(UserColumn::Name, UserColumn::Name.as_str(), |c| {
            c.align(HAlign::Center).width_percent(20)
        })
        .column(
            UserColumn::ClientAddress,
            UserColumn::ClientAddress.as_str(),
            |c| c.align(HAlign::Right).width_percent(40),
        )
        .column(UserColumn::LoginTime, UserColumn::LoginTime.as_str(), |c| {
            c.align(HAlign::Center)
                .align(HAlign::Right)
                .width_percent(40)
        });
    table.sort_by(UserColumn::Name, Ordering::Greater);
    table.with_name(name)
}
