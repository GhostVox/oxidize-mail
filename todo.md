# Rust Email Client Development Roadmap

## Phase 1: Foundation (Weeks 1-2)
**Goal: Basic GTK app that can connect to email servers**

### Week 1: Project Setup
- Set up Rust project with GTK4, webkit2gtk, tokio dependencies
- Create basic GTK window with three-pane layout (sidebar, list, content)
- Implement basic UI structure with headerbar
- Add CSS styling for clean macOS-like appearance

### Week 2: Email Protocol Basics
- Implement IMAP connection using `imap` crate
- Add OAuth2 authentication for Gmail (start with app passwords for testing)
- Create basic email fetching (inbox only, plain text)
- Display email list in middle pane

## Phase 2: Core Email Features (Weeks 3-5)
**Goal: Functional email reading**

### Week 3: Email Parsing & Display
- Integrate `mail-parser` for proper MIME handling
- Add HTML email display using WebKit2GTK
- Implement email header parsing (From, To, Subject, Date)
- Add email content switching (plain text/HTML toggle)

### Week 4: Email Management
- Add folder support (Inbox, Sent, Drafts, Custom folders)
- Implement email marking (read/unread, flagged)
- Add email search functionality
- Basic keyboard shortcuts (j/k for navigation, etc.)

### Week 5: Polish & UX
- Improve email list UI (sender avatars, preview text)
- Add loading states and progress indicators
- Implement proper error handling with user-friendly messages
- Add dark mode support

## Phase 3: Sending & Composition (Weeks 6-7)
**Goal: Full email client functionality**

### Week 6: Email Composition
- Create compose window with rich text editor
- Implement SMTP sending using `lettre` crate
- Add reply/reply-all/forward functionality
- Basic attachment support

### Week 7: Advanced Composition
- Add contact autocomplete
- Implement draft saving
- Add email signatures
- HTML composition support

## Phase 4: Advanced Features (Weeks 8-10)
**Goal: Power user features**

### Week 8: Performance & Caching
- Implement local email caching/database (maybe `sled` or SQLite)
- Add offline reading support
- Optimize for large mailboxes
- Background sync

### Week 9: Multi-Account & Providers
- Add support for multiple email accounts
- Implement other providers (Outlook, Yahoo, custom IMAP)
- Account management UI
- Unified inbox view

### Week 10: Linux Integration
- Desktop notifications for new emails
- System tray integration with unread count
- Integration with desktop search
- Proper .desktop file and icon

## Phase 5: Distribution & Polish (Weeks 11-12)
**Goal: Ready for daily use**

### Week 11: Packaging
- Create Flatpak package
- Add to AUR (Arch User Repository)
- AppImage for universal Linux distribution
- Basic documentation/README

### Week 12: macOS Support
- Test and fix macOS-specific issues
- Create macOS installer/bundle
- Homebrew formula (optional)

## Bonus Features (Future)
- PGP encryption support
- Custom themes/CSS
- Plugin system
- Calendar integration
- Advanced filtering rules

## Tech Stack Summary
- **GUI:** GTK4 + webkit2gtk
- **Networking:** tokio + imap + lettre
- **Parsing:** mail-parser
- **Storage:** sled or rusqlite
- **Auth:** oauth2 crate

## Timeline Overview
- **Phase 1-2 (5 weeks):** Basic functional email reader
- **Phase 3 (2 weeks):** Full email client with sending
- **Phase 4 (3 weeks):** Advanced features and multi-account
- **Phase 5 (2 weeks):** Distribution ready

**Total estimated time:** ~3 months for a fully functional email client that'll blow Thunderbird out of the water!

## Getting Started
Start with Phase 1 and see how it feels - you might find some phases go faster or slower than expected. The key is building incrementally so you have a working email client early on that you can actually use and iterate on.
