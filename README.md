# Oxidize-Mail

A fast, clean email client built with Rust and GTK, designed for a seamless and powerful desktop email experience.

Note: This project is currently in the early stages of development. The features and structure described here are based on the development roadmap and may evolve over time.

## 🏛️ Project Structure

Oxidize-Mail is built using a Cargo workspace to promote modularity and clear separation of concerns. This design isolates different parts of the application, making the codebase easier to maintain, test, and develop.

The workspace is organized into the following key crates:

- **crates/oxidize-mail**: This is the main application crate that houses the GTK4 user interface. It brings together all the other components to create the final user-facing client.

- **crates/oxidize-mail-core**: The engine of the email client. This crate is responsible for handling all backend logic, including email protocols like IMAP and SMTP, as well as authentication.

- **crates/oxidize-mail-parser**: Handles the complex task of parsing emails. It processes raw email data, manages MIME types, decodes attachments, and makes sense of various content formats.

- **crates/oxidize-mail-storage**: Manages all aspects of data persistence. This includes the local database, caching of emails for offline access, and the indexing required for fast search functionality.

## ✨ Features (Current & Planned)

The development of Oxidize-Mail is planned in several phases, with the goal of creating a full-featured and user-friendly email client.

### Core Features

- **IMAP & SMTP**: Full support for connecting to email servers to fetch and send mail.
- **HTML & Plain Text Rendering**: Display emails beautifully with a WebKit-based renderer, with an option to toggle to plain text.
- **Multi-Account Support**: Manage all your email accounts from a single, unified interface.
- **Offline Access**: A local cache ensures you can read and manage your emails even when you're not connected to the internet.

### Advanced Functionality

- **Full-Text Search**: Powerful and fast email search powered by Tantivy (optional feature).
- **Desktop Notifications**: Get notified of new emails directly on your desktop.
- **Keyring Integration**: Securely store your account passwords in the system's native keyring.
- **Rich Text Composer**: A modern compose window with support for rich text and HTML.

## 🛠️ Tech Stack

Oxidize-Mail leverages a modern, high-performance tech stack to deliver a responsive and reliable experience.

- **GUI**: GTK4 for the core user interface, with webkit6 for rendering HTML emails.
- **Asynchronous Runtime**: Tokio is used for all networking operations, ensuring the UI remains fluid and responsive.
- **Email Protocols**: async-imap and lettre for handling IMAP and SMTP communication.
- **Database & Storage**: Rusqlite provides a robust local database for caching emails and metadata.
- **Parsing**: mail-parser is used for handling the complexities of email MIME structures.

## 🚀 Building from Source

### Prerequisites

Before you begin, you'll need to install the necessary system dependencies for GTK4 and WebKit.

**Arch Linux**:

```bash
sudo pacman -S gtk4 webkit2gtk-4.1 openssl pkg-config sqlite
```

**Ubuntu/Debian**:

```bash
sudo apt install libgtk-4-dev libwebkit2gtk-4.1-dev libssl-dev pkg-config libsqlite3-dev
```

**macOS (via Homebrew)**:

```bash
brew install gtk4 webkitgtk pkg-config openssl sqlite
```

### Build Steps

1. Clone the repository:

```bash
git clone https://github.com/GhostVox/oxidize-mail.git
cd oxidize-mail
```
2. Build the project:

Since this is a workspace, you can build all the crates at once.

```bash
cargo build --workspace
```

3. Run the application:

To run the main GUI application, specify the oxidize-mail package.
```bash
cargo run -p oxidize-mail
```

## 🗺️ Development Roadmap

The project is structured into several development phases, starting with foundational features and progressing to advanced functionality and packaging. The goal is to have a functional and usable email client early in the process and iterate from there. You can follow the detailed progress in the todo.md file.
