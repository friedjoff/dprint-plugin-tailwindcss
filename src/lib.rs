mod config;

use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
use dprint_core::generate_plugin_code;
use dprint_core::plugins::{
    FormatResult, PluginInfo, PluginResolveConfigurationResult, 
    SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};

use config::Configuration;

struct TailwindCssPluginHandler;

impl TailwindCssPluginHandler {
    const fn new() -> Self {
        TailwindCssPluginHandler
    }
}

impl SyncPluginHandler<Configuration> for TailwindCssPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "tailwindcss".to_string(),
            help_url: "https://github.com/friedjoff/dprint-plugin-tailwindcss".to_string(),
            config_schema_url: String::new(),
            update_url: Some("https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss/latest.json".to_string()),
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE").to_string()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        config::resolve_config(config, global_config)
    }

    fn check_config_updates(&self, _message: dprint_core::plugins::CheckConfigUpdatesMessage) -> anyhow::Result<Vec<dprint_core::plugins::ConfigChange>> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        // Check if plugin is enabled
        if !request.config.enabled {
            return Ok(None);
        }

        // TODO: Implement actual formatting logic
        // For now, return None (no changes)
        Ok(None)
    }
}

// Generate the WASM plugin code
generate_plugin_code!(TailwindCssPluginHandler, TailwindCssPluginHandler::new());
