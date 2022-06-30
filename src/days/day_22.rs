use crate::utils::*;

fn reduce_effect(effect: &mut usize) -> bool {
    if *effect > 0 {
        *effect -= 1;
        true
    } else {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct State {
    player_hp: isize,
    player_mp: usize,
    mp_spent: usize,
    boss_hp: isize,
    boss_damage: isize,
    shield_duration: usize,
    poison_duration: usize,
    recharge_duration: usize,
    hard_mode: bool,
}
impl State {
    fn turn(&self, out_states: &mut HashSet<State>) -> Option<State> {
        let mut next_state = *self;
        if next_state.hard_mode {
            next_state.player_hp -= 1;
            if next_state.player_hp <= 0 {
                return None;
            }
        }
        reduce_effect(&mut next_state.shield_duration);
        let has_poison = reduce_effect(&mut next_state.poison_duration);
        let has_recharge = reduce_effect(&mut next_state.recharge_duration);
        if has_poison {
            next_state.boss_hp -= 3;
        }
        if has_recharge {
            next_state.player_mp += 101;
        }

        if next_state.boss_hp <= 0 {
            return Some(next_state);
        }

        let mut next_states = vec![];
        // missile
        if let Some(mut next_state) = next_state.cast(53) {
            next_state.boss_hp -= 4;
            next_states.push(next_state);
        }
        // drain
        if let Some(mut next_state) = next_state.cast(73) {
            next_state.player_hp += 2;
            next_state.boss_hp -= 2;
            next_states.push(next_state);
        }
        // shield
        if next_state.shield_duration == 0 {
            if let Some(mut next_state) = next_state.cast(113) {
                next_state.shield_duration = 6;
                next_states.push(next_state);
            }
        }
        // poison
        if next_state.poison_duration == 0 {
            if let Some(mut next_state) = next_state.cast(173) {
                next_state.poison_duration = 6;
                next_states.push(next_state);
            }
        }
        // recharge
        if next_state.recharge_duration == 0 {
            if let Some(mut next_state) = next_state.cast(229) {
                next_state.recharge_duration = 5;
                next_states.push(next_state);
            }
        }

        next_states.sort_unstable_by_key(|s| s.mp_spent);
        for state in &next_states {
            if state.boss_hp <= 0 {
                return Some(*state);
            }
        }

        // boss' turn
        for state in &mut next_states {
            reduce_effect(&mut state.shield_duration);
            if reduce_effect(&mut state.poison_duration) {
                state.boss_hp -= 3;
            }
            if reduce_effect(&mut state.recharge_duration) {
                state.player_mp += 101;
            }
            state.player_hp -= if state.shield_duration > 0 {
                (state.boss_damage - 7).max(1)
            } else {
                state.boss_damage
            };
        }

        out_states.extend(next_states.iter().filter(|s| s.player_hp > 0).copied());

        None
    }
    fn cast(&self, mp: usize) -> Option<Self> {
        (self.player_mp >= mp).then(|| {
            let mut next_state = *self;
            next_state.player_mp -= mp;
            next_state.mp_spent += mp;
            next_state
        })
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let mut lines = input.lines();
    let boss_hp = scanf!(lines.next().unwrap(), "Hit Points: {}", isize).unwrap();
    let boss_damage = scanf!(lines.next().unwrap(), "Damage: {}", isize).unwrap();

    let player_hp = 50;
    let player_mp = 500;

    let mut states = HashSet::new();
    states.insert(State {
        player_hp,
        player_mp,
        boss_hp,
        boss_damage,
        hard_mode: true,
        ..Default::default()
    });

    let mut min_mp_spent = usize::MAX;
    while !states.is_empty() {
        let mut next_states = HashSet::new();
        for state in states.drain() {
            if let Some(end_state) = state.turn(&mut next_states) {
                min_mp_spent = min_mp_spent.min(end_state.mp_spent);
            }
        }
        next_states.retain(|s| s.mp_spent < min_mp_spent);
        states = next_states;
    }
    pv!(min_mp_spent);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let mut lines = input.lines();
    let boss_hp = scanf!(lines.next().unwrap(), "Hit Points: {}", isize).unwrap();
    let boss_damage = scanf!(lines.next().unwrap(), "Damage: {}", isize).unwrap();

    let player_hp = 50;
    let player_mp = 500;

    let mut states = HashSet::new();
    states.insert(State {
        player_hp,
        player_mp,
        boss_hp,
        boss_damage,
        ..Default::default()
    });

    let mut min_mp_spent = usize::MAX;
    while !states.is_empty() {
        let mut next_states = HashSet::new();
        for state in states.drain() {
            if let Some(end_state) = state.turn(&mut next_states) {
                min_mp_spent = min_mp_spent.min(end_state.mp_spent);
            }
        }
        next_states.retain(|s| s.mp_spent < min_mp_spent);
        states = next_states;
    }
    pv!(min_mp_spent);
}
