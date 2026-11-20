// What a run's shops offer before planets 2 and 3, over many run seeds, from
// the real `Run::restock` (balance review, 2026-11-20). Not built by cargo
// from here; to run it, copy it into examples/ and:
//
//     cp Reports/balance-review-2026-11-20/shopsim.rs examples/shopsim.rs
//     cargo run --release --example shopsim
//     rm examples/shopsim.rs
//
// Each run wins planet 1 with 2 stars (the shop before planet 2), buys the
// Torch bullet if offered (as the owner did on 11-19, and as the shop's
// `bring_shell` intends), wins planet 2 with 2 stars (the shop before planet
// 3), and counts what the two shops offered. Points are set high so the
// Torch is always affordable; nothing else is bought.
use slime_mould::mould::bullets::{Set, Shell};
use slime_mould::mould::rounds::Card;
use slime_mould::mould::run::{self, Pick, Run};
use std::collections::HashMap;

fn main() {
    let n = 4000u64;
    let mut count: HashMap<String, u32> = HashMap::new();
    let mut add = |k: String| *count.entry(k).or_insert(0) += 1;
    let mut needle_blast_prices: Vec<u32> = Vec::new();
    let (mut needle_blast, mut needle_blast_and_one, mut best_deck, mut needs_bullet_in_shop2) = (0, 0, 0, 0);
    // Cards that do nothing without a bullet card after them (carriers) or
    // on a plain bullet (modifiers): see finding 4.
    let needs_bullet = [Card::Wide, Card::Scatter, Card::Quiet, Card::Delay, Card::Thick, Card::Trigger, Card::Expire, Card::Multicast];
    for seed in 1..=n {
        let mut r = Run::new(seed);
        r.won(2);
        let s2 = r.offers.clone();
        r.points = 10_000;
        if let Some(i) = s2.iter().position(|c| matches!(c, Card::Ammo(a) if a.shell == Shell::Torch)) {
            r.pick(Pick::Offer(i));
        }
        r.won(2);
        let s3 = r.offers.clone();
        let both: Vec<Card> = s2.iter().chain(s3.iter()).copied().collect();
        let has = |p: &dyn Fn(&Card) -> bool| both.iter().any(p);
        let bn = has(&|c| matches!(c, Card::Ammo(a) if a.shell == Shell::Blast && a.set == Set::Needle));
        let (wide, rate, seek) = (has(&|c| *c == Card::Wide), has(&|c| *c == Card::Rate), has(&|c| *c == Card::Seek));
        needle_blast += bn as u32;
        needle_blast_and_one += (bn && (wide || rate || seek)) as u32;
        best_deck += (bn && wide && rate) as u32;
        needs_bullet_in_shop2 += s2.iter().any(|c| needs_bullet.contains(c)) as u32;
        for (tag, s) in [("shop 2", &s2), ("shop 3", &s3)] {
            for c in s.iter() {
                match c {
                    Card::Ammo(a) => {
                        add(format!("{tag} bullet shell {:?}", a.shell));
                        add(format!("{tag} bullet set {:?}", a.set));
                        if a.shell == Shell::Blast && a.set == Set::Needle {
                            needle_blast_prices.push(run::card_price(*c));
                        }
                    }
                    c => add(format!("{tag} card {}", c.name())),
                }
            }
        }
    }
    let pct = |k: u32| 100.0 * k as f32 / n as f32;
    println!("runs {n}");
    println!("Needle Blast offered by planet 3: {:.1}%", pct(needle_blast));
    println!("Needle Blast plus Wide, Rate or Seek offered by planet 3: {:.1}%", pct(needle_blast_and_one));
    println!("Wide, Rate and Needle Blast all offered by planet 3: {:.2}%", pct(best_deck));
    println!("shop 2 offers a card that needs a bullet after it, or a payload: {:.1}%", pct(needs_bullet_in_shop2));
    needle_blast_prices.sort();
    if let (Some(lo), Some(hi)) = (needle_blast_prices.first(), needle_blast_prices.last()) {
        println!("Needle Blast price {lo} to {hi}, median {}", needle_blast_prices[needle_blast_prices.len() / 2]);
    }
    let mut v: Vec<_> = count.into_iter().collect();
    v.sort();
    for (k, c) in v {
        println!("{k}: {:.1}% of shops", pct(c));
    }
}
