# Oxidize Mail - Project Dependencies

## Core GUI Framework
```toml
gtk = "0.18"                    # Main GUI toolkit - windows, widgets, event handling
gdk = "0.18"                    # Low-level graphics/input handling for GTK
glib = "0.18"                   # Event loops, signals, utilities for GTK
webkit2gtk = "2.0"              # HTML email rendering engine
```

## Email Protocol Libraries
```toml
async-imap = "0.9"              # IMAP client for receiving emails
lettre = "0.11"                 # SMTP client for sending emails
mail-parser = "0.6"             # Parse email MIME structure and headers
```

## Authentication & Security
```toml
oauth2 = "4.4"                  # OAuth2 authentication for Gmail/Outlook
rustls = "0.21"                 # TLS encryption for secure connections
rustls-native-certs = "0.6"     # System certificate store integration
```

## Async Runtime & Networking
```toml
tokio = { version = "1.0", features = ["full"] }  # Async runtime for network operations
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }  # HTTP client for OAuth
```

## Database & Storage
```toml
rusqlite = { version = "0.29", features = ["bundled", "chrono"] }  # Local email cache database
serde = { version = "1.0", features = ["derive"] }  # Serialization for config/cache
serde_json = "1.0"              # JSON serialization for settings
```

## Date/Time Handling
```toml
chrono = { version = "0.4", features = ["serde"] }  # Date/time parsing and formatting
```

## Text Processing & Search
```toml
regex = "1.9"                   # Email address validation, text processing
html2text = "0.6"               # Convert HTML emails to plain text preview
pulldown-cmark = "0.9"          # Markdown parsing for compose window
```

## Configuration & Logging
```toml
config = "0.13"                 # Configuration file handling (TOML/JSON)
log = "0.4"                     # Logging framework
env_logger = "0.10"             # Environment-based logger implementation
```

## Utilities
```toml
uuid = { version = "1.4", features = ["v4"] }  # Generate unique message IDs
base64 = "0.21"                 # Encode/decode email attachments
mime = "0.3"                    # MIME type handling for attachments
url = "2.4"                     # URL parsing for links in emails
```

## Development & Testing
```toml
[dev-dependencies]
tokio-test = "0.4"              # Testing utilities for async code
tempfile = "3.7"                # Temporary files for testing
mockall = "0.11"                # Mocking for unit tests
```

## Optional Advanced Features
```toml
# Full-text search (alternative to SQLite FTS)
tantivy = "0.20"                # High-performance search engine

# Rich text editing
gtk4-sourceview = "0.7"         # Code/text editor widget for compose

# Notifications
notify-rust = "4.8"             # Desktop notifications for new emails

# Keyring integration
secret-service = "3.0"          # Store passwords in system keyring

# Image handling for attachments
image = "0.24"                  # Image processing and thumbnails
```

## System Dependencies (via package manager)
```bash
# Arch Linux
sudo pacman -S gtk4 webkit2gtk-4.1 openssl pkg-config sqlite

# Ubuntu/Debian
sudo apt install libgtk-4-dev libwebkit2gtk-4.1-dev libssl-dev pkg-config libsqlite3-dev

# macOS (via Homebrew)
brew install gtk4 webkitgtk pkg-config openssl sqlite
```

## Cargo.toml Structure
```toml
[package]
name = "oxidize-mail"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A fast, clean email client built with Rust and GTK"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/oxidize-mail"

[dependencies]
# Core dependencies listed above...

[features]
default = ["notifications", "keyring"]
notifications = ["notify-rust"]
keyring = ["secret-service"]
search = ["tantivy"]
```

## Purpose Summary by Category

**GUI (gtk, webkit2gtk)** - The foundation for the entire user interface
**Email Protocols (imap, lettre)** - Core functionality for sending/receiving emails
**Security (oauth2, rustls)** - Safe authentication and encrypted connections
**Database (rusqlite)** - Local email caching for offline access and fast search
**Async (tokio)** - Non-blocking network operations to keep UI responsive
**Parsing (mail-parser, chrono)** - Handle complex email formats and timestamps
**Utilities** - Quality of life features like search, notifications, config management
