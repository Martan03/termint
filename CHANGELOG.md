# Termint changelog

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
