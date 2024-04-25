use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{io::{stdout, Write}, thread, time::Duration};

pub fn monitor_tasks() {
    let mut stdout = stdout();
    loop {
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        // Mock task status output
        println!("Task 1: In Progress");
        println!("Task 2: Completed");

        stdout.flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
