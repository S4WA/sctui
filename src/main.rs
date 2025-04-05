use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem},
    layout::{Layout, Constraint, Direction},
    Terminal,
};
use rsoundcloud::{SoundCloudClient, ResourceId, TracksApi};
use std::{error::Error, io::stdout};
use tokio;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
mod appconfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create Config
    let app_config = appconfig::init()?;

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize SoundCloud client
    let client = SoundCloudClient::new(
        Some(app_config.get_client_id().to_string()),
        None
    ).await?;

    // Fetch track data
    let track = client
        .get_track(ResourceId::Url(
            "https://soundcloud.com/wolverave/wolverave-vielleicht-vielleicht-hardtekk-edit-1".to_string()
        ))
        .await?;

    // Create user info list
    let user_info = vec![
        ListItem::new(format!("Username: {}", track.user.username)),
        ListItem::new(format!("Full Name: {}", track.user.full_name)),
        ListItem::new(format!("Followers: {}", track.user.followers_count)),
        ListItem::new(format!("City: {:?}", track.user.city)),
        ListItem::new(format!("Country: {:?}", track.user.country_code)),
        ListItem::new(format!("Permalink: {}", track.user.permalink_url)),
    ];

    // Create track info list
    let track_info = vec![
        ListItem::new(format!("Track Title: {}", track.track.title)),
        ListItem::new(format!("Genre: {}", track.track.genre.unwrap_or_else(|| "Unknown".to_string()))),
        ListItem::new(format!("Likes: {}", track.track.likes_count.unwrap_or(0))),
        ListItem::new(format!("Reposts: {}", track.track.reposts_count.unwrap_or(0))),
        ListItem::new(format!("Playback Count: {}", track.track.playback_count.unwrap_or(0))),
        ListItem::new(format!("Description: {}", track.track.description.unwrap_or_else(|| "No description available".to_string()))),
        ListItem::new(format!("Purchase Link: {}", track.track.purchase_url.unwrap_or_else(|| "No purchase link".to_string()))),
    ];

    // Main UI loop
    loop {
        terminal.draw(|f| {
            // Split the screen vertically into two chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(f.area());

            // Create and render user info widget
            let user_list = List::new(user_info.clone())
                .block(Block::default().title("User Info").borders(Borders::ALL));
            f.render_widget(user_list, chunks[0]);

            // Create and render track info widget
            let track_list = List::new(track_info.clone())
                .block(Block::default().title("Track Info").borders(Borders::ALL));
            f.render_widget(track_list, chunks[1]);
        })?;

        // Handle keyboard input (press 'q' to quit)
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}