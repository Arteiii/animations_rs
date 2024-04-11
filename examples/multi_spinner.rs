use std::thread::sleep;
use std::time::Duration;

use unicode_icons::icons::symbols::{check_mark_button, cross_mark};

use zenity::spinner::Frames;
use zenity::spinner::MultiSpinner;

fn main() {
    {
        let check_mark_text = check_mark_button();
        let cross_mark_text = cross_mark();

        let spinner = MultiSpinner::new(Frames::dot_spinner11());
        let spinner1 = spinner.get_last();

        // main thread operations
        let spinner2 = spinner.add(Frames::binary());
        let spinner3 = spinner.add(Frames::dot_spinner9());
        let spinner4 = spinner.add(Frames::dot_spinner8());

        spinner.run_all();

        sleep(Duration::from_secs(4));
        spinner.set_text(&spinner2, "  spinner2".to_string());
        // stop spinner1
        spinner.set_text(&spinner1, "  spinner1".to_string());

        sleep(Duration::from_secs(2));

        spinner.stop(&spinner2);
        spinner.set_text(&spinner2, format!("  {}   Successfully", &check_mark_text));

        sleep(Duration::from_secs(2));
        spinner.set_text(&spinner1, "  spinner1 stopped".to_string());

        spinner.stop(&spinner1);

        sleep(Duration::from_secs(9));
        spinner.stop(&spinner3);
        spinner.stop(&spinner4);

        spinner.set_text(&spinner3, format!("  {}   Failed!", &cross_mark_text));
        spinner.set_text(&spinner4, format!("   {}   Failed!", &cross_mark_text));

        sleep(Duration::from_secs(1));
    }

    println!("animation stopped");

    sleep(Duration::from_secs(5));
}
