use std::time::Duration;

use crate::history::HistManager;
use anyhow::Result;
use api_provider::WebEndpoint;
use client::*;
use colored::Colorize;
use config::*;
use tokio::time;

mod api_provider;
mod client;
mod config;
mod history;
mod macros;

const FETCH_USER_DEPLAY: u64 = 1333;

#[tokio::main]
async fn main() -> Result<()> {
    let env: Config = Config::read_env()?;
    let ep = WebEndpoint::new(&env.endpoint);
    let mut client = HikClient::new(&env.username, &env.password, ep);
    let mut hist_mngr: HistManager<OnlineUser> = HistManager::new();

    client.login().await?;
    println!("{}", "Login success!".green());

    let mut interval = time::interval(Duration::from_millis(FETCH_USER_DEPLAY));
    loop {
        interval.tick().await;

        let online = match client.fetch_online_users().await {
            Ok(o) => o,
            Err(e) => {
                println!("{}\n{}", "error while fetching users".red(), e);
                continue;
            }
        };

        hist_mngr.add_vec(&online.users);
        let hist = hist_mngr.histories(&online.users);

        let filtered_hist = utils::ignore_self(&hist, &env.username);
        let current = utils::ignore_self(&online.users, &env.username);

        cls!();
        println!("{}", "Running...");
        println!();
        println!();
        println!("{}", "Online 👀".bright_green());
        println!("{}", current);
        println!();
        println!();
        println!("{}", "History 📜".truecolor(66, 66, 66),);
        println!("{}", filtered_hist);
    }
}

mod utils {
    use crate::client::OnlineUser;

    use std::fmt::Display;

    pub fn ignore_self<'a>(users: &'a [OnlineUser], username: &str) -> UserList<'a> {
        let vec = users
            .iter()
            .filter(|o| o.name != username)
            .collect::<Vec<&'a OnlineUser>>();

        UserList(vec)
    }

    pub struct UserList<'a>(pub Vec<&'a OnlineUser>);

    impl<'a> Display for UserList<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.0.is_empty() {
                return f.write_str("...");
            }

            for user in self.0.iter() {
                f.write_fmt(format_args!(
                    "\n username: {} | ip: {} | logon: {}",
                    user.name, user.client_address.ip_address, user.login_time
                ))?;
            }
            Ok(())
        }
    }
}
