use clap::{App, Arg};
use gh_emoji;

fn main() {
    let matches = App::new("SpotifyWhat")
        .version("0.1.0")
        .author("David B. <david@davidbuckley.ca>")
        .about("Transform emoji shortcodes into actual emoji")
        .arg(
            Arg::new("shortcode")
                .about("The shortcode of an emoji")
                .index(1)
                .required(true),
        )
        .get_matches();

    if let Some(shortcode) = matches.value_of("shortcode") {
        if let Some(emoji) = gh_emoji::get(shortcode) {
            print!("{}", emoji);
        }
    }
}
