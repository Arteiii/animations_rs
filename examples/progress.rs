use std::thread;
use std::time::Duration;

use rand::Rng;

use zenity::progress::{Bar, ProgressBar, ProgressBarFrames};

fn main() {
    {
        let progress = ProgressBar::default();

        let loading = 1_usize;

        for loading in loading..=100 {
            thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(1..=70)));
            progress.set(&progress.get_last(), &loading);
        }
    }
    
    multiple();
    println!("test line ending")
}

fn multiple() {
    println!("multiple progressbar");

    let mut progress = ProgressBar::new();

    let progress1 = progress.add(Bar::new(ProgressBarFrames::rect()).set_goal(253));
    let progress2 = progress.add(Bar::new(ProgressBarFrames::equal()).set_goal(253).set_size(7));
    let progress3 = progress.add(Bar::new(ProgressBarFrames::hash()).set_goal(253).set_size(60));

    progress.run_all();

    let loading = 1_usize;

    for loading in loading..=253 {
        progress.set(&progress1, &loading);
        progress.set(&progress2, &loading);
        progress.set(&progress3, &loading);

        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(1..=70)));
    }
}