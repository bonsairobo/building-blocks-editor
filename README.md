# Building Blocks Editor

Voxel map editor using [building-blocks](https://github.com/bonsairobo/building-blocks) and [bevy](https://github.com/bevyengine/bevy).

## Warning

This is very much a work in progress and very experimental. But we hope that eventually this will actually be
useful for making games.

## Assets

To get our example assets, install [Git LFS](https://git-lfs.github.com/) before cloning.

## Controls

### Camera

Unreal Engine style mouse camera.

- Left drag: Locomotion
- Right drag: Change viewing angle
- Left and Right drag: Translate up/down/left/right

Orbital Camera
- CTRL + mouse: Orbit camera
- Middle click + mouse: Pan camera
- Mouse wheel: Zoom

### Editing Tools

- `T`: Enter terraforming mode
  - `Z`: create terrain
  - `X`: remove terrain
  - `1..4`: Select voxel type
  - `UP`/`DOWN`: Increase/decrease brush radius
- `D`: Enter face dragging mode
  - Click two face corners, then drag the highlighted region
- `U`: Undo last edit
- `R`: Redo last undone edit
