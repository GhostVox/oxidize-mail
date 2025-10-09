# GTK4 Complete Reference Guide

## GTK Elements (Widgets)

### Container Widgets
Widgets that hold other widgets and control layout.

#### Box
Container that arranges children in a single row or column.
```rust
Box::new(Orientation::Vertical, spacing)
Box::new(Orientation::Horizontal, spacing)
```
- **Methods**: `append()`, `prepend()`, `remove()`
- **Use case**: Arranging widgets in a line

#### Grid
Container that arranges children in rows and columns (table layout).
```rust
Grid::new()
grid.attach(&widget, column, row, width, height)
```
- **Methods**: `attach()`, `attach_next_to()`, `set_row_spacing()`, `set_column_spacing()`
- **Use case**: Forms, calculator layouts

#### Paned
Container with two children separated by an adjustable divider.
```rust
Paned::new(Orientation::Horizontal)
```
- **Methods**: `set_start_child()`, `set_end_child()`, `set_position()`, `set_resize_start_child()`, `set_shrink_start_child()`
- **Use case**: Resizable split panes (like email client layout)

#### ScrolledWindow
Container that makes child scrollable when content is too large.
```rust
ScrolledWindow::new()
```
- **Methods**: `set_child()`, `set_policy()`, `set_min_content_width()`, `set_min_content_height()`
- **Use case**: Long lists, large content

#### Stack
Container that shows one child at a time (like tabs).
```rust
Stack::new()
```
- **Methods**: `add_titled()`, `add_named()`, `set_visible_child_name()`
- **Use case**: Multi-page interfaces, settings panels

#### Frame
Decorative frame around a single child widget.
```rust
Frame::new(Some("Title"))
```
- **Methods**: `set_child()`, `set_label()`
- **Use case**: Grouping related content

#### Overlay
Container that stacks children on top of each other.
```rust
Overlay::new()
```
- **Methods**: `set_child()`, `add_overlay()`
- **Use case**: Floating widgets, badges

---

### Display Widgets
Widgets that show content to the user.

#### Label
Displays non-editable text.
```rust
Label::new(Some("Text"))
```
- **Methods**: `set_text()`, `set_markup()`, `set_wrap()`, `set_selectable()`, `set_halign()`, `set_valign()`
- **Properties**: `label`, `use-markup`, `wrap`, `selectable`
- **Use case**: Displaying text, titles, descriptions

#### Image
Displays images from files, icons, or resources.
```rust
Image::from_file("path.png")
Image::from_icon_name("icon-name")
```
- **Methods**: `set_from_file()`, `set_from_icon_name()`, `set_pixel_size()`
- **Use case**: Icons, photos, graphics

#### Spinner
Animated spinning icon to indicate activity.
```rust
Spinner::new()
```
- **Methods**: `start()`, `stop()`
- **Use case**: Loading indicators

#### ProgressBar
Shows progress of a long-running operation.
```rust
ProgressBar::new()
```
- **Methods**: `set_fraction()`, `set_text()`, `pulse()`
- **Use case**: Download progress, operation status

#### Separator
Visual line to separate content.
```rust
Separator::new(Orientation::Horizontal)
```
- **Use case**: Dividing sections

---

### Interactive Widgets
Widgets that respond to user input.

#### Button
Clickable button.
```rust
Button::with_label("Click Me")
Button::from_icon_name("icon-name")
```
- **Methods**: `set_label()`, `set_icon_name()`, `connect_clicked()`
- **Signals**: `clicked`
- **Use case**: Actions, commands

#### Entry
Single-line text input field.
```rust
Entry::new()
```
- **Methods**: `set_text()`, `text()`, `set_placeholder_text()`, `set_visibility()` (for passwords)
- **Signals**: `activate`, `changed`
- **Use case**: Text input, search boxes, passwords

#### TextView
Multi-line text editor.
```rust
TextView::new()
```
- **Methods**: `buffer()`, `set_editable()`, `set_wrap_mode()`
- **Use case**: Notes, comments, long text

#### CheckButton
Checkbox or radio button.
```rust
CheckButton::with_label("Option")
```
- **Methods**: `set_active()`, `is_active()`, `set_group()` (for radio buttons)
- **Signals**: `toggled`
- **Use case**: Boolean options, selections

#### Switch
Toggle switch (on/off).
```rust
Switch::new()
```
- **Methods**: `set_active()`, `is_active()`
- **Signals**: `state-set`
- **Use case**: Enable/disable features

#### Scale
Slider for selecting numeric values.
```rust
Scale::with_range(Orientation::Horizontal, min, max, step)
```
- **Methods**: `set_value()`, `value()`, `set_draw_value()`
- **Signals**: `value-changed`
- **Use case**: Volume, brightness, adjustments

#### SpinButton
Numeric input with up/down buttons.
```rust
SpinButton::with_range(min, max, step)
```
- **Methods**: `set_value()`, `value()`, `set_digits()`
- **Use case**: Precise numeric input

#### ComboBoxText
Dropdown menu with text options.
```rust
ComboBoxText::new()
```
- **Methods**: `append_text()`, `active_text()`, `set_active()`
- **Use case**: Selection from list

#### DropDown
Modern dropdown selector (GTK4+).
```rust
DropDown::from_strings(&["Option 1", "Option 2"])
```
- **Methods**: `set_selected()`, `selected()`
- **Use case**: Modern selection lists

---

### List Widgets
Widgets for displaying collections of items.

#### ListBox
Vertical list of rows.
```rust
ListBox::new()
```
- **Methods**: `append()`, `prepend()`, `remove()`, `set_selection_mode()`
- **Signals**: `row-activated`, `row-selected`
- **Use case**: Simple lists (< 100 items), settings panels

#### ListView
High-performance list for large datasets.
```rust
ListView::new(selection_model, factory)
```
- **Use case**: Large lists (1000s of items), email lists

#### GridView
Grid layout for items (like icon view).
```rust
GridView::new(selection_model, factory)
```
- **Use case**: Photo galleries, icon grids

#### ColumnView
Multi-column list with sortable columns.
```rust
ColumnView::new(selection_model)
```
- **Methods**: `append_column()`, `remove_column()`
- **Use case**: Tables, spreadsheets

---

### Window Widgets

#### ApplicationWindow
Main application window.
```rust
ApplicationWindow::builder()
    .application(app)
    .title("Title")
    .default_width(800)
    .default_height(600)
    .build()
```
- **Methods**: `set_title()`, `set_titlebar()`, `set_child()`, `present()`

#### Dialog
Modal dialog window.
```rust
Dialog::new()
```
- **Methods**: `add_button()`, `response()`, `set_modal()`
- **Use case**: Confirmations, forms, alerts

#### HeaderBar
Modern title bar with controls.
```rust
HeaderBar::new()
```
- **Methods**: `pack_start()`, `pack_end()`, `set_title_widget()`, `set_show_title_buttons()`
- **Use case**: Window title area

#### Popover
Popup that appears relative to a widget.
```rust
Popover::new()
```
- **Methods**: `set_child()`, `popup()`, `popdown()`, `set_parent()`
- **Use case**: Context menus, tooltips

---

### Special Widgets

#### DrawingArea
Custom drawing canvas.
```rust
DrawingArea::new()
```
- **Methods**: `set_draw_func()`
- **Use case**: Custom graphics, charts, games

#### GLArea
OpenGL rendering area.
```rust
GLArea::new()
```
- **Use case**: 3D graphics, games

#### WebView (webkit6)
Embedded web browser.
```rust
WebView::new()
```
- **Methods**: `load_uri()`, `load_html()`
- **Use case**: HTML content, rich text

---

## GTK Properties

### Size & Expansion Properties

#### set_vexpand(bool)
Controls whether widget expands vertically to fill available space.
- `true`: Widget grows to fill parent's vertical space
- `false`: Widget uses only its requested height
```rust
widget.set_vexpand(true);
```

#### set_hexpand(bool)
Controls whether widget expands horizontally to fill available space.
- `true`: Widget grows to fill parent's horizontal space
- `false`: Widget uses only its requested width
```rust
widget.set_hexpand(true);
```

#### set_vexpand_set(bool)
Whether to use the vexpand property (explicit control).
```rust
widget.set_vexpand_set(true);
```

#### set_hexpand_set(bool)
Whether to use the hexpand property (explicit control).
```rust
widget.set_hexpand_set(true);
```

#### set_width_request(i32)
Sets minimum width in pixels.
```rust
widget.set_width_request(200);  // Minimum 200px wide
```

#### set_height_request(i32)
Sets minimum height in pixels.
```rust
widget.set_height_request(100);  // Minimum 100px tall
```

#### set_size_request(width, height)
Sets both minimum width and height.
```rust
widget.set_size_request(200, 100);
```

---

### Alignment Properties

#### set_halign(Align)
Horizontal alignment of widget within its allocated space.
```rust
use gtk4::Align;
widget.set_halign(Align::Start);   // Left (or right in RTL)
widget.set_halign(Align::Center);  // Center
widget.set_halign(Align::End);     // Right (or left in RTL)
widget.set_halign(Align::Fill);    // Stretch to fill
```

#### set_valign(Align)
Vertical alignment of widget within its allocated space.
```rust
widget.set_valign(Align::Start);   // Top
widget.set_valign(Align::Center);  // Center
widget.set_valign(Align::End);     // Bottom
widget.set_valign(Align::Fill);    // Stretch to fill
```

---

### Margin Properties
Margins add space **outside** the widget's border.

#### set_margin_start(i32)
Left margin in LTR, right margin in RTL.
```rust
widget.set_margin_start(12);  // 12 pixels
```

#### set_margin_end(i32)
Right margin in LTR, left margin in RTL.
```rust
widget.set_margin_end(12);
```

#### set_margin_top(i32)
Top margin.
```rust
widget.set_margin_top(12);
```

#### set_margin_bottom(i32)
Bottom margin.
```rust
widget.set_margin_bottom(12);
```

**Note**: GTK4 removed the generic `margin` property. You must set each side individually.

---

### Visibility & Sensitivity

#### set_visible(bool)
Controls whether widget is visible.
```rust
widget.set_visible(true);   // Show
widget.set_visible(false);  // Hide
```

#### set_sensitive(bool)
Controls whether widget responds to input.
```rust
widget.set_sensitive(true);   // Enabled
widget.set_sensitive(false);  // Disabled (greyed out)
```

#### set_opacity(f64)
Widget transparency (0.0 = invisible, 1.0 = opaque).
```rust
widget.set_opacity(0.5);  // 50% transparent
```

---

### Style Properties

#### add_css_class(str)
Adds a CSS class to the widget for styling.
```rust
button.add_css_class("suggested-action");  // Blue button
label.add_css_class("title-1");            // Large title
```

**Common CSS Classes**:
- Text: `title-1`, `title-2`, `title-3`, `title-4`, `heading`, `caption`, `dim-label`, `monospace`
- Buttons: `suggested-action` (blue), `destructive-action` (red), `flat`
- Containers: `card`, `toolbar`, `navigation-sidebar`

#### remove_css_class(str)
Removes a CSS class.
```rust
widget.remove_css_class("flat");
```

#### set_name(str)
Sets widget's name for CSS selection.
```rust
widget.set_name("my-custom-widget");
```

---

### Tooltip Properties

#### set_tooltip_text(str)
Sets simple text tooltip.
```rust
button.set_tooltip_text("Click to save");
```

#### set_tooltip_markup(str)
Sets tooltip with Pango markup.
```rust
button.set_tooltip_markup("<b>Bold</b> and <i>italic</i>");
```

---

### Focus Properties

#### set_can_focus(bool)
Whether widget can receive keyboard focus.
```rust
widget.set_can_focus(true);
```

#### set_focus_on_click(bool)
Whether widget grabs focus when clicked.
```rust
button.set_focus_on_click(true);
```

#### grab_focus()
Gives keyboard focus to widget.
```rust
entry.grab_focus();  // Focus text input
```

---

### ScrolledWindow Properties

#### set_policy(h_policy, v_policy)
Controls when scrollbars appear.
```rust
use gtk4::PolicyType;
scrolled.set_policy(
    PolicyType::Never,      // Never show horizontal scrollbar
    PolicyType::Automatic   // Show vertical scrollbar when needed
);
```

**PolicyType values**:
- `Never`: Never show scrollbar
- `Always`: Always show scrollbar
- `Automatic`: Show only when content overflows

#### set_min_content_width(i32)
Minimum width of scrolled content.
```rust
scrolled.set_min_content_width(300);
```

#### set_min_content_height(i32)
Minimum height of scrolled content.
```rust
scrolled.set_min_content_height(200);
```

---

### Paned Properties

#### set_position(i32)
Position of divider in pixels.
```rust
paned.set_position(200);  // Divider at 200px from start
```

#### set_resize_start_child(bool)
Whether start child can resize when paned resizes.
```rust
paned.set_resize_start_child(false);  // Fixed size
```

#### set_shrink_start_child(bool)
Whether start child can shrink below its minimum size.
```rust
paned.set_shrink_start_child(false);  // Cannot shrink
```

#### set_resize_end_child(bool)
Whether end child can resize.
```rust
paned.set_resize_end_child(true);  // Can resize
```

#### set_shrink_end_child(bool)
Whether end child can shrink.
```rust
paned.set_shrink_end_child(true);
```

---

### Box Properties

#### set_spacing(i32)
Space between children in pixels.
```rust
box.set_spacing(12);  // 12px between widgets
```

#### set_homogeneous(bool)
Whether all children get equal space.
```rust
box.set_homogeneous(true);  // Equal sizes
```

---

### Grid Properties

#### set_row_spacing(u32)
Space between rows.
```rust
grid.set_row_spacing(12);
```

#### set_column_spacing(u32)
Space between columns.
```rust
grid.set_column_spacing(12);
```

#### set_row_homogeneous(bool)
Whether all rows have the same height.
```rust
grid.set_row_homogeneous(true);
```

#### set_column_homogeneous(bool)
Whether all columns have the same width.
```rust
grid.set_column_homogeneous(true);
```

---

### Label Properties

#### set_wrap(bool)
Whether text wraps to multiple lines.
```rust
label.set_wrap(true);
```

#### set_wrap_mode(WrapMode)
How text wraps.
```rust
use gtk4::pango::WrapMode;
label.set_wrap_mode(WrapMode::Word);      // Wrap at word boundaries
label.set_wrap_mode(WrapMode::Char);      // Wrap at any character
label.set_wrap_mode(WrapMode::WordChar);  // Word, then char
```

#### set_selectable(bool)
Whether user can select text.
```rust
label.set_selectable(true);  // Can copy text
```

#### set_markup(str)
Sets text with Pango markup.
```rust
label.set_markup("<b>Bold</b> and <span color='red'>Red</span>");
```

---

## Quick Reference Tables

### Common Widget Patterns

| Pattern | Code |
|---------|------|
| Centered widget | `widget.set_halign(Align::Center); widget.set_valign(Align::Center);` |
| Full-width widget | `widget.set_hexpand(true); widget.set_halign(Align::Fill);` |
| Full-height widget | `widget.set_vexpand(true); widget.set_valign(Align::Fill);` |
| Fill all space | `widget.set_hexpand(true); widget.set_vexpand(true);` |
| Fixed size | `widget.set_width_request(200); widget.set_height_request(100);` |
| Uniform margins | `widget.set_margin_start(12); widget.set_margin_end(12); widget.set_margin_top(12); widget.set_margin_bottom(12);` |

### Layout Decision Guide

| Use Case | Widget Choice |
|----------|---------------|
| Linear arrangement | `Box` |
| Table/grid layout | `Grid` |
| Resizable panels | `Paned` |
| Long scrollable content | `ScrolledWindow` |
| Multiple pages | `Stack` + `StackSwitcher` |
| Simple list (< 100 items) | `ListBox` |
| Large list (1000+ items) | `ListView` |
| Tabular data | `ColumnView` |
| Photo grid | `GridView` |

### Signal Connection

```rust
// Button click
button.connect_clicked(|btn| {
    println!("Clicked!");
});

// Entry text changed
entry.connect_changed(|entry| {
    println!("Text: {}", entry.text());
});

// Entry activated (Enter pressed)
entry.connect_activate(|entry| {
    println!("Submitted: {}", entry.text());
});

// CheckButton toggled
check.connect_toggled(|check| {
    println!("Checked: {}", check.is_active());
});

// ListBox row selected
listbox.connect_row_selected(|_, row| {
    if let Some(row) = row {
        println!("Selected row #{}", row.index());
    }
});

// Window close
window.connect_close_request(|window| {
    println!("Window closing");
    glib::Propagation::Proceed
});
```

---

## Best Practices

### Memory Management & Reference Counting

#### Use Weak References for Child Elements in Callbacks
**Critical Rule**: When connecting signals/callbacks, use **weak references** (`downgrade()`) for child widgets to prevent reference cycles and memory leaks.

```rust
// ❌ BAD - Creates reference cycle, memory leak
let button = Button::with_label("Click");
let label = Label::new(Some("0"));

let label_clone = label.clone();  // Strong reference
button.connect_clicked(move |_| {
    label_clone.set_text("Clicked!");  // Keeps label alive forever
});

// ✅ GOOD - Uses weak reference, prevents memory leak
let button = Button::with_label("Click");
let label = Label::new(Some("0"));

let label_weak = label.downgrade();  // Weak reference
button.connect_clicked(move |_| {
    if let Some(label) = label_weak.upgrade() {  // Safely upgrade when needed
        label.set_text("Clicked!");
    }
});
```

#### Understanding Strong vs Weak References

**Strong References** (`.clone()`):
- Keep the widget alive as long as the reference exists
- Parent widgets hold strong references to their children
- Use for: Storing widgets as struct fields, parent-child relationships

**Weak References** (`.downgrade()`):
- Don't keep the widget alive
- Must be upgraded to use: `weak_ref.upgrade()` returns `Option<Widget>`
- Use for: Callbacks, signal handlers, closures that reference other widgets

#### The `clone!` Macro Pattern

GTK Rust provides a convenient `clone!` macro for managing references in closures:

```rust
use gtk4::glib::clone;

let button = Button::with_label("Click");
let label = Label::new(Some("Counter: 0"));
let counter = Rc::new(RefCell::new(0));

// Automatically creates weak references and handles upgrading
button.connect_clicked(clone!(
    #[weak] label,           // Weak reference to label
    #[strong] counter,       // Strong reference to counter
    move |_| {
        *counter.borrow_mut() += 1;
        label.set_text(&format!("Counter: {}", counter.borrow()));
    }
));
```

#### Reference Guidelines

1. **Parent widgets automatically hold strong references** to children when you append/attach them
   ```rust
   let container = Box::new(Orientation::Vertical, 0);
   let button = Button::with_label("Test");
   container.append(&button);  // Container now owns button (strong ref)
   // button can go out of scope, it's kept alive by container
   ```

2. **Callbacks should use weak references** to widgets they manipulate
   ```rust
   // Entry and button are siblings in the same container
   let entry = Entry::new();
   let button = Button::with_label("Submit");

   let entry_weak = entry.downgrade();
   button.connect_clicked(move |_| {
       if let Some(entry) = entry_weak.upgrade() {
           println!("Text: {}", entry.text());
       }
   });
   ```

3. **State shared between callbacks needs strong references** (wrapped in `Rc<RefCell<T>>`)
   ```rust
   use std::rc::Rc;
   use std::cell::RefCell;

   let counter = Rc::new(RefCell::new(0));

   let counter_clone = counter.clone();  // Strong clone for closure
   button.connect_clicked(move |_| {
       *counter_clone.borrow_mut() += 1;
   });
   ```

#### Complete Example: Proper Reference Management

```rust
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use gtk4::glib::clone;
use std::rc::Rc;
use std::cell::RefCell;

fn build_ui(app: &Application) {
    // Create widgets - these are initially strong references
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Counter App")
        .default_width(300)
        .default_height(200)
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
    let label = Label::new(Some("Count: 0"));
    let inc_button = Button::with_label("Increment");
    let dec_button = Button::with_label("Decrement");
    let reset_button = Button::with_label("Reset");

    // Shared state needs Rc<RefCell<T>>
    let count = Rc::new(RefCell::new(0));

    // Append children - vbox now holds strong references
    vbox.append(&label);
    vbox.append(&inc_button);
    vbox.append(&dec_button);
    vbox.append(&reset_button);
    window.set_child(Some(&vbox));

    // Connect signals using weak references to avoid cycles
    inc_button.connect_clicked(clone!(
        #[weak] label,
        #[strong] count,
        move |_| {
            *count.borrow_mut() += 1;
            label.set_text(&format!("Count: {}", count.borrow()));
        }
    ));

    dec_button.connect_clicked(clone!(
        #[weak] label,
        #[strong] count,
        move |_| {
            *count.borrow_mut() -= 1;
            label.set_text(&format!("Count: {}", count.borrow()));
        }
    ));

    reset_button.connect_clicked(clone!(
        #[weak] label,
        #[strong] count,
        move |_| {
            *count.borrow_mut() = 0;
            label.set_text("Count: 0");
        }
    ));

    window.present();
}
```

#### When You'll Get Reference Cycles

**Problem**: Widget A holds a strong reference to Widget B in a callback, and Widget B's parent holds Widget A.

```rust
// ❌ This creates a cycle:
// parent -> button (strong) -> closure -> label (strong) -> parent
let parent = Box::new(Orientation::Vertical, 0);
let button = Button::new();
let label = Label::new(None);

parent.append(&button);
parent.append(&label);

let label_clone = label.clone();  // Strong reference in closure
button.connect_clicked(move |_| {
    label_clone.set_text("Clicked");  // Cycle: button -> label -> parent -> button
});
```

**Solution**: Use weak references in closures:
```rust
// ✅ No cycle: weak reference breaks the chain
let label_weak = label.downgrade();
button.connect_clicked(move |_| {
    if let Some(label) = label_weak.upgrade() {
        label.set_text("Clicked");
    }
});
```

### Layout Rules
1. **Always set expansion on containers** that should fill space
2. **Set expansion on ScrolledWindow** to make scrolling work properly
3. **Use consistent spacing** (typically 6, 12, or 24 pixels)
4. **Set margins for breathing room** (12-20 pixels typical)
5. **Use weak references in callbacks** to prevent memory leaks

### Common Mistakes
- ❌ Forgetting `set_vexpand(true)` on scrollable containers
- ❌ Not setting expansion on parent containers
- ❌ Using absolute positioning instead of layouts
- ❌ Mixing expansion without proper alignment
- ❌ **Using strong references (`.clone()`) in signal callbacks** - causes memory leaks
- ❌ **Not upgrading weak references before use** - will panic
- ❌ **Forgetting to wrap shared state in `Rc<RefCell<T>>`**

### Debugging Layout Issues
1. Check if parent has expansion set
2. Check if widget itself needs expansion
3. Verify ScrolledWindow policy settings
4. Add temporary CSS borders to visualize layout
5. Use GTK Inspector (Ctrl+Shift+D or Ctrl+Shift+I)

### Debugging Memory Issues
1. **Use weak references in all callbacks** - this prevents most leaks
2. **Check for reference cycles** - if widgets aren't being freed when windows close
3. **Use `Rc::strong_count()` and `Rc::weak_count()`** to debug reference counting
4. **Valgrind or similar tools** can detect memory leaks in native code
