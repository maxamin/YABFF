use super::*;
use crate::{
    progress::PROGRESS_PRINTER,
    scan_manager::{FeroxState, PAUSE_SCAN},
    scanner::RESPONSES,
    statistics::StatError,
    utils::slugify_filename,
    utils::{open_file, write_to},
    SLEEP_DURATION,
};
use anyhow::Result;
use console::style;
use crossterm::event::{self, Event, KeyCode};
use std::{
    env::temp_dir,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

/// Atomic boolean flag, used to determine whether or not the terminal input handler should exit
pub static SCAN_COMPLETE: AtomicBool = AtomicBool::new(false);

/// Container for filters transmitter and FeroxFilters object
pub struct TermInputHandler {
    /// handles to other handlers
    handles: Arc<Handles>,
}

/// implementation of event handler for terminal input
///
/// kicks off the following handlers related to terminal input:
///     termination handler (SIGINT/SIGTERM/SIGHUP) that saves scan state to disk
///     enter handler that listens for enter during scans to drop into interactive scan management menu
impl TermInputHandler {
    /// Create new event handler
    pub fn new(handles: Arc<Handles>) -> Self {
        Self { handles }
    }

    /// Initialize the sigint and enter handlers that are responsible for handling initial user
    /// interaction during scans
    pub fn initialize(handles: Arc<Handles>) {
        log::trace!("enter: initialize({handles:?})");

        let handler = Self::new(handles);
        handler.start();

        log::trace!("exit: initialize");
    }

    /// wrapper around sigint_handler and enter_handler
    fn start(&self) {
        tokio::task::spawn_blocking(Self::enter_handler);

        if self.handles.config.save_state || self.handles.config.ml {
            // start the termination handler (ctrlc's "termination" feature routes
            // SIGINT/SIGTERM/SIGHUP here). Also needed when ML is on, so the learned
            // model is persisted on those signals even if scan-state saving is off.
            let cloned = self.handles.clone();

            let result = ctrlc::set_handler(move || {
                let _ = Self::sigint_handler(cloned.clone());
            });

            if result.is_err() {
                log::warn!("Could not set termination handler; scan state will not be saved");
                self.handles
                    .stats
                    .send(Command::AddError(StatError::Other))
                    .unwrap_or_default();
            }
        }
    }

    /// Writes the current state of the program to disk (if save_state is true) and then exits
    pub fn sigint_handler(handles: Arc<Handles>) -> Result<()> {
        log::trace!("enter: sigint_handler({handles:?})");

        // ML layer: persist the model learned so far before we exit. The handler
        // ends in process::exit, so clean_up's save() never runs on a signal;
        // without this the whole run's online learning would be lost. No-op if
        // --ml-model is unset.
        if handles.config.ml {
            crate::ml::save();
        }

        // check for STATE_FILENAME env var first; credit to Tobias Rauch for the idea
        let filename = if let Ok(path) = std::env::var("STATE_FILENAME") {
            path
        } else if !handles.config.target_url.is_empty() {
            // target url populated
            slugify_filename(&handles.config.target_url, "ferox", "state")
        } else {
            // stdin used
            slugify_filename("stdin", "ferox", "state")
        };

        let warning = format!(
            "🚨 Caught {} 🚨 saving scan state to {} ...",
            style("termination signal").yellow(),
            filename
        );

        PROGRESS_PRINTER.println(warning);

        let state = FeroxState::new(
            handles.ferox_scans()?,
            handles.config.clone(),
            &RESPONSES,
            handles.stats.data.clone(),
            handles.filters.data.clone(),
        );

        // User didn't set the --no-state flag (so saved_state is still the default true)
        if handles.config.save_state {
            let Ok(mut state_file) = open_file(&filename) else {
                // couldn't open the file, let the user know we're going to try again
                let error = format!(
                    "❌ Could not save {}, falling back to {}",
                    filename,
                    temp_dir().to_string_lossy()
                );
                PROGRESS_PRINTER.println(error);

                let temp_filename = temp_dir().join(&filename);

                let Ok(mut state_file) = open_file(&temp_filename.to_string_lossy()) else {
                    // couldn't open the fallback file, let the user know
                    let error = format!("❌❌ Could not save {temp_filename:?}, giving up...");
                    PROGRESS_PRINTER.println(error);

                    log::trace!("exit: sigint_handler (failed to write)");
                    std::process::exit(1);
                };

                write_to(&state, &mut state_file, true)?;

                let msg = format!("✅ Saved scan state to {temp_filename:?}");
                PROGRESS_PRINTER.println(msg);

                log::trace!("exit: sigint_handler (saved to temp folder)");
                std::process::exit(1);
            };

            write_to(&state, &mut state_file, true)?;
        }

        log::trace!("exit: sigint_handler (end of program)");
        std::process::exit(1);
    }

    /// Handles specific key events triggered by the user over stdin
    fn enter_handler() {
        // todo eventually move away from atomics, the blocking recv is the problem
        log::trace!("enter: start_enter_handler");

        loop {
            if PAUSE_SCAN.load(Ordering::Relaxed) {
                // if the scan is already paused, we don't want this event poller fighting the user
                // over stdin
                sleep(Duration::from_millis(SLEEP_DURATION));
            } else if event::poll(Duration::from_millis(SLEEP_DURATION)).unwrap_or(false) {
                // It's guaranteed that the `read()` won't block when the `poll()`
                // function returns `true`

                if let Ok(key_pressed) = event::read() {
                    // ignore any other keys
                    if key_pressed == Event::Key(KeyCode::Enter.into()) {
                        // if the user presses Enter, set PAUSE_SCAN to true. The interactive menu
                        // will be triggered and will handle setting PAUSE_SCAN to false
                        PAUSE_SCAN.store(true, Ordering::Release);
                    }
                }
            } else {
                // Timeout expired and no `Event` is available; use the timeout to check SCAN_COMPLETE
                if SCAN_COMPLETE.load(Ordering::Relaxed) {
                    // scan has been marked complete by main, time to exit the loop
                    break;
                }
            }
        }
        log::trace!("exit: start_enter_handler");
    }
}
