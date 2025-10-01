use crate::colors::*;
use egui::Color32;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;

pub struct IconInfo {
    pub icon: &'static str,
    pub color: Color32,
}

static ICONS_BY_FILENAME: Lazy<HashMap<&'static str, IconInfo>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "cargo.toml",
        IconInfo {
            icon: " ",
            color: LIGHT1,
        },
    );
    m.insert(
        "license",
        IconInfo {
            icon: " ",
            color: LIGHT1,
        },
    );
    m
});

static ICONS_BY_EXTENSION: Lazy<HashMap<&'static str, IconInfo>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "md",
        IconInfo {
            icon: " ",
            color: NEUTRAL_BLUE,
        },
    );
    m.insert(
        "rs",
        IconInfo {
            icon: " ",
            color: NEUTRAL_RED,
        },
    );
    m.insert(
        "toml",
        IconInfo {
            icon: " ",
            color: LIGHT1,
        },
    );
    m
});

static DEFAULT_FILE_ICON: IconInfo = IconInfo {
    icon: " ",
    color: LIGHT1,
};

static DEFAULT_FOLDER_ICON: IconInfo = IconInfo {
    icon: " ",
    color: NEUTRAL_GREEN,
};

static DEFAULT_FOLDER_OPEN_ICON: IconInfo = IconInfo {
    icon: " ",
    color: NEUTRAL_GREEN,
};

pub fn get_icon_for_file(filename: &str) -> &'static IconInfo {
    let lowercase_filename = filename.to_lowercase();
    if let Some(icon) = ICONS_BY_FILENAME.get(lowercase_filename.as_str()) {
        return icon;
    }

    let path = Path::new(filename);
    if let Some(ext_os) = path.extension() {
        if let Some(ext) = ext_os.to_str() {
            if let Some(icon) = ICONS_BY_EXTENSION.get(ext) {
                return icon;
            }
        }
    }

    &DEFAULT_FILE_ICON
}

pub fn get_icon_for_folder(is_open: bool) -> &'static IconInfo {
    if is_open {
        &DEFAULT_FOLDER_OPEN_ICON
    } else {
        &DEFAULT_FOLDER_ICON
    }
}