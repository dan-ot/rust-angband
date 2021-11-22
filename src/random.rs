use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

const NORMAL_TABLE: [i32; 256] = [
    206, 613, 1022, 1430, 1838, 2245, 2652, 3058, 3463, 3867, 4271, 4673, 5075, 5475, 5874, 6271,
    6667, 7061, 7454, 7845, 8234, 8621, 9006, 9389, 9770, 10148, 10524, 10898, 11269, 11638, 12004,
    12367, 12727, 13085, 13440, 13792, 14140, 14486, 14828, 15168, 15504, 15836, 16166, 16492,
    16814, 17133, 17449, 17761, 18069, 18374, 18675, 18972, 19266, 19556, 19842, 20124, 20403,
    20678, 20949, 21216, 21479, 21738, 21994, 22245, 22493, 22737, 22977, 23213, 23446, 23674,
    23899, 24120, 24336, 24550, 24759, 24965, 25166, 25365, 25559, 25750, 25937, 26120, 26300,
    26476, 26649, 26818, 26983, 27146, 27304, 27460, 27612, 27760, 27906, 28048, 28187, 28323,
    28455, 28585, 28711, 28835, 28955, 29073, 29188, 29299, 29409, 29515, 29619, 29720, 29818,
    29914, 30007, 30098, 30186, 30272, 30356, 30437, 30516, 30593, 30668, 30740, 30810, 30879,
    30945, 31010, 31072, 31133, 31192, 31249, 31304, 31358, 31410, 31460, 31509, 31556, 31601,
    31646, 31688, 31730, 31770, 31808, 31846, 31882, 31917, 31950, 31983, 32014, 32044, 32074,
    32102, 32129, 32155, 32180, 32205, 32228, 32251, 32273, 32294, 32314, 32333, 32352, 32370,
    32387, 32404, 32420, 32435, 32450, 32464, 32477, 32490, 32503, 32515, 32526, 32537, 32548,
    32558, 32568, 32577, 32586, 32595, 32603, 32611, 32618, 32625, 32632, 32639, 32645, 32651,
    32657, 32662, 32667, 32672, 32677, 32682, 32686, 32690, 32694, 32698, 32702, 32705, 32708,
    32711, 32714, 32717, 32720, 32722, 32725, 32727, 32729, 32731, 32733, 32735, 32737, 32739,
    32740, 32742, 32743, 32745, 32746, 32747, 32748, 32749, 32750, 32751, 32752, 32753, 32754,
    32755, 32756, 32757, 32757, 32758, 32758, 32759, 32760, 32760, 32761, 32761, 32761, 32762,
    32762, 32763, 32763, 32763, 32764, 32764, 32764, 32764, 32765, 32765, 32765, 32765, 32766,
    32766, 32766, 32766, 32767,
];
const MAX_RAND_DEPTH: i32 = 128;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Aspect {
    MAXIMIZE,
    EXTREMIFY,
    RANDOMIZE,
    MINIMIZE,
    AVERAGE,
}

pub struct Chance {
    numerator: i32,
    denominator: i32,
}

impl Chance {
    pub fn scaled(self, scale: i32) -> i32 {
        scale * self.numerator / self.denominator
    }
}

pub struct Random {
    rng: ThreadRng,
}

impl Random {
    pub fn new() -> Random {
        Random { rng: thread_rng() }
    }

    pub fn rand_normal(&mut self, mean: i32, stand: i32) -> i32 {
        let mut low = 0;
        let mut high = NORMAL_TABLE.len();

        let tmp = self.randint0(32768);

        while low < high {
            let mid = (low + high) >> 1;
            if NORMAL_TABLE[mid] < tmp {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        let offset = stand as usize * low / 64;

        if self.one_in(2) {
            mean - offset as i32
        } else {
            mean + offset as i32
        }
    }

    pub fn rand_sample(&mut self, mean: i32, upper: i32, lower: i32, stand_u: i32, stand_l: i32) -> i32 {
        let pick = self.rand_normal(0, 1000);

        if pick > 0 {
            mean + (pick * (upper - mean) / (100 * stand_u))
        } else if pick < 0 {
            mean + (pick * (mean - lower) / (100 * stand_l))
        } else {
            mean + pick
        }
    }

    pub fn randint0(&mut self, excluding: i32) -> i32 {
        self.rng.gen_range(0..excluding)
    }

    pub fn randint1(mut self, including: i32) -> i32 {
        self.rng.gen_range(1..=including)
    }

    pub fn rand_spread(&mut self, center: i32, spread: i32) -> i32 {
        let max = spread + spread;
        let num = self.rng.gen_range(0..max);
        center + num - spread
    }
    pub fn one_in(&mut self, chance: i32) -> bool {
        let num = self.rng.gen_range(0..chance);
        num == 0
    }
    pub fn damroll(&mut self, qty: i32, sides: i32) -> i32 {
        let mut sum = 0;
        for _die in 0..qty {
            sum += self.rng.gen_range(1..=sides);
        }
        sum
    }
    pub fn damcalc(&mut self, qty: i32, sides: i32, aspect: &Aspect) -> i32 {
        match aspect {
            Aspect::MAXIMIZE => qty * sides,
            Aspect::EXTREMIFY => qty * sides,
            Aspect::RANDOMIZE => self.damroll(qty, sides),
            Aspect::MINIMIZE => qty,
            Aspect::AVERAGE => qty * (sides + 1) / 2,
        }
    }

    pub fn simulate_division(&mut self, dividend: i32, divisor: i32) -> i32 {
        let quotient: i32 = dividend / divisor;
        let remainder: i32 = dividend % divisor;
        if self.randint0(divisor) < remainder {
            quotient + 1
        } else {
            quotient
        }
    }

    pub fn m_bonus(&mut self, max: i32, level: i32) -> i32 {
        let clamped_level = if level >= MAX_RAND_DEPTH {
            MAX_RAND_DEPTH - 1
        } else {
            level
        };

        let bonus = self.simulate_division(max * clamped_level, MAX_RAND_DEPTH);
        let stand = self.simulate_division(max, 4);

        let value = self.rand_normal(bonus, stand);

        if value < 0 {
            0
        } else if value > max {
            max
        } else {
            value
        }
    }

    pub fn m_bonus_calc(&mut self, max: i32, level: i32, aspect: &Aspect) -> i32 {
        match aspect {
            Aspect::EXTREMIFY => max,
            Aspect::MAXIMIZE => max,
            Aspect::RANDOMIZE => self.m_bonus(max, level),
            Aspect::MINIMIZE => 0,
            Aspect::AVERAGE => max * level / MAX_RAND_DEPTH,
        }
    }

    pub fn chance_check(&mut self, c: Chance) -> bool {
        self.randint0(c.denominator) >= c.denominator - c.numerator
    }
}

#[derive(Debug, Clone)]
pub struct Diceroll {
    pub base: i32,
    pub dice: i32,
    pub sides: i32,
    pub m_bonus: i32,
}

impl Diceroll {
    pub fn new(base: i32, dice: i32, sides: i32, m_bonus: i32) -> Diceroll {
        Diceroll {
            base,
            dice,
            sides,
            m_bonus,
        }
    }

    pub fn resolve(&self, rng: &mut Random, level: i32, aspect: &Aspect) -> i32 {
        if *aspect == Aspect::EXTREMIFY {
            let min = self.resolve(rng, level, &Aspect::MINIMIZE);
            let max = self.resolve(rng, level, &Aspect::MAXIMIZE);
            if min > max {
                min
            } else {
                max
            }
        } else {
            let dmg = rng.damcalc(self.dice, self.sides, aspect);
            let bonus = rng.m_bonus_calc(self.m_bonus, level, aspect);
            self.base + dmg + bonus
        }
    }

    pub fn valid(self, rng: &mut Random, test: i32) -> bool {
        if test < self.resolve(rng, 0, &Aspect::MINIMIZE) {
            false
        } else if test > self.resolve(rng, 0, &Aspect::MAXIMIZE) {
            false
        } else {
            true
        }
    }

    pub fn varies(self, rng: &mut Random) -> bool {
        self.resolve(rng, 0, &Aspect::MINIMIZE) != self.resolve(rng, 0, &Aspect::MAXIMIZE)
    }
}

