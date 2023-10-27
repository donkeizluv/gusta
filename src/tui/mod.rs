use self::{
    table::{build_table, UserColumn},
    theme::get_theme,
};
use crate::{
    api_provider::WebEndpoint,
    client::{HikClient, OnlineUser},
    config::Config,
    history::HistManager,
};
use anyhow::Result;
use cursive::{
    align::HAlign,
    view::Resizable,
    views::{Dialog, LinearLayout, TextView},
    CbSink, CursiveRunnable,
};
use cursive_table_view::TableView;
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::Mutex,
    task::{self, JoinHandle},
    time,
};

mod table;
mod theme;

enum Status {
    Idle,
    Running {
        cursive: CursiveRunnable,
        cb_sink: CbSink,
        fetch_online_h: JoinHandle<()>,
        hist_mngr: Arc<Mutex<HistManager<OnlineUser>>>,
        hik_client: Arc<HikClient<WebEndpoint>>,
    },
}
pub struct AppTui {
    status: Status,
    config: Arc<Config>,
}

mod view_names {
    pub const ONLINE_USER: &str = "online_tbl";
    pub const HISTORY: &str = "history_tbl";
}
const FETCH_USER_DEPLAY: u64 = 1111;

impl AppTui {
    pub fn new(conf: Config) -> Self {
        Self {
            config: Arc::new(conf),
            status: Status::Idle,
        }
    }
    fn build_siv() -> (CursiveRunnable, CbSink) {
        let mut siv = cursive::default();
        siv.add_global_callback('q', |s| s.quit());
        // siv.add_global_callback('c', |_s| {
        //     // hist_mngr.clear()
        // });

        siv.add_fullscreen_layer(
            LinearLayout::vertical()
                .child(
                    Dialog::new()
                        .title("Online")
                        .content(build_table(view_names::ONLINE_USER))
                        .full_screen(),
                )
                .child(
                    Dialog::new()
                        .title("History")
                        .content(build_table(view_names::HISTORY))
                        .full_screen(),
                )
                .child(TextView::new("Press q to quit").h_align(HAlign::Right)), // .child(TextView::new("Press c to clear").h_align(HAlign::Right)),
        );
        let sink = siv.cb_sink().clone();

        (siv, sink)
    }
    
    pub async fn start(&mut self) -> Result<()> {
        let (mut siv, cb_sink) = Self::build_siv();
        let mut hik_client = HikClient::new(
            &self.config.username,
            &self.config.password,
            WebEndpoint::new(&self.config.endpoint),
        );
        hik_client.login().await?;

        let arc_client = Arc::new(hik_client);
        let arc_hist_mngr = Arc::new(Mutex::new(HistManager::new()));

        // captures
        let sink = cb_sink.clone();
        let conf = self.config.clone();
        let c_client = arc_client.clone();
        let c_hist_mngr = arc_hist_mngr.clone();
        let fetch_h: JoinHandle<()> = task::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(FETCH_USER_DEPLAY));
            loop {
                interval.tick().await;

                let online = match c_client.fetch_online_users().await {
                    Ok(o) => o,
                    Err(_e) => {
                        // TODO show error
                        continue;
                    }
                };
                let hist = {
                    let mut hist_lock = c_hist_mngr.lock().await;
                    hist_lock.add_vec(&online.users);
                    hist_lock
                        .histories(&online.users)
                        .into_iter()
                        .filter(|o| o.name != conf.username)
                        .collect::<Vec<OnlineUser>>()
                };

                let current = online
                    .users
                    .into_iter()
                    .filter(|o| o.name != conf.username)
                    .collect::<Vec<OnlineUser>>();

                let _res = sink.clone().send(Box::new(|s| {
                    s.call_on_name(
                        view_names::ONLINE_USER,
                        |t: &mut TableView<OnlineUser, UserColumn>| {
                            t.set_items_stable(current);
                        },
                    );
                }));
                // if res.is_err() {}

                let _res = sink.clone().send(Box::new(|s| {
                    s.call_on_name(
                        view_names::HISTORY,
                        |t: &mut TableView<OnlineUser, UserColumn>| {
                            t.set_items_stable(hist);
                        },
                    );
                }));
                // TODO show error
                // if res.is_err() {}
            }
        });
        siv.set_theme(get_theme());
        siv.run();

        self.status = Status::Running {
            cursive: siv,
            cb_sink,
            hik_client: arc_client,
            hist_mngr: arc_hist_mngr,
            fetch_online_h: fetch_h,
        };

        Ok(())
    }

    pub fn stop(&mut self) {
        self.status = Status::Idle
    }
}

impl Drop for AppTui {
    fn drop(&mut self) {
        match &mut self.status {
            Status::Idle => {}
            Status::Running {
                fetch_online_h,
                hist_mngr: _,
                hik_client: _,
                cursive,
                cb_sink: _,
            } => {
                cursive.quit();
                fetch_online_h.abort()
            }
        }
    }
}
