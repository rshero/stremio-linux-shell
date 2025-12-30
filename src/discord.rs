use anyhow::Result;
use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, Assets, Button, Timestamps},
};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info};

const APP_ID: &str = "1361448446862692492"; // Stremio Discord App ID

pub struct Discord {
    client: Option<DiscordIpcClient>,
    enabled: bool,
}

impl Discord {
    pub fn new(enabled: bool) -> Self {
        let mut discord = Self {
            client: None,
            enabled,
        };

        if enabled {
            discord.connect();
        }

        discord
    }

    fn connect(&mut self) {
        if let Ok(mut client) = DiscordIpcClient::new(APP_ID) {
            match client.connect() {
                Ok(_) => {
                    info!("🎮 Discord Rich Presence connected");
                    self.client = Some(client);
                }
                Err(e) => {
                    error!("Failed to connect to Discord: {}", e);
                }
            }
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if enabled && self.client.is_none() {
            self.connect();
        } else if !enabled {
            self.clear();
            if let Some(mut client) = self.client.take() {
                let _ = client.close();
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn update_presence(&mut self, args: Vec<String>) {
        if !self.enabled || self.client.is_none() {
            return;
        }

        if args.is_empty() {
            return;
        }

        match args[0].as_str() {
            "watching" if args.len() >= 10 => self.set_watching_presence(&args),
            "meta-detail" if args.len() >= 4 => self.set_meta_detail_presence(&args),
            "board" => self.set_discover_presence("Resuming Favorites", "On Board"),
            "discover" => self.set_discover_presence("Finding New Gems", "In Discover"),
            "library" => self.set_discover_presence("Revisiting Old Favorites", "In Library"),
            "calendar" => self.set_discover_presence("Planning My Next Binge", "On Calendar"),
            "addons" => self.set_discover_presence("Exploring Add-ons", "In Add-ons"),
            "settings" => self.set_discover_presence("Tuning Preferences", "In Settings"),
            "search" => self.set_discover_presence("Searching for Shows & Movies", "In Search"),
            "clear" => self.clear(),
            _ => {}
        }
    }

    fn set_watching_presence(&mut self, args: &[String]) {
        // 0: "watching"
        // 1: type (movie, series)
        // 2: title
        // 3: season
        // 4: episode
        // 5: episode name
        // 6: episode thumbnail (small image)
        // 7: show/movie image (large image)
        // 8: elapsed seconds
        // 9: duration seconds
        // 10: isPaused ("yes" or "no")
        // 11: more detail button link (imdb)
        // 12: watch on stremio button link

        let is_paused = args.get(10).map(|s| s.as_str() == "yes").unwrap_or(false);

        let mut activity = Activity::new().details(&args[2]);

        // Build state string outside the conditional blocks to avoid lifetime issues
        let state_str = if is_paused {
            "Paused".to_string()
        } else if args[1] == "series" {
            format!("{} (S{}-E{})", args[5], args[3], args[4])
        } else {
            "Enjoying a Movie".to_string()
        };

        activity = activity.state(&state_str);

        // Set timestamps and images
        if !is_paused {
            let elapsed: i64 = args[8].parse().unwrap_or(0);
            let duration: i64 = args[9].parse().unwrap_or(0);
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let start_time = now - elapsed;
            let end_time = now + (duration - elapsed);

            activity = activity.timestamps(Timestamps::new().start(start_time).end(end_time));
        }

        // Set assets (images)
        if args[1] == "series" {
            if let Some(thumb) = args.get(6).filter(|s| !s.is_empty()) {
                activity = activity.assets(
                    Assets::new()
                        .large_image(&args[7])
                        .large_text(&args[2])
                        .small_image(thumb)
                        .small_text(&args[5]),
                );
            } else {
                activity =
                    activity.assets(Assets::new().large_image(&args[7]).large_text(&args[2]));
            }
        } else {
            activity = activity.assets(Assets::new().large_image(&args[7]).large_text(&args[2]));
        }

        // Add buttons
        let mut buttons = Vec::new();
        if let Some(imdb) = args.get(11).filter(|s| !s.is_empty()) {
            buttons.push(Button::new("More Details", imdb));
        }
        if let Some(stremio) = args.get(12).filter(|s| !s.is_empty()) {
            buttons.push(Button::new("Watch on Stremio", stremio));
        }
        if !buttons.is_empty() {
            activity = activity.buttons(buttons);
        }

        self.update_activity(activity);
    }

    fn set_meta_detail_presence(&mut self, args: &[String]) {
        // 0: "meta-detail"
        // 1: type (movie, series)
        // 2: title
        // 3: image URL

        let state = if args[1] == "movie" {
            "Exploring a Movie"
        } else {
            "Exploring a Series"
        };

        let activity = Activity::new()
            .details(&args[2])
            .state(state)
            .assets(Assets::new().large_image(&args[3]).large_text(&args[2]));

        self.update_activity(activity);
    }

    fn set_discover_presence(&mut self, details: &str, state: &str) {
        let activity = Activity::new()
            .details(details)
            .state(state)
            .assets(
                Assets::new()
                    .large_image("https://raw.githubusercontent.com/Stremio/stremio-web/refs/heads/development/images/icon.png")
                    .large_text("Stremio")
            );

        self.update_activity(activity);
    }

    fn update_activity(&mut self, activity: Activity) {
        if let Some(client) = &mut self.client {
            if let Err(e) = client.set_activity(activity) {
                error!("Failed to update Discord presence: {}", e);
            }
        }
    }

    pub fn clear(&mut self) {
        if let Some(client) = &mut self.client {
            let _ = client.clear_activity();
        }
    }
}

impl Drop for Discord {
    fn drop(&mut self) {
        if let Some(mut client) = self.client.take() {
            let _ = client.close();
        }
    }
}
