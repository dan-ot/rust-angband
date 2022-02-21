// A note about this module - 'Bitflags' are actually a tagging system. Each
// Bitflag represents a set of unique enum values that are either present or not.
// It seems like a more memory-expensive but less complex representation would be
// a HashSet. A quick glance on the documentation there reveals a very similar
// set of operators to the ones we see here

#[derive(Debug, Clone)]
pub struct Bitflag {
    /// A set of u8's representing a block of flag sets (each u8 is an array of 8 bits/flags)
    bytes: Vec<u8>,
    /// What's the largest value (enum + 1) that our total set of bytes will have?
    max: usize,
    /// How many u8's will it take to hold an enum with this many flags?
    size: usize,
}

/// Each set of flags is 8 bits
const FLAG_WIDTH: usize = 8;
/// Flags start at 1
const FLAG_START: usize = 1;
/// There are no more flags when we're down to 0
const FLAG_END: usize = 0;

/// Given an ordinal inside an enum set, what's the index of its bit set?
pub fn flag_offset(ordinal: usize) -> usize {
    (ordinal - FLAG_START) / FLAG_WIDTH
}

/// Given an ordinal, return the bitmask for that ordinal (masked against the other bits in the flag set at that position)
pub fn flag_binary(ordinal: usize) -> u8 {
    1 << ((ordinal - FLAG_START) % FLAG_WIDTH)
}

/// Given an ordinal, figure how how deep into the set vectors and a bitmask, so the exact value can be retrieved
pub fn coordinates(ordinal: usize) -> (usize, u8) {
    (
        (ordinal - FLAG_START) / FLAG_WIDTH,
        1 << ((ordinal - FLAG_START) & FLAG_WIDTH),
    )
}

impl Bitflag {
    /// Create a Bitflag which will hold 'size' values
    pub fn new(size: usize) -> Bitflag {
        let s = size + FLAG_WIDTH - 1 / FLAG_WIDTH;
        Bitflag {
            // Start filled with all 0's
            bytes: vec![0; s],
            max: size * FLAG_WIDTH + FLAG_START,
            size: s,
        }
    }

    /// Tests if an ordinal is 'on' in the flag set
    pub fn has(&self, flag_at: usize) -> bool {
        if FLAG_END == flag_at {
            false
        } else {
            let (offset, bitmask) = coordinates(flag_at);

            self.bytes[offset] & bitmask == bitmask
        }
    }

    /// Returns the next 'on' ordinal in the bitflags, starting with start
    pub fn next(&self, start: usize) -> usize {
        for ord in (start)..self.max {
            let (offset, bitmask) = coordinates(ord);

            if self.bytes[offset] & bitmask == bitmask {
                return ord;
            }
        }

        FLAG_END
    }

    /// Counts the number of 'on' flags
    pub fn count(&self) -> usize {
        let mut count = 0;

        for i in 0..self.size {
            for j in 1..=FLAG_WIDTH {
                let b = flag_binary(j);

                if self.bytes[i] & b == b {
                    count += 1;
                }
            }
        }
        count
    }

    /// Checks if all flags are 'off'
    pub fn is_empty(&self) -> bool {
        for b in &self.bytes {
            if *b != 0 {
                return false;
            }
        }

        true
    }

    /// Checks if all flags are 'on'
    pub fn is_full(&self) -> bool {
        for b in &self.bytes {
            if *b != 0b11111111 {
                return false;
            }
        }

        true
    }

    /// Checks if any 'on' flags are common between the two Bitflags
    pub fn intersects(&self, other: &Bitflag) -> bool {
        for i in 0..self.size {
            if self.bytes[i] & other.bytes[i] > 0 {
                return true;
            }
        }

        false
    }

    /// Checks if every 'on' flag in other is also 'on' in self
    pub fn has_subset(&self, other: &Bitflag) -> bool {
        for i in 0..self.size {
            if (!self.bytes[i]) & other.bytes[i] > 0 {
                return false;
            }
        }

        true
    }

    /// Check two Bitfields for exact equality
    pub fn is_equal(&self, other: &Bitflag) -> bool {
        for i in 0..self.size {
            if self.bytes[i] != other.bytes[i] {
                return false;
            }
        }

        true
    }

    /// Sets the given flag index to 'on', returns true if something changed and false if it was already on
    pub fn turn_on(&mut self, ordinal: usize) -> bool {
        let (offset, bitmask) = coordinates(ordinal);

        if self.bytes[offset] & bitmask == bitmask {
            false
        } else {
            self.bytes[offset] |= bitmask;
            true
        }
    }

    /// Sets the given flag index to 'off', returns true if something changed and false if it was already off
    pub fn turn_off(&mut self, ordinal: usize) -> bool {
        let (offset, bitmask) = coordinates(ordinal);

        if self.bytes[offset] & bitmask == 0 {
            false
        } else {
            self.bytes[offset] &= !bitmask;
            true
        }
    }

    /// Clears all flags
    pub fn wipe(&mut self) {
        for b in self.bytes.iter_mut() {
            *b = 0b00000000;
        }
    }

    /// Sets all flags to 'on'
    pub fn setall(&mut self) {
        for b in self.bytes.iter_mut() {
            *b = 0b11111111;
        }
    }

    /// Flips all flags to the opposite of their current value
    pub fn negate(&mut self) {
        for b in self.bytes.iter_mut() {
            *b = !*b;
        }
    }

    /// Copies the flags from self onto other
    pub fn copy_from(&mut self, other: &Bitflag) {
        for i in 0..self.size {
            self.bytes[i] = other.bytes[i];
        }
    }

    /// All 'on' flags from other become 'on' flags in self;
    /// returns true when there was a change
    pub fn union(&mut self, other: &Bitflag) -> bool {
        let mut changed = false;

        for i in 0..self.size {
            if !self.bytes[i] & other.bytes[i] > 0 {
                changed = true;
            }

            self.bytes[i] |= other.bytes[i];
        }

        changed
    }

    /// All 'off' flags from other become 'off' flags in self;
    /// returns true if there was a change
    pub fn intersect(&mut self, other: &Bitflag) -> bool {
        let mut changed = false;

        for i in 0..self.size {
            if self.bytes[i] != other.bytes[i] {
                changed = true;
            }

            self.bytes[i] &= other.bytes[i];
        }

        changed
    }

    /// All 'on' flags in other become 'off' flags in self;
    /// returns true if there was a change
    pub fn difference(&mut self, other: &Bitflag) -> bool {
        let mut changed = false;

        for i in 0..self.size {
            if self.bytes[i] & other.bytes[i] > 0 {
                changed = true;
            }

            self.bytes[i] &= !other.bytes[i];
        }

        changed
    }

    /// Checks if any of the iterator-supplied indices are 'on'
    pub fn any(&self, flags: Box<dyn Iterator<Item = usize>>) -> bool {
        flags.fold(false, |p, c| {
            if !p {
                let (offset, bitmask) = coordinates(c);
                self.bytes[offset] & bitmask > 0
            } else {
                p
            }
        })
    }

    /// Checks if all of the iterator-supplied indices are 'on'
    pub fn all(&self, flags: Box<dyn Iterator<Item = usize>>) -> bool {
        flags.fold(true, |p, c| {
            if p {
                let (offset, bitmask) = coordinates(c);
                !(self.bytes[offset] & bitmask) > 0
            } else {
                p
            }
        })
    }

    /// Sets all the incoming flag indices to 'off'. Returns true if anything changed.
    pub fn clear(&mut self, flags: Box<dyn Iterator<Item = usize>>) -> bool {
        let mut changed = false;

        for flag in flags {
            let (offset, bitmask) = coordinates(flag);

            if self.bytes[offset] & bitmask > 0 {
                changed = true;
            }

            self.bytes[offset] &= !bitmask;
        }

        changed
    }

    /// Sets all the incoming flag indices to 'on'. Returns true if anything changed.
    pub fn set(&mut self, flags: Box<dyn Iterator<Item = usize>>) -> bool {
        let mut changed = false;

        for flag in flags {
            let (offset, bitmask) = coordinates(flag);

            if !(self.bytes[offset] & bitmask) > 0 {
                changed = true;
            }

            self.bytes[offset] |= bitmask;
        }

        changed
    }

    /// Sets the provided flags to 'on' and all others to 'off'
    pub fn init<T: Into<usize>>(&mut self, flags: Box<dyn Iterator<Item = T>>) {
        self.wipe();

        for flag in flags {
            self.turn_on(flag.into());
        }
    }

    /// Clears all flags not passed in; returns true if anything changed
    pub fn mask(&mut self, flags: Box<dyn Iterator<Item = usize>>) -> bool {
        let mut other = Bitflag::new(self.size);
        other.init(flags);
        self.intersect(&other)
    }
}
