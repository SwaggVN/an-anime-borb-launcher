use relm4::{
    prelude::*,
    Sender
};

use gtk::glib::clone;

use crate::*;
use crate::i18n::*;
use crate::ui::components::*;

use super::{App, AppMsg};

pub fn download_diff(sender: ComponentSender<App>, progress_bar_input: Sender<ProgressBarMsg>, diff: VersionDiff) {
    sender.input(AppMsg::SetDownloading(true));

    std::thread::spawn(move || {
        let config = Config::get().unwrap();

        #[allow(unused_must_use)]
        let result = diff.install_to(config.game.path, clone!(@strong sender => move |state| {
            progress_bar_input.send(ProgressBarMsg::UpdateFromDiffState(state));
        }));

        let mut perform_on_download_needed = true;

        if let Err(err) = result {
            tracing::error!("Downloading failed: {err}");

            sender.input(AppMsg::Toast {
                title: tr("downloading-failed"),
                description: Some(err.to_string())
            });

            // Don't try to download something after state updating
            // because we just failed to do it
            perform_on_download_needed = false;
        }

        sender.input(AppMsg::SetDownloading(false));
        sender.input(AppMsg::UpdateLauncherState {
            perform_on_download_needed,
            show_status_page: false
        });
    });
}
