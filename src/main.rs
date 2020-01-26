// Copyright 2020 Mathias Kraus - All rights reserved
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be
// copied, modified, or distributed except according to those terms.

mod params;

mod app;
mod event;
mod types;
mod ui;

use crate::app::App;
use crate::event::{Config, Event, Events};

use iceoryx_rs::Runtime;

use structopt::StructOpt;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use tui::backend::TermionBackend;
use tui::Terminal;

use std::io;
use std::time::Duration;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let params = params::Params::from_args();

    Runtime::get_intance("/iceray");

    let events = Events::new(Config {
        tick_rate: Duration::from_millis(params.update_interval),
        ..Config::default()
    });

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new("IceRay - IceOryx Introspection");
    app.tabs.index = params.initial_page as usize;
    ui::draw(&mut terminal, &mut app)?;

    loop {
        match events.next()? {
            Event::Input(key) => {
                app.on_key(key);
            },
            Event::Mouse(m) => {
                app.on_mouse(m);
            }
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }

        ui::draw(&mut terminal, &mut app)?;
    }

    Ok(())
}
