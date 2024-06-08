use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crossterm::{cursor, execute, terminal};

fn main() {
    let spinner = ["⣯", "⣷", "⣾", "⣽", "⣻", "⢿", "⡿", "⣟"];
    let phases = [
        ("Fetching data...", "Data fetched"),
        ("Computing...", "Computation complete"),
        ("Loading modules...", "Modules loaded"),
        ("Optimizing performance...", "Optimization complete"),
        ("Finalizing setup...", "Setup complete"),
    ];
    let stdout = stdout();
    let mut handle = stdout.lock();

    execute!(handle, terminal::Clear(terminal::ClearType::All)).unwrap();

    for phase in phases.iter() {
        for _ in 0..1 {
            for symbol in spinner.iter() {
                execute!(
                    handle,
                    cursor::MoveTo(0, phases.iter().position(|&p| p == *phase).unwrap() as u16),
                    terminal::Clear(terminal::ClearType::CurrentLine)
                )
                .unwrap();
                print!("{}: {}", symbol, phase.0);
                handle.flush().unwrap();
                sleep(Duration::from_millis(100));
            }
        }
        execute!(
            handle,
            cursor::MoveTo(0, phases.iter().position(|&p| p == *phase).unwrap() as u16),
            terminal::Clear(terminal::ClearType::CurrentLine)
        )
        .unwrap();
        println!("   {}", phase.1);
    }

    println!("All phases completed!");
}
