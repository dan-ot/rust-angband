/**
 A sub-window for HUD concerns. Should have a solid background (for now) and display some kind of content. Panels are rendered
 over the playfield and can receive focus/input.

 The coordinates given are intended to be in screen space. Panels define the visual space within which Layouts can operate.
 */
pub struct Panel {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    /** 1.0f = character panel HUD size */
    zoom: f32
}

/**
 Layouts are given some constraints, and some content, and decide how to break that content into renderable
 chunks. Most of this is behavior with inputs, not actually state - Layouts may be an enum and a series of functions
 which operate on that enum.
 */
pub enum Layout {
    /**
     Flex layouts have a set width. They distribute their contents along that width based on absolute and relative sizes.
     Maybe in the future it can include priorities, so in cramped layouts content can get dropped.
     */
    Flex,
    /**
     Stack layouts place each item on top of the next. They take up the full width of their parent.
     */
    Stack,
    /**
     Box layouts give their content a specific space to fill.
     */
    Box
}

/* Here's a sample...
    The Visible Monster List from the original code does this:
    * If the player is hallucinating, put 'Your hallucinations are too wild to see anything clearly' in orange - bail out
    * Divide the list of known monsters into 2 sections - Visible, ESP (we might want visible, infra, ESP)
    * For each section:
        * If there are no entries, skip it
        * If there are no sections, put 'No Monsters' instead of any content
        * Put the section header - 'You can see' or 'You are aware of', followed by the quantity - '1 monster' or '30 
          monsters'. The headers are sensitive to the sense being used.
        * Determine fit - leave room for an 'nnn others' line at the bottom of the display area, leave room for the two headers
          just determined. If there aren't enough lines for everything that's left, drop ESP monsters first, then drop LOS
          monsters. Minimum is the two header lines.
        * For each line that will fit in the parent:
            * Build an 'asleep' tag - multiple monsters = ' (nn asleep)', single monster = ' (asleep)'
            * If there is only one of them, build a direction: '23 N 24 W'. I think if we can have the concept of a group of
              monsters instead of only dividing by race/graphic, we can do directions on most groups.
            * Build a line out of:
                * First 2 spaces is the graphic, in the correct color
                * Direction goes right-justified
                * Asleep tag is inside that, right-justified
                * Name fills in everything between. Clip the name if it doesn't fit.
              Everything but the graphic is violet for uniques, red if the monster comes from deeper than the player has been,
              white otherwise.
    If we scrolled this, we'd want an 'others' on any end that had more content. We'd want the headings to be 'sticky', so
    they were always visible.

    So, the content is deduced from the list of monsters. It's a Box/Panel controlling the set width and height 
    (minimum height 1 + the number of non-empty sections, minimum width 23 which only gives 3 letters of the name if it's 
    asleep and solo). There's a Stack arranging the entries vertically (is a Stack with required entries interspersed
    a Tree?). Each entry is either a Box with a heading or message, or a Flow with between 2 and 4 fields. It's not a table
    because many entries will not have all fields and we don't want to waste that space.
 */

/* Here's another sample...
    The Monster Recall display from the original code does this:
    * Build a list of things the player knows (flags) about the monster. If we have spoiler cheats, turn all the flags on.
    * Add the monster's title
        * Start with 'The ' if it's not a unique
        * Otherwise, make it violet
        * Append the graphic surrounded by parenthesis
        * If there's an optional graphic, append that one surrounded by other parenthesis
        * Add a newline
    * Add kills information
        * Add a newline
    * Add 'flavor text' (which is some kind of race information)
        * Add a newline
    * Add 'movement' information
        * normal depth, speed, movement style, chase or no chase behavior
        * Add a period
    * Add 'toughness' information
        * HP, AC, % chance to hit
        * Add a period
    * Add 'XP' information
        * Actual amount is in blue
        * Based on level
        * ends in a period
    
    And so on. Each thing getting appended is a mix of static and bound text, and contains color shifts. This is best
    expressed as a string with embedded markup - the Box layout will:
        * Parse through the text, converting runs of similar text into lines
        * Process the resulting lines, given the total width allowed. Any line which extends beyond the width will be
          broken into two lines (at the previous whitespace).
 */

/* Here's another example...
    The Objects Seen display from the original code does this:
    * Like the Monster List, we divide the original set into sections. The current implementation has LOS ("You can see")
      and NO_LOS ("You are aware of"). I think in a future step I'd divide it out a bit further - things you can see,
      things you can currently divine the location of, and things you remember (with the last one being potentially stale).
    * Also like the Monster List, if there isn't room for headings and all content, the NO_LOS set gets cut first and an
      "...others" line is added at the end.
    * Each header includes the count of objects seen. If no objects are seen, the header may be omitted (on the fence about
      this one. May have to try it both ways).
    * Each line item starts with the display (2 chars), then the quantity (or an appropriate prefix if single - seems to be
      3 chars), then the name (clipped), then the same kind of direction block the Monster List uses.
    * Each line is normally white. 
        * If it's an unknown object, it's red
        * If it's a known artifact, it's violet (the unique color)
        * If it's an unknown kind of object, it's light red
        * If it's known to be worthless (cursed, harmful), it's slate/light grey

    We'll want to change this a bit beyond just objects - rather, the player can mark things they want the character to 
    remember about the current level. This list will build as the player explores, then clear once they leave.
*/

/* Here's another example...
    The Character HUD on the left side of the display does this:
    * Print the race, title, and class - Light Blue, one line each
    * Numeric stats are all uppercase letters with green numbers if they're fine, PascalCase letters with green numbers
      if they're not. Text is left-justified, numbers are right-justified.
    * First three stats: Level, Next, AU (gold)
    * Then the 'equippy' line, which is one icon/graphic for each slot.
        * Great place for tooltips...
    * Then we have the 5 stats
    * Then a break
    * Then we have AC, HP, and Mana (the last two are measures - current/max, with yellow for low and red for critical)
    * Break
    * Monster Health - a progress bar of health for the current monster. [--] if it's not visible, [* ] if it is. Colored
      to match health level.
    * 2 empty lines
    * Speed (-NN) to (+NN), colored to match (Light Umber/Light Green)
    * Depth (Town or feet then floors)

    Despite the order, these are in a list marked by priority. If there aren't enough lines to show them all, they begin
    to disappear - blank lines, race/title/class, exp, equippy, speed, depth, monster health, gold, level, sp/hp/ac, and
    the stats are the last to go.
*/

/////////// Things after this are for managing Panels, they're a distraction until we know what a Panel actually is
pub enum PanelDisplay {
    Panel(Panel),
    Hidden
}

pub struct PanelManager {
    /** left 11? 13? columns of playfield (data\[0\]) */
    pub summary: PanelDisplay, 
    /** bottom row of playfield (data\[0\]) */
    pub status: PanelDisplay, 
    /** data\[1\] - List of messages. Would be toasts, logs, or floating icons in a modern engine. */
    pub messages: PanelDisplay,
    /** data\[2\] - Lists pack contents or (and?) body slot contents */
    pub possessions: PanelDisplay,
    /** data\[3\] - Lists monsters you can see (or otherwise currently detect) */
    pub known_monsters: PanelDisplay,
    /** data\[4\] - Lists relatively static things you've seen. Currently this is just objects,
     * but as a player I'd really like to extend this to stairs or other features as they get added.
     */
    pub landmarks: PanelDisplay,
    /** data\[5\] - Describes details of the current selection - monster or object */
    pub recall: PanelDisplay,
    /** data\[6\] - Shows the map of the current level. Also zoomed out? */
    pub map: PanelDisplay,
    /** data\[7\] - Shows details of the character sheet */
    pub character_detail: PanelDisplay
}

impl PanelManager {
    fn default(window_width: f32, window_height: f32) -> PanelManager {
        PanelManager { 
            summary: PanelDisplay::Hidden, 
            status: PanelDisplay::Hidden, 
            messages: PanelDisplay::Hidden, 
            possessions: PanelDisplay::Hidden, 
            known_monsters: PanelDisplay::Hidden, 
            landmarks: PanelDisplay::Hidden, 
            recall: PanelDisplay::Hidden, 
            map: PanelDisplay::Hidden, 
            character_detail: PanelDisplay::Hidden 
        }
    }
}