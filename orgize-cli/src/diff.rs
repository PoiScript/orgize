use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use clap::builder::styling::{AnsiColor, Color, Style};

pub fn print(orgi: &str, mut patches: Vec<(usize, usize, String)>) {
    patches.sort_by(|a, b| a.0.cmp(&b.0));

    let mut off = 0;

    for (start, end, content) in patches {
        print!("{}", &orgi[off..(start)]);

        if orgi[start..end] != content {
            let style = Style::new().fg_color(Color::Ansi(AnsiColor::Cyan).into());
            print!("{}{}{}", style.render(), &content, style.render_reset());
        } else {
            print!("{}", &content);
        }

        off = end;
    }

    print!("{}", &orgi[off..]);
}

pub fn write_to_file(
    orgi: &str,
    mut patches: Vec<(usize, usize, String)>,
    path: PathBuf,
) -> anyhow::Result<()> {
    patches.sort_by(|a, b| a.0.cmp(&b.0));

    let file = &mut OpenOptions::new().write(true).open(path)?;
    let mut off = 0;

    for (start, end, content) in patches {
        write!(file, "{}{}", &orgi[off..start], &content)?;
        off = end;
    }

    write!(file, "{}", &orgi[off..])?;

    Ok(())
}
