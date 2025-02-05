pub mod args;

use crate::config::Config;

impl args::Args {
    pub fn overwrite(self, config: &mut Config) {
        if let Some(color) = self.color {
            config.general.color = color;
        }
        if let Some(interval) = self.interval {
            config.general.interval = interval;
        }
        if let Some(x_pos) = self.x_pos {
            config.position.x = x_pos;
        }
        if let Some(y_pos) = self.y_pos {
            config.position.y = y_pos;
        }
        if self.blink {
            config.general.blink = true;
        }
        if self.bold {
            config.general.bold = true;
        }
        if let Some(fmt) = self.fmt {
            config.date.fmt = fmt;
        }
        if self.use_12h {
            config.date.use_12h = true;
        }
        if self.utc {
            config.date.utc = true;
        }
        if self.hide_seconds {
            config.date.hide_seconds = true;
        }
    }
}
