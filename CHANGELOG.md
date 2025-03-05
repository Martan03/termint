# Termint changelog

## v0.6.0
### Features:
- Add Table widget
- Optional `serde` trait implementation

### Changes:
- Rename `StrSpanExtension` to `ToSpan`
- Rework text rendering - add generic text parser

### Fixes:
- List widget width function prioritizes expanding list items to the width
instead of taking up as little width as possible

## v0.5.3
### Features:
- Add missing From traits to Overlay widget

### Fixes:
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
