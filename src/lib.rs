//! Exact 1:1 translation of Auto Reauth (upstream Java, MIT,
//! (c) Basicprogrammer10), master @e5b0f59 (2026-10-19). The Minecraft,
//! Fabric and JDK APIs that do not exist on Horizon are host seams with
//! the same shapes and semantics.
//!
//! The mod API wires what the pinned SDK (api 0.2, f00125f61f) reaches:
//! - the original's config toggles the Settings page can hold (`auto`,
//!   `debug`), same defaults as the config file;
//! - `client_setup` runs the mod entry 1:1 (load
//!   `config/auto-reauth/config.json`, then sync the settings into the
//!   live config cells the flows read).
//!
//! Gaps, tracked: the outbound-IO seam (the Microsoft OAuth chain's HTTP,
//! the browser open and the localhost callback listener: the port's
//! `HostSeam`) waits on a carrier HTTP hook; the `/autoreauth` and
//! `/reauth` chat commands wait on a chat-command hook in the mod API.
//! The command grammar and the whole OAuth/status core carry 1:1 in
//! `runtime`, tested.

use mod_api::{ClientSetupEvent, ClientHost, RegistrationContext, Registrar, SettingSpec};

pub mod runtime;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    /// The original's config toggles as settings, same defaults as the
    /// annotated config file.
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::toggle(
            "auto",
            "setting.auto_reauth.auto",
            true,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "debug",
            "setting.auto_reauth.debug",
            false,
        ));
    }

    /// 1:1 of the original's mod init: load `config/auto-reauth/config.json`
    /// (defaults when missing), then sync the settings into the live config
    /// cells the flows read.
    fn client_setup(&self, host: &mut dyn ClientHost, _event: &mut ClientSetupEvent) {
        let config = runtime::load_config(&runtime::game_config_dir())
            .unwrap_or_else(|_| {
                serde_json::from_str(runtime::DEFAULT_CONFIG)
                    .expect("the default config parses")
            });
        runtime::set_active_config(config);
        let auto = host.setting_bool("auto").unwrap_or(true);
        let debug = host.setting_bool("debug").unwrap_or(false);
        runtime::set_flag_active("auto", auto);
        runtime::set_flag_active("debug", debug);
    }
}
