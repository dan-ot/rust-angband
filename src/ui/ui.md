# Basic Types
The UI needs to handle three basic cases - Panels, Widgets, and Dialogs.

## Panels
Panels are the HUD - they remain visible during play and overlay the playfield/camera. Their focus interacts with the game commands system - the player might be issuing in-game commands or Panel interactions at any given moment, and many Panel commands are equivalent to in-game commands. As part of that interaction, Panels might be moved, resized, called, or dismissed, and their arrangement is remembered and reactive across screens. That being said, Panels are still a UX-layer domain - they do not understand the game systems or game data they express, so it is up to some other conversion mechanism to correctly express game data as Panel content. Panels should be rendered after the playfield and Widgets: on top of everything else.

Layout owns content
    - Tree content
        - child-pointing nodes
        - Expanded/collapsed state
        - Ordering/filtering
        - Detail/expanded node
        - Color + decoration + icon
        - Action(s) for each item
            - Command to be sent
    - Table content
        - Header, content rows
            - Column definition
            - 
        - Ordering/filtering
        - Lines, alternating background
Panel resolves Layout to primitives
PanelManager transforms primitives and passes to OpenGL, if the referred Panel is active
    - PanelManager stores base coords and sizes for each Panel (saves them on pause, restores them on resume)
    - PanelManager responds to graphics mode changes (resize, switch monitor)

## Widgets
Widgets are UX elements that map to in-game objects - meaning, they appear based on world-to-screen conversions, or even are pinned to in-game objects. These are things like the cursor, health bars, tooltips, and context menus. Like Panels, they are UX constructs, and don't/shouldn't understand the game-domain information and systems they express - that's the job of some intermediate conversion layer. Reticles, target highlights, in-game text, and the like are part of the UX but need to interact deeply with the game world - they exist in 3D space, not screen space like Widgets, so they are part of the in-game systems. Widgets can have focus and issue commands, but cannot be interacted with as UX elements (no moving or resizing). Widgets are rendered after the in-game content, but before Panels (they are under the Panel layer but on top of all game content).

## Dialogs
Dialogs are modal - when they appear, they take all focus. If the game has pause, dialogs pause the game. If the dialog occludes the playfield - it has a non-transparent background - it prevents the playfield from rendering. This includes the other UX elements, as they're part of the playfield. If it does not occlude the playfield, it has a shroud to clearly indicate that focus has been lost. Dialogs may stretch to fill the screen, or be centered just above perfect center and sized to content.