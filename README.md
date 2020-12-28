# Building Blocks Editor

Voxel map editor using [building-blocks](https://github.com/bonsairobo/building-blocks) and [bevy](https://github.com/bevyengine/bevy).

## Warning

This is very much a work in progress and very experimental. But we hope that eventually this will actually be
useful for making games.

## Controls

### Camera

Unreal Engine style mouse camera.

- Left drag: Locomotion
- Right drag: Change viewing angle
- Left and Right drag: Translate up/down/left/right

### Editing Tools

- `T`: Enter terraforming mode
  - `SPACE`: create terrain
  - `BACKSPACE`: remove terrain
  - `1..4`: Select voxel type
- `D`: Enter face dragging mode
  - Click two face corners, then drag the highlighted region
- `U`: Undo last edit
- `R`: Redo last undone edit
