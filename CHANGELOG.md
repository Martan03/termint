# Termint changelog

## v0.8.2

### Features

- Add `Stylize` and `Styleable` traits for better styling
    - Such as `.red()`, `.on_white()` and `.bold()`

### Fixes

- Fix table row style not being applied
- Fix text parser ignoring whitespaces when `Wrap::Word`
- Fix text parser incorrectly measuring width

## v0.8.1

### Features:

- Add dummy `TestBackend` (blank `Backend` implementation)
    - `Backend::get_size` returns configured terminal size
- Add `LayoutNode::layout`, which simplifies `Widget::layout` implementations

### Changes:

- `Term` now cannot be used without `Backend`
- Remove `NoBackend` type

### Fixes:

- Fix MacOS compile error by removing `termal/raw` from default features

## v0.8.0

### Features:

- Add mouse event handling (`Widget::on_event`)
- Add `Application::message` to capture event messages
- Add `Button` widget
- Add default mouse event handling to some widgets
    - `List`, `Table` and `Scrollable` implement scrolling
- Add option to set custom event `Message` to some widgets
    - `Button`, `ProgressBar`, `Scrollable`, `Table`
- Add `LayoutNode` for layout caching (replaces `Cache`)

### Changes:

- Add `delta` argument (time between frames) to `Application::update`
- Remove `BgGrad` layout forwarding (such as `BgGrad::center`,...)
- `Widget` trait API changes due to `LayoutNode` replacing `Cache`
    - `diff` and `layout` are added to the `Widget`
    - `render` method accepts `LayoutNode` instead of `Cache` and `Rect`

## v0.7.0

### Features:

- Add `Backend` trait with `CrosstermBackend` and `TermalBackend`
  implementations
- Add option to change `Term` backend
- Add `Application` trait and `Term::run` for managed main loop
- Add `Term::draw` for size-aware rendering using `Frame` context
- Add option to force scrollbar visibility to List and Table
- Add automatic terminal restoration on crash or when `Term` dropped

### Changes:

- `Term` now requires a backend type (such as `Term::<Backend>::new()`)

### Fixes:

- Fix `Modifier` incorrectly coloring text blue
- Fix `ProgressBar` thumb styling issues

## v0.6.1

### Features:

- Add `Buffer` grapheme handling

### Fixes:

- `Layout` using cache incorrectly

## v0.6.0

### Features:

- Add missing From traits to Overlay widget
- Add `ProgressBar` widget
- Add `Table` widget
- Add Widget Cache
- Optional `serde` trait implementation

### Changes:

- Change `BgGrad` API
- Change `Border` and `Modifier` to use bitflags macro
- Rename `StrSpanExtension` to `ToSpan`
- Rework text rendering - add generic text parser
- Use `termal` library for getting terminal size and some other useful things

### Fixes:

- List widget width function prioritizes expanding list items to the width
  instead of taking up as little width as possible
- Layout not setting background properly

## v0.5.2

### Features:

- Optimise Buffer merge function
- Add more useful trait implementations
- Edit docs

## v0.5.1

### Features:

- Rename Coords to Vec2
- Add trait implementations and new features:
    - Buffer
    - Padding
    - Rect
    - Vec2
- Scrollbar widget
- Scrollable widget - allows overflown content to be accessed by scrolling
- Overlay widget

## v0.5.0

### Features:

- Add Grid widget
- Add rendering Buffer (edit all rendering functions)
- Add rerendering and rendering only changed characters to Term
- Add small screen option to Term, when widget cannot fit
- Add Style struct
- Add universal Color enum (removed Fg and Bg enums)
- Edit Modifier to work as bitflag rather then enum

### Fixes:

- Fix Layout width & height functions
- Fix List to have proper state

## v0.4.2

### Features:

- Add option to get String representation of Widget and Text

### Fixes:

- Fix Span and Grad ellipsis (when ellipsis couldn't fit)

## v0.4.1

### Features:

- Add option to get List offset
- Add selected background and character to List

### Fixes:

- Remove all `println` printing (could cause overflow)

## v0.4.0

### Features:

- Add BgGrad widget that renders background gradient
- Add centering to the Layout widget
- Add hex and HSL color code support
- Add Min and MinMax Constrain
- Add option to automatically scroll to item in List widget
- Add Spacer widget for better layouting
- Add text alignment to Span and Grad widget
- List doesn't show scrollbar when not necessary

### Fixes:

- Fix Block border taking space even when not used
- Fix Grad and Span showing ellipsis when not necessary
- Fix Grad letter wrap rendering (was shifted down)
- Fix Span rendering when using newline characters
- Make Grad with Letter Wrap render all spaces

## v0.3.1

### Fixes:

- Grad widget with vertical gradient and letter wrap
- Make Span with Letter Wrap render all spaces

## v0.3.0

### Features:

- Add padding to Block, Layout and Term
- Add List widget with scrollbar
- Add Term for full screen rendering
- Automatic conversion when adding child to widget to Box value
- Created useful macros
- Paragraph supports Grad widget as well

### Fixes:

- Fix block overflow when rendering on full screen

## v0.2.0

### Features:

- Add new Layout constrains (Min, Fill)
- Grad widget (text with gradient background)
- Paragraph widget (text build from Spans)

### Fixes:

- Fix block span ellipsis underflow

## v0.1.1

### Fixes:

- Fix readme and docs not containing correct links

## v0.1.0

### Features:

- Block widget
- Enums for better work with ANSI codes
- Layout widget
- Span widget
