Panel is a high-level concept for organizing the UX. It loosely takes the place of Terms from the C project - the original project had a set of OS-managed windows it would trade data between. With rust-angband, we want to condense that into a single window, but still provide the ability to rearrange content as needed.

Like any UI effort, there are two domains involved - the content and the visual primitives. The primary owner of the content is the Layout; each specialization of Layout does its best to arrange the content into a reasonable format. The visual primitives are a font-texture reference and a set of renderable tris that will result in the content with a minimum of draw calls...

Layout owns content
Panel resolves Layout to primitives
PanelManager transforms primitives and passes to OpenGL, if the referred Panel is active
    - PanelManager stores base coords and sizes for each Panel (saves them on pause, restores them on resume)
    - PanelManager responds to graphics mode changes (resize, switch monitor)