use crate::utils::*;

const SHOP: &str = "Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3";

fn parse_section(lines: &mut impl Iterator<Item = &'static str>) -> Vec<(isize, isize, isize)> {
    lines
        .by_ref()
        .skip(1)
        .take_while(|s| !s.is_empty())
        .map(|s| &s[11..])
        .map(|s| s.split_whitespace().map(parse).to_vec())
        .map(|a| (a[0], a[1], a[2]))
        .to_vec()
}

fn div_mod(x: &mut usize, m: usize) -> usize {
    let ret = *x % m;
    *x /= m;
    ret
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let mut lines = input.lines();
    let boss_hp = scanf!(lines.next().unwrap(), "Hit Points: {}", isize).unwrap();
    let boss_damage = scanf!(lines.next().unwrap(), "Damage: {}", isize).unwrap();
    let boss_armor = scanf!(lines.next().unwrap(), "Armor: {}", isize).unwrap();

    let player_hp = 100;

    let mut shop_lines = SHOP.lines();
    let shop_weapons = parse_section(&mut shop_lines);
    let mut shop_armor = parse_section(&mut shop_lines);
    let shop_rings = parse_section(&mut shop_lines);

    shop_armor.push((0, 0, 0));

    let mut max_cost = 0;

    for mut combo in 0.. {
        let weapon = shop_weapons[div_mod(&mut combo, shop_weapons.len())];
        let armor = shop_armor[div_mod(&mut combo, shop_armor.len())];
        let first_ring_index = div_mod(&mut combo, (shop_rings.len() + 1));
        let second_ring_index = combo;
        if second_ring_index == first_ring_index {
            continue;
        }
        if second_ring_index > shop_rings.len() {
            break;
        }
        let ring_1 = shop_rings.get(first_ring_index).unwrap_or(&(0, 0, 0));
        let ring_2 = shop_rings.get(second_ring_index).unwrap_or(&(0, 0, 0));

        let total_cost = weapon.0 + armor.0 + ring_1.0 + ring_2.0;
        if total_cost <= max_cost {
            continue;
        }
        let player_damage = weapon.1 + ring_1.1 + ring_2.1;
        let player_armor = armor.2 + ring_1.2 + ring_2.2;

        let mut player_damage = (player_damage - boss_armor).max(1);
        let mut boss_damage = (boss_damage - player_armor).max(1);
        let rounds = num::integer::div_ceil(boss_hp, player_damage);
        let remaining_hp = player_hp - boss_damage * (rounds - 1);
        if remaining_hp <= 0 {
            max_cost = total_cost;
        }
    }
    pv!(max_cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let mut lines = input.lines();
    let boss_hp = scanf!(lines.next().unwrap(), "Hit Points: {}", isize).unwrap();
    let boss_damage = scanf!(lines.next().unwrap(), "Damage: {}", isize).unwrap();
    let boss_armor = scanf!(lines.next().unwrap(), "Armor: {}", isize).unwrap();

    let player_hp = 100;

    let mut shop_lines = SHOP.lines();
    let shop_weapons = parse_section(&mut shop_lines);
    let mut shop_armor = parse_section(&mut shop_lines);
    let shop_rings = parse_section(&mut shop_lines);

    shop_armor.push((0, 0, 0));

    let mut min_cost = std::isize::MAX;

    for mut combo in 0.. {
        let weapon = shop_weapons[div_mod(&mut combo, shop_weapons.len())];
        let armor = shop_armor[div_mod(&mut combo, shop_armor.len())];
        let first_ring_index = div_mod(&mut combo, (shop_rings.len() + 1));
        let second_ring_index = combo;
        if second_ring_index == first_ring_index {
            continue;
        }
        if second_ring_index > shop_rings.len() {
            break;
        }
        let ring_1 = shop_rings.get(first_ring_index).unwrap_or(&(0, 0, 0));
        let ring_2 = shop_rings.get(second_ring_index).unwrap_or(&(0, 0, 0));

        let total_cost = weapon.0 + armor.0 + ring_1.0 + ring_2.0;
        if total_cost >= min_cost {
            continue;
        }
        let player_damage = weapon.1 + ring_1.1 + ring_2.1;
        let player_armor = armor.2 + ring_1.2 + ring_2.2;

        let mut player_damage = (player_damage - boss_armor).max(1);
        let mut boss_damage = (boss_damage - player_armor).max(1);
        let rounds = num::integer::div_ceil(boss_hp, player_damage);
        let remaining_hp = player_hp - boss_damage * (rounds - 1);
        if remaining_hp > 0 {
            min_cost = total_cost;
        }
    }
    pv!(min_cost);
}
