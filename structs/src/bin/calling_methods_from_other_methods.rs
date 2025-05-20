#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Years Since Release: {}", self.years_since_release());
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn double_length(&mut self) {
        self.duration_secs = self.duration_secs * 2;
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        2024 - self.release_year
    }
}

fn main() {
    let blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    blank_space.display_song_info();
}
