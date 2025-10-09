# GTK4 UI Development Guide for Oxidize Mail

## Table of Contents
1. [GTK4 Fundamentals](#gtk4-fundamentals)
2. [Widget Hierarchy](#widget-hierarchy)
3. [Layout System](#layout-system)
4. [Building the Three-Pane Layout](#building-the-three-pane-layout)
5. [Common Patterns](#common-patterns)
6. [Styling and CSS](#styling-and-css)
7. [Event Handling](#event-handling)
8. [Best Practices](#best-practices)

---

## GTK4 Fundamentals

### The Widget Tree
Every GTK4 application is built as a **tree of widgets**. Think of it like a family tree:
- **Root**: `Application` (your app)
- **Trunk**: `ApplicationWindow` (main window)
- **Branches**: Container widgets (`Box`, `Paned`, `ScrolledWindow`)
- **Leaves**: Display widgets (`Label`, `Button`, `ListBox`)

### Core Concepts

#### 1. Widgets
Everything you see is a widget. Widgets can be:
- **Containers**: Hold other widgets (`Box`, `Paned`, `Grid`)
- **Display**: Show content (`Label`, `Image`, `TextView`)
- **Interactive**: Respond to input (`Button`, `Entry`, `Switch`)
- **Hybrid**: Both contain and display (`ListBox`, `TreeView`)

#### 2. Parent-Child Relationships
```rust
let parent = Box::new(Orientation::Vertical, 0);
let child = Label::new(Some("Hello"));
parent.append(&child); // child is now inside parent
```

#### 3. The Prelude
```rust
use gtk4::prelude::*;
```
This imports all the trait methods that make GTK4 work. Without it, you can't call methods like `.append()`, `.set_child()`, etc.

---

## Widget Hierarchy

### Our Email Client Structure
```
ApplicationWindow
└── HeaderBar (titlebar)
└── Paned (horizontal) ← main_paned
    ├── Box (vertical) ← Folder Sidebar
    │   └── ScrolledWindow
    │       └── ListBox (folders)
    └── Paned (horizontal) ← content_paned
        ├── Box (vertical) ← Email List
        │   └── ScrolledWindow
        │       └── ListBox (emails)
        └── Box (vertical) ← Email Viewer
            ├── Label (subject)
            ├── Label (from)
            ├── Label (date)
            └── ScrolledWindow
                └── Label (body)
```

### Why This Structure?

**Paned Widgets**: Allow users to drag and resize sections
- Main `Paned`: Splits sidebar from content (left/right)
- Content `Paned`: Splits email list from viewer (left/right)

**ScrolledWindow**: Makes content scrollable when it's too large
- Always put long lists or text inside `ScrolledWindow`

**Box**: Groups widgets together in a direction
- `Vertical`: Stacks widgets top-to-bottom
- `Horizontal`: Places widgets left-to-right

---

## Layout System

### Understanding Orientation

```rust
// Vertical Box - stacks children from top to bottom
let vbox = Box::new(Orientation::Vertical, spacing);
vbox.append(&widget1); // Top
vbox.append(&widget2); // Below widget1
vbox.append(&widget3); // Below widget2

// Horizontal Box - places children left to right
let hbox = Box::new(Orientation::Horizontal, spacing);
hbox.append(&widget1); // Left
hbox.append(&widget2); // Right of widget1
hbox.append(&widget3); // Right of widget2
```

### Expansion: The Key to Layout

GTK4 uses **expansion** to determine how widgets grow:

```rust
widget.set_hexpand(true);  // Expand horizontally
widget.set_vexpand(true);  // Expand vertically
```

**Without expansion**:
- Widgets use their minimum size
- Empty space remains unused
- UI looks cramped

**With expansion**:
- Widgets fill available space
- UI looks balanced and professional

#### Rule of Thumb
- **Containers** (Box, ScrolledWindow): Should expand in both directions
- **Content** (Label, Button): Only expand if you want them to fill space

### Example: Why Expansion Matters

```rust
// ❌ BAD - Won't fill the window
let sidebar = Box::new(Orientation::Vertical, 0);
let scrolled = ScrolledWindow::new();
scrolled.set_child(Some(&listbox));
sidebar.append(&scrolled);

// ✅ GOOD - Fills available space
let sidebar = Box::new(Orientation::Vertical, 0);
sidebar.set_vexpand(true);  // Fill vertical space
sidebar.set_hexpand(true);  // Fill horizontal space

let scrolled = ScrolledWindow::new();
scrolled.set_vexpand(true);  // Fill vertical space
scrolled.set_hexpand(true);  // Fill horizontal space
scrolled.set_child(Some(&listbox));
sidebar.append(&scrolled);
```

---

## Building the Three-Pane Layout

### Step 1: Create the Main Window

```rust
let window = ApplicationWindow::builder()
    .application(app)
    .title("Oxidize Mail")
    .default_width(1200)   // Initial width
    .default_height(800)   // Initial height
    .build();
```

**Builder Pattern**: GTK4 uses builders to create widgets with multiple properties.

### Step 2: Add a Header Bar

```rust
let header = HeaderBar::new();
header.set_show_title_buttons(true);  // Show minimize/maximize/close
window.set_titlebar(Some(&header));   // Attach to window
```

**HeaderBar**: Modern title bar that can contain buttons and controls.

### Step 3: Create the Layout Structure

```rust
// Main split: Sidebar | Content
let main_paned = Paned::new(Orientation::Horizontal);

// Content split: Email List | Email Viewer
let content_paned = Paned::new(Orientation::Horizontal);

// Nest them
main_paned.set_start_child(Some(&folder_sidebar));
main_paned.set_end_child(Some(&content_paned));

content_paned.set_start_child(Some(&email_list));
content_paned.set_end_child(Some(&email_viewer));
```

**Paned Methods**:
- `set_start_child()`: Left side (horizontal) or top (vertical)
- `set_end_child()`: Right side (horizontal) or bottom (vertical)
- `set_position()`: Initial divider position in pixels
- `set_resize_start_child()`: Allow start child to resize
- `set_shrink_start_child()`: Allow start child to shrink below minimum

### Step 4: Configure Paned Behavior

```rust
main_paned.set_position(200);           // Sidebar is 200px wide
main_paned.set_resize_start_child(false);  // Sidebar fixed size
main_paned.set_shrink_start_child(false);  // Can't shrink below minimum

content_paned.set_position(400);        // Email list is 400px wide
content_paned.set_resize_start_child(true);   // List can resize
content_paned.set_resize_end_child(true);     // Viewer can resize
```

---

## Common Patterns

### Pattern 1: Scrollable List

```rust
fn create_scrollable_list() -> Box {
    // Container
    let container = Box::new(Orientation::Vertical, 0);
    container.set_vexpand(true);
    container.set_hexpand(true);

    // Scrolled window
    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(
        PolicyType::Never,      // No horizontal scrollbar
        PolicyType::Automatic   // Vertical scrollbar when needed
    );
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    // List
    let listbox = ListBox::new();

    // Add items
    for i in 0..100 {
        let label = Label::new(Some(&format!("Item {}", i)));
        listbox.append(&label);
    }

    // Assemble
    scrolled.set_child(Some(&listbox));
    container.append(&scrolled);

    container
}
```

**ScrolledWindow Policies**:
- `Never`: Never show scrollbar
- `Always`: Always show scrollbar
- `Automatic`: Show when content overflows

### Pattern 2: Form with Labels and Inputs

```rust
use gtk4::{Entry, Grid};

fn create_form() -> Grid {
    let grid = Grid::new();
    grid.set_row_spacing(12);
    grid.set_column_spacing(12);

    // Row 0
    grid.attach(&Label::new(Some("Name:")), 0, 0, 1, 1);
    grid.attach(&Entry::new(), 1, 0, 1, 1);

    // Row 1
    grid.attach(&Label::new(Some("Email:")), 0, 1, 1, 1);
    grid.attach(&Entry::new(), 1, 1, 1, 1);

    grid
}
```

**Grid**: Table-like layout
- `attach(widget, column, row, width, height)`

### Pattern 3: Button Bar

```rust
use gtk4::Button;

fn create_button_bar() -> Box {
    let button_bar = Box::new(Orientation::Horizontal, 6);
    button_bar.set_margin_top(12);
    button_bar.set_margin_bottom(12);
    button_bar.set_halign(gtk4::Align::End);  // Align right

    let cancel = Button::with_label("Cancel");
    let save = Button::with_label("Save");
    save.add_css_class("suggested-action");  // Blue button

    button_bar.append(&cancel);
    button_bar.append(&save);

    button_bar
}
```

### Pattern 4: Detail View with Header

```rust
fn create_detail_view() -> Box {
    let detail = Box::new(Orientation::Vertical, 12);
    detail.set_margin_start(20);
    detail.set_margin_end(20);
    detail.set_margin_top(20);
    detail.set_margin_bottom(20);

    // Header section
    let title = Label::new(Some("Title"));
    title.set_halign(gtk4::Align::Start);
    title.add_css_class("title-1");  // Large text

    let subtitle = Label::new(Some("Subtitle"));
    subtitle.set_halign(gtk4::Align::Start);
    subtitle.add_css_class("dim-label");  // Dimmed text

    // Content section
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

    let content = Label::new(Some("Content..."));
    content.set_wrap(true);
    scrolled.set_child(Some(&content));

    // Assemble
    detail.append(&title);
    detail.append(&subtitle);
    detail.append(&scrolled);

    detail
}
```

---

## Styling and CSS

### Built-in CSS Classes

GTK4 includes many predefined CSS classes:

**Text Styles**:
- `title-1` to `title-4`: Heading sizes
- `heading`: Section heading
- `caption`: Small text
- `dim-label`: Dimmed/secondary text
- `monospace`: Monospace font

**Button Styles**:
- `suggested-action`: Blue (primary action)
- `destructive-action`: Red (delete/remove)
- `flat`: No border/background

**Container Styles**:
- `navigation-sidebar`: Sidebar appearance
- `toolbar`: Toolbar appearance
- `card`: Card-like container

### Applying CSS Classes

```rust
widget.add_css_class("title-1");
widget.add_css_class("suggested-action");
widget.remove_css_class("flat");
```

### Custom CSS (Advanced)

```rust
use gtk4::CssProvider;

let provider = CssProvider::new();
provider.load_from_data(r#"
    .email-row {
        padding: 12px;
        border-bottom: 1px solid @borders;
    }

    .email-row:hover {
        background: @headerbar_bg_color;
    }
"#);

gtk4::style_context_add_provider_for_display(
    &window.display(),
    &provider,
    gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
);
```

---

## Event Handling

### Signals and Callbacks

GTK4 uses **signals** for events. Connect callbacks with `.connect_*` methods:

```rust
use gtk4::Button;

let button = Button::with_label("Click Me");

button.connect_clicked(|btn| {
    btn.set_label("Clicked!");
    println!("Button was clicked");
});
```

### Common Signals

**Button**:
- `connect_clicked`: Button was clicked

**ListBox**:
- `connect_row_activated`: Row was double-clicked or Enter pressed
- `connect_row_selected`: Selected row changed

**Entry**:
- `connect_activate`: Enter key pressed
- `connect_changed`: Text changed

**Window**:
- `connect_close_request`: Window is closing

### Accessing External State

Use Rust's `Rc` and `RefCell` for shared mutable state:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let counter = Rc::new(RefCell::new(0));
let label = Label::new(Some("0"));

let counter_clone = counter.clone();
let label_clone = label.clone();

button.connect_clicked(move |_| {
    *counter_clone.borrow_mut() += 1;
    label_clone.set_label(&counter_clone.borrow().to_string());
});
```

### Example: Email List Selection

```rust
fn create_email_list_with_selection() -> (Box, ListBox) {
    let container = Box::new(Orientation::Vertical, 0);
    container.set_vexpand(true);

    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

    let listbox = ListBox::new();

    // Add emails
    for i in 1..=20 {
        let email_row = Box::new(Orientation::Vertical, 4);
        let subject = Label::new(Some(&format!("Email {}", i)));
        email_row.append(&subject);
        listbox.append(&email_row);
    }

    // Handle selection
    listbox.connect_row_selected(|_, row| {
        if let Some(row) = row {
            let index = row.index();
            println!("Selected email #{}", index);
            // Update email viewer here
        }
    });

    scrolled.set_child(Some(&listbox));
    container.append(&scrolled);

    (container, listbox)
}
```

---

## Best Practices

### 1. Structure Your Code

Break UI into functions:

```rust
fn build_ui(app: &Application) {
    let window = create_main_window(app);
    let header = create_header();
    let content = create_content();

    window.set_titlebar(Some(&header));
    window.set_child(Some(&content));
    window.present();
}

fn create_header() -> HeaderBar {
    // Header logic
}

fn create_content() -> Box {
    // Content logic
}
```

### 2. Use Builders for Complex Widgets

```rust
// ❌ Verbose
let button = Button::new();
button.set_label("Click");
button.set_width_request(100);
button.set_tooltip_text(Some("Click me"));

// ✅ Clean
let button = Button::builder()
    .label("Click")
    .width_request(100)
    .tooltip_text("Click me")
    .build();
```

### 3. Set Expansion Early

Always set expansion on containers immediately after creation:

```rust
let container = Box::new(Orientation::Vertical, 0);
container.set_vexpand(true);  // Do this right away
container.set_hexpand(true);
```

### 4. Margins and Spacing

Use consistent spacing throughout your UI:

```rust
const SPACING: i32 = 12;
const MARGIN: i32 = 20;

widget.set_margin_start(MARGIN);
widget.set_margin_end(MARGIN);
widget.set_margin_top(MARGIN);
widget.set_margin_bottom(MARGIN);

let container = Box::new(Orientation::Vertical, SPACING);
```

### 5. Debug Layout Issues

If a widget isn't appearing:
1. Check if parent has expansion set
2. Check if widget itself has expansion set (if it should)
3. Check ScrolledWindow policies
4. Verify parent-child relationships

Add temporary borders to debug:
```rust
widget.add_css_class("debug");  // Then define .debug in CSS with a border
```

### 6. Resource Management

GTK4 uses reference counting. Don't worry about manual cleanup:

```rust
{
    let button = Button::new();  // Ref count: 1
    container.append(&button);    // Ref count: 2
    // button ref dropped here, but container still holds it
}
// When container is dropped, button is cleaned up
```

### 7. Performance

- Use `ListBox` for short lists (< 100 items)
- Use `ListView` for long lists (100+ items) - more efficient
- Don't create all widgets upfront - create on demand
- Use `ScrolledWindow` to show only visible items

---

## Quick Reference

### Widget Creation
```rust
Box::new(Orientation::Vertical, spacing)
Button::with_label("Text")
Label::new(Some("Text"))
Entry::new()
ScrolledWindow::new()
Paned::new(Orientation::Horizontal)
ListBox::new()
Grid::new()
```

### Common Methods
```rust
// Adding children
box.append(&child)
grid.attach(&widget, col, row, width, height)
paned.set_start_child(Some(&widget))
scrolled.set_child(Some(&widget))

// Expansion
widget.set_vexpand(true)
widget.set_hexpand(true)

// Alignment
widget.set_halign(gtk4::Align::Start)  // Start, End, Center, Fill
widget.set_valign(gtk4::Align::Start)

// Margins
widget.set_margin_start(12)
widget.set_margin_end(12)
widget.set_margin_top(12)
widget.set_margin_bottom(12)

// Size
widget.set_width_request(200)
widget.set_height_request(100)

// Styling
widget.add_css_class("class-name")
widget.remove_css_class("class-name")
```

---

## Next Steps

Now you can:
1. **Add a toolbar** to the header with compose/reply buttons
2. **Implement email selection** to update the viewer pane
3. **Add a search bar** above the email list
4. **Create a compose dialog** for new emails
5. **Add keyboard shortcuts** with `gtk4::EventControllerKey`

Experiment and build! The best way to learn GTK4 is by doing.
