#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn double_length(&mut self) {
        self.duration_secs = self.duration_secs * 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }
}

fn main() {
    let blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    let all_too_well = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration_secs: 327,
    };

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "{} is longer than {}.",
            blank_space.title, all_too_well.title
        );
    } else {
        println!(
            "{} is shorter than or equal to {}.",
            blank_space.title, all_too_well.title
        );
    }
}
