use self::{
    audio::SoundBank,
    table::{build_table, UserColumn},
    theme::dark, history::HistManager,
};
use crate::{
    api_provider::WebEndpoint,
    assets,
    client::{HikClient, OnlineUser},
    config::Config,
};
use anyhow::{Error, Result};
use cursive::{
    align::HAlign,
    view::Resizable,
    views::{Dialog, LinearLayout, TextView},
    CbSink, CursiveRunnable,
};
use cursive_table_view::TableView;
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinHandle, time};

mod audio;
mod table;
mod theme;
mod history;

enum Status {
    Idle,
    Running {
        cursive: CursiveRunnable,
        fetch_jh: JoinHandle<()>,
    },
}
pub struct AppTui {
    status: Status,
    config: Arc<Config>,
    audio_man: Arc<Mutex<SoundBank>>,
}

mod view_names {
    pub const ONLINE_USER: &str = "online_tbl";
    pub const HISTORY: &str = "history_tbl";
}

impl AppTui {
    const FETCH_USER_DEPLAY: u64 = 1111;

    pub fn new(conf: Config) -> Result<Self> {
        Ok(Self {
            config: Arc::new(conf),
            status: Status::Idle,
            audio_man: Arc::new(Mutex::new(SoundBank::from_array(assets::alert_sound())?)),
        })
    }
    fn build_tui() -> (CursiveRunnable, CbSink) {
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
                .child(TextView::new("Press q to quit").h_align(HAlign::Right)),
            // .child(TextView::new("Press c to clear").h_align(HAlign::Right)),
        );
        let sink = siv.cb_sink().clone();

        (siv, sink)
    }

    pub async fn start(&mut self) -> Result<()> {
        // captures
        let mut client = HikClient::new(
            &self.config.username,
            &self.config.password,
            WebEndpoint::new(&self.config.endpoint),
        );
        client.login().await?;

        let (mut siv, sink) = Self::build_tui();
        let conf = self.config.clone();
        let sb = self.audio_man.clone();

        let fetch_jh = tokio::spawn(async move {
            let mut hist_mngr = HistManager::new();
            let mut interval = time::interval(Duration::from_millis(Self::FETCH_USER_DEPLAY));
            let mut last_cur_count: usize = 0;
            loop {
                interval.tick().await;

                let online = match client.fetch_online_users().await {
                    Ok(o) => o,
                    Err(_e) => {
                        // TODO show error
                        continue;
                    }
                };

                hist_mngr.add_vec(&online.users);
                let hist = hist_mngr
                    .histories(&online.users)
                    .into_iter()
                    .filter(|o| o.name != conf.username)
                    .collect::<Vec<OnlineUser>>();

                let current = online
                    .users
                    .into_iter()
                    .filter(|o| o.name != conf.username)
                    .collect::<Vec<OnlineUser>>();

                // play alert if cur online count changed
                if current.len() != last_cur_count {
                    // TODO handle result err
                    let mut sb_lock = sb.lock().await;
                    sb_lock.play().unwrap();
                }
                last_cur_count = current.len();

                // updates TUI
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
        siv.set_theme(dark());
        siv.run();

        self.status = Status::Running {
            cursive: siv,
            fetch_jh,
        };

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        match &mut self.status {
            Status::Idle => return Err(Error::msg("app not running")),
            Status::Running { cursive, fetch_jh } => {
                cursive.quit();
                fetch_jh.abort();

                Ok(())
            }
        }
    }
}

impl Drop for AppTui {
    fn drop(&mut self) {
        match &mut self.status {
            Status::Idle => {}
            Status::Running {
                fetch_jh: _,
                cursive: _,
            } => {
                self.stop().unwrap();
            }
        }
    }
}
