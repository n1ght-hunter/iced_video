//! This module contains the SVGs used in the application.
use iced::widget::svg;

/// The play svg.
pub fn play_svg() -> svg::Handle {
    svg::Handle::from_memory("
    <svg xmlns='http://www.w3.org/2000/svg' fill='white' class='bi bi-play-fill' viewBox='0 0 16 16'>
        <path d='m11.6 8.7-6.37 3.69a.82.82 0 0 1-1.23-.7V4.31c0-.63.7-1.01 1.23-.7l6.37 3.7a.8.8 0 0 1 0 1.39z' />
    </svg>
    ".as_bytes())
}

/// low volume svg.
pub fn low_volume_svg() -> svg::Handle {
    svg::Handle::from_memory("
    <svg xmlns='http://www.w3.org/2000/svg' fill='white' class='bi bi-volume-off-fill' viewBox='0 0 16 16'>
        <path d='M10.72 3.55A.5.5 0 0 1 11 4v8a.5.5 0 0 1-.81.39L7.83 10.5H5.5A.5.5 0 0 1 5 10V6a.5.5 0 0 1 .5-.5h2.33l2.36-1.89a.5.5 0 0 1 .53-.06z' />
    </svg>
    ".as_bytes())
}

/// medium volume svg.
pub fn medium_volume_svg() -> svg::Handle {
    svg::Handle::from_memory("
    <svg xmlns='http://www.w3.org/2000/svg' fill='white' class='bi bi-volume-down-fill' viewBox='0 0 16 16'>
        <path d='M9 4a.5.5 0 0 0-.81-.39L5.83 5.5H3.5A.5.5 0 0 0 3 6v4a.5.5 0 0 0 .5.5h2.33l2.36 1.89A.5.5 0 0 0 9 12V4zm3.02 4a4.49 4.49 0 0 1-1.31 3.18l-.71-.7A3.49 3.49 0 0 0 11.03 8 3.49 3.49 0 0 0 10 5.53l.7-.71A4.49 4.49 0 0 1 12.04 8z'/>
    </svg>
    ".as_bytes())
}

/// high volume svg.
pub fn high_volume_svg() -> svg::Handle {
    svg::Handle::from_memory("
    <svg xmlns='http://www.w3.org/2000/svg' fill='white' class='bi bi-volume-up-fill' viewBox='0 0 16 16'>
        <path d='M11.54 14.01A8.47 8.47 0 0 0 14.03 8a8.47 8.47 0 0 0-2.5-6.01l-.7.7A7.48 7.48 0 0 1 13.03 8c0 2.07-.84 3.95-2.2 5.3l.7.71z' />
        <path d='M10.12 12.6a6.48 6.48 0 0 0 1.9-4.6 6.48 6.48 0 0 0-1.9-4.6l-.7.71A5.48 5.48 0 0 1 11.01 8a5.48 5.48 0 0 1-1.6 3.89l.7.7z' />
        <path d='M8.7 11.18A4.49 4.49 0 0 0 10.04 8 4.49 4.49 0 0 0 8.7 4.82l-.71.7A3.49 3.49 0 0 1 9.03 8 3.49 3.49 0 0 1 8 10.47l.7.71zM6.73 3.55A.5.5 0 0 1 7 4v8a.5.5 0 0 1-.81.39L3.82 10.5H1.5A.5.5 0 0 1 1 10V6a.5.5 0 0 1 .5-.5h2.33l2.36-1.89a.5.5 0 0 1 .53-.06z' />
    </svg>
    ".as_bytes())
}

/// The pause svg.
pub fn pause_svg() -> svg::Handle {
    svg::Handle::from_memory("
    <svg xmlns='http://www.w3.org/2000/svg' fill='white' class='bi bi-pause-fill' viewBox='0 0 16 16'>
        <path d='M5.5 3.5A1.5 1.5 0 0 1 7 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5zm5 0A1.5 1.5 0 0 1 12 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5z'/>
    </svg>
    ".as_bytes())
}

/// The mute svg.   
pub fn muted_svg() -> svg::Handle {
    svg::Handle::from_memory("
    <svg xmlns='http://www.w3.org/2000/svg' fill='white' class='bi bi-volume-mute-fill' viewBox='0 0 16 16'>
        <path d='M6.72 3.55A.5.5 0 0 1 7 4v8a.5.5 0 0 1-.81.39L3.82 10.5H1.5A.5.5 0 0 1 1 10V6a.5.5 0 0 1 .5-.5h2.33l2.36-1.89a.5.5 0 0 1 .53-.06zm7.13 2.1a.5.5 0 0 1 0 .7L12.21 8l1.64 1.65a.5.5 0 0 1-.7.7L11.5 8.71l-1.65 1.64a.5.5 0 0 1-.7-.7L10.79 8 9.15 6.35a.5.5 0 1 1 .7-.7l1.65 1.64 1.65-1.64a.5.5 0 0 1 .7 0z'/>
    </svg>
    ".as_bytes())
}
