//! Recording without a window: films, sound and the shot marks. Moved out
//! of `larder-world.rs` unchanged (C9, 2026-11-14).

use super::*;

/// **No window** (`look=DIR`): the round played by a simple hand, with the
/// window's whole picture (the look, marks and HUD) saved at the game
/// seconds in `at`, and the counts beside each printed: what the window
/// shows, checkable where there is no screen (the cloud's software
/// Vulkan). Hands: `rush` (all attack the core), `farm` (all harvest the
/// crystal nearest the larder, buying soldiers from 1,200 food), `idle`.
// Two more than clippy's seven: the look's options are plain values from
// the command line, and a struct for them would only rename them.
#[allow(clippy::too_many_arguments)]
pub(super) fn headless(args: Args, dir: &str, hand: &str, at: &[f32], screen: (u32, u32), wav: Option<&str>, time_holes: bool, fixed: Option<(f32, f32, f32)>, cut: Option<Cut>, bank: Option<f32>, booms: &[(f32, f32, f32, Option<Vec<rounds::Card>>)], bosses: &[(f32, f32, f32)], turrets: &[(Kind, (f32, f32), f32)], renders: &[usize], menu_page: Option<(slime_mould::world::start::Page, &str)>, hover: Option<(f32, f32)>, hover_to: Option<(f32, f32)>, select: bool, drags: &[(String, String)], scroll: i32) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{dir}: {e}"))?;
    let (device, queue) = gpu().ok_or("no GPU adapter")?;
    let mut load_at = args.load_at.clone();
    let mut tload_at = args.tload_at.clone();
    let mut app = offscreen(args, screen, &device, &queue)?;
    if let (Some(b), Some(g)) = (bank, app.game.as_mut()) {
        g.bank = b;
    }
    // `turret=X,Y` and the other kinds: standing from the start (the
    // building films).
    if let Some(g) = app.game.as_mut() {
        for (i, &(kind, at, _)) in turrets.iter().enumerate().filter(|(_, b)| b.2 <= 0.0) {
            stand(g, kind, at, renders.contains(&i))?;
        }
    }
    // `menu=PAGE`: the start menu's page drawn over the round, as the
    // window shows it (the menu's pictures).
    let mut shop_run = None;
    if let Some((p, flag)) = menu_page {
        // `menu=shop-mid@TAB@NAME`: the shop on its tab TAB with the row
        // NAME picked (its pane), as a click picks it (Shop B, 11-15).
        let (flag, extra) = flag.split_once('@').unwrap_or((flag, ""));
        let mut m = start_menu(&app.args);
        // The gun range's armory (`range=SEED menu=shop`) over its round,
        // as H opens it; a drag changes the range's gun. Not paused: the
        // film steps through `App::step`, which holds still while paused,
        // so a frame's steps never ended (its first try hung on frame 1).
        // Give the armory's frames a few hundredths of a second apart.
        if let (slime_mould::world::start::Page::Shop, Some(r)) = (p, app.range.as_ref()) {
            m.shop = Some(shop_of(r));
            app.set_armory(true);
        }
        // The run's shop (`run=SEED menu=shop`): as after its first planet
        // won with two stars, so the picture has points to spend.
        if let (slime_mould::world::start::Page::Shop, Some(r), false) = (p, app.run.as_ref(), app.armory) {
            let mut r = r.clone();
            if r.points == 0 {
                r.won(2);
            }
            // `menu=shop-mid` films a run three planets in: cards in the
            // gun and the bag, a second gun bought, armour and turrets, and
            // points left; `shop-build` and `shop-tools` with that tab open,
            // `shop-mid@TAB@NAME` with that tab open and that row picked.
            if flag != "shop" {
                r.won(2);
                r.won(3);
                r.points += 900;
                for _ in 0..3 {
                    r.pick(slime_mould::mould::run::Pick::Offer(0));
                }
                r.pick(slime_mould::mould::run::Pick::Gun(1));
                // A fresh offer, one more card (the gun is full: it goes
                // in the bag), and a fresh offer again.
                // Fresh draws (dev mode's redraw, so they differ).
                let reroll = |r: &mut Run| {
                    let dev = std::mem::replace(&mut r.dev, true);
                    r.pick(slime_mould::mould::run::Pick::DevReroll);
                    r.dev = dev;
                };
                reroll(&mut r);
                r.pick(slime_mould::mould::run::Pick::Offer(0));
                reroll(&mut r);
                for p in [r.shop().iter().find(|x| x.name == "Armour").map(|x| x.pick), r.shop().iter().find(|x| x.name == "Turrets").map(|x| x.pick)].into_iter().flatten() {
                    r.pick(p);
                }
                // `menu=shop-turrets` (the turrets thread, 2026-11-16): the
                // gun slots switched to the turrets' deck, with the first
                // bag card that fits put on it.
                if flag == "shop-turrets" {
                    r.pick(slime_mould::mould::run::Pick::EditTurrets);
                    if let Some(i) = (0..r.cards.len()).find(|&i| !r.cards[i].1 && r.fits(r.cards[i].0)) {
                        r.pick(slime_mould::mould::run::Pick::Card(i));
                    }
                }
            }
            m.shop = Some(shop_of(&r));
            m.run_live = true;
            // The run the shop shows is the app's, so a drag (`drag=`)
            // changes it as the window's would.
            if !drags.is_empty() {
                shop_run = Some(r);
            }
        }
        m.show(p);
        match flag {
            "shop-build" => {
                m.open_shop_tab_named("Buildings");
            }
            "shop-tools" => {
                m.open_shop_tab_named("Tools");
            }
            // Options' second and third tabs (11-15), as Tab opens them.
            "options-planet" | "options-mould" => {
                for _ in 0..if flag == "options-planet" { 1 } else { 2 } {
                    m.key(KeyCode::Tab, true);
                }
            }
            _ => {}
        }
        let mut extra = extra.split('@');
        if let Some(tab) = extra.next().filter(|t| !t.is_empty()) {
            if !m.open_shop_tab_named(tab) {
                return Err(format!("menu=...@{tab}: no such shop tab"));
            }
        }
        if let Some(name) = extra.next() {
            if !m.pick_named(name) {
                return Err(format!("menu=...@{name}: no such row on the tab"));
            }
        }
        app.menu = Some(m);
    }
    if shop_run.is_some() {
        app.run = shop_run;
    }
    // `hover=X,Y` over a menu page: its pointer there (the shop's detail
    // box describes the card under it).
    if let (Some(m), Some(p), true) = (app.menu.as_mut(), hover, drags.is_empty()) {
        m.moved(Some(p));
    }
    // `drag=`: where each drag starts and ends on the screen, found once
    // the shop is laid out (the slots stay put as cards move).
    let mut drag_path: Vec<((f32, f32), (f32, f32))> = Vec::new();
    // `go` (the range's armory closed): the drags share the frames before
    // the film's first whole second, the round plays after. A drag from a
    // gun to itself is a click on it.
    let go = drags.iter().any(|d| d.0 == "go");
    let drags: Vec<&(String, String)> = drags.iter().filter(|d| d.0 != "go").collect();
    let still = if go { at.iter().take_while(|&&t| t < at[0].floor() + 1.0).count() } else { at.len() };
    if !drags.is_empty() {
        // The shop's tab that holds the first card dragged (Shop B, 11-15:
        // the cards for sale are on a tab of their own).
        if let (Some(m), Some(a)) = (app.menu.as_mut(), drags.iter().map(|d| d.0.as_str()).find(|a| a.parse::<usize>().is_err() && *a != "bag")) {
            m.open_shop_tab_named(a);
        }
        app.prepare();
        let m = app.menu.as_ref().ok_or("drag= needs menu=shop..")?;
        let at = |s: &str| match s.parse::<usize>() {
            Ok(k) => m.slot_at(k.saturating_sub(1)),
            Err(_) if s == "bag" => m.bag_at(),
            Err(_) => m.card_at(s),
        };
        for &(a, b) in &drags {
            drag_path.push((at(a).ok_or_else(|| format!("drag=: no {a} in the shop"))?, at(b).ok_or_else(|| format!("drag=: no {b} in the shop"))?));
        }
    }
    if wav.is_some() {
        app.tape = Some((VoiceBank::new(48_000, app.args.seed), Vec::new()));
    }
    // The recording's ear: on the squad, at the window's opening zoom.
    let span = app.camera.span();
    let mut voice_at = u64::MAX;
    let step = { let r = Rules::default(); r.frame_ticks as f32 / r.tps as f32 };
    let mut last_sec = u64::MAX;
    // `hand=` a player AI style (boom, claim, late, fort, strike) plays
    // through `larder_world_ai`, as the census does, so a campaign mission's
    // pictures show the hands its numbers were set on.
    let mut ai = match hand {
        "rush" | "farm" | "stand" | "volley" | "edge" | "squads" => None,
        _ => app.game.as_ref().and_then(|g| slime_mould::mould::larder_world_ai::Hand::named(hand, g)),
    };
    // Each hit: where, its tick, the body there the tick before, and the
    // ticks it took to refill.
    type Hole = ((f32, f32), u64, f32, Option<u64>);
    let mut holes: Vec<Hole> = Vec::new();
    let mut prev_lk: Vec<u8> = Vec::new();
    // `cut=T,X,Y,R` (2026-11-13, the slime-enemy thread: does a part cut
    // off from every crystal wither?): at game time T, a ring 6 world cells
    // wide (the fine grid, as `centre=` and the squad's places) from R out
    // round (X, Y) goes off the agar for the rest of the round (`Engine::block`, as a wall the game built but
    // with no health), so the cut is held. Each game second after, the
    // walkers inside the ring (the look's walker count, channel 1) are
    // printed against their count at the cut.
    let mut cut_inside: Option<(Vec<usize>, u32)> = None;
    let mut cut_sec = u64::MAX;
    let mut watch_sec = u64::MAX;
    // `crustlook=T` (2026-11-17): the offline crust a cell, and the second
    // it was last stepped.
    let mut crust: Vec<f32> = Vec::new();
    let mut crust_walk: Vec<u8> = Vec::new();
    let mut crust_sec = u64::MAX;
    // With HOLD (`cut=T,X,Y,R,HOLD`), the ring's agar cells go back on the
    // agar HOLD game seconds after the cut, so a piece that has gone stale
    // inside can leave (does a cut-off part panic?).
    let mut ring_back: Option<(f32, Vec<usize>)> = None;
    // The soldiers `cutsquad=` set down, kept there.
    let mut squad_set: Option<Vec<usize>> = None;
    let mut boomed: Vec<usize> = Vec::new();
    let mut lit: Vec<usize> = Vec::new();
    let mut bossed: Vec<usize> = Vec::new();
    // `turret=X,Y,T`: the buildings still to stand, at their seconds.
    let mut late: Vec<usize> = (0..turrets.len()).filter(|&i| turrets[i].2 > 0.0).collect();
    let inside = |lk: &[u8], cells: &[usize]| -> u32 { cells.iter().map(|&i| lk[4 * i + 1] as u32).sum() };
    for (fi, &t) in at.iter().enumerate() {
        if fi > 0 && scroll != 0 {
            if let Some(m) = app.menu.as_mut() {
                m.wheel(-scroll);
            }
        }
        // `drag=`: frame `fi`'s share of its drag: the pointer on the card,
        // pressed, carried (eased) to its place, let go at seven tenths,
        // then resting there.
        if go && fi == still {
            println!("drag: frame {fi} go");
            app.menu_event(slime_mould::world::start::StartEvent::NextLevel)?;
        }
        if !drag_path.is_empty() && fi < still {
            let per = still.div_ceil(drag_path.len()).max(4);
            let (k, j) = (fi / per, fi % per);
            if let (Some(&(a, b)), Some(m)) = (drag_path.get(k), app.menu.as_mut()) {
                let held = (per as f32 * 0.7) as usize;
                let u = ((j as f32 - 1.0) / (held as f32 - 2.0).max(1.0)).clamp(0.0, 1.0);
                let e = u * u * (3.0 - 2.0 * u);
                let p = (a.0 + (b.0 - a.0) * e, a.1 + (b.1 - a.1) * e - 30.0 * (std::f32::consts::PI * e).sin());
                let mut events = Vec::new();
                match j {
                    0 => m.moved(Some(a)),
                    1 => {
                        m.moved(Some(a));
                        m.button(slime_mould::world::Button::Left, true);
                    }
                    j if j < held => m.moved(Some(p)),
                    j if j == held => {
                        m.moved(Some(b));
                        events = m.button(slime_mould::world::Button::Left, false);
                        // A click picks (Shop B); Enter then buys it.
                        if a == b && events.is_empty() {
                            events = m.key(winit::keyboard::KeyCode::Enter, true);
                        }
                    }
                    _ => m.moved(Some(b)),
                }
                println!("drag: frame {fi} pointer {:.0},{:.0} {}", if j == 0 { a.0 } else if j < held { p.0 } else { b.0 }, if j == 0 { a.1 } else if j < held { p.1 } else { b.1 }, if (1..held).contains(&j) { "down" } else { "up" });
                for ev in events {
                    println!("drag: {ev:?}");
                    app.menu_event(ev)?;
                    if let Some(r) = app.range.as_ref().filter(|_| app.armory).or(app.run.as_ref()) {
                        println!("drag: the gun fires {}", r.gun_text());
                    }
                }
            }
        }
        // `real=1` (D6, 11-15): past the first frame, one window frame of
        // `t` less the last frame's time in real seconds, so a hit-stop
        // holds the game still across frames as the window does.
        let real = app.args.real && fi > 0;
        let mut once = real;
        // A mission's verdict stops the steps, as the game's end does.
        while app.game.as_ref().is_some_and(|g| (once || (!real && g.time() < t)) && g.end.is_none()) && app.mission.as_ref().is_none_or(|m| m.verdict.is_none()) {
            once = false;
            let g = app.game.as_mut().unwrap();
            let sec = g.world.ticks() / g.rules.tps as u64;
            if sec != last_sec {
                last_sec = sec;
                match ai.as_mut() {
                    Some(h) => {
                        h.play(g);
                    }
                    None => plan(g, hand),
                }
            }
            if let (Some(Cut { t: ct, x: cx, y: cy, r: cr, width: cw, .. }), None) = (cut, cut_inside.as_ref()) {
                let g = app.game.as_mut().unwrap();
                if g.time() >= ct {
                    let (w, h) = (g.world.e.dish.w, g.world.e.dish.h);
                    let (mut ring, mut core) = (Vec::new(), Vec::new());
                    for i in 0..w * h {
                        let d = ((i % w) as f32 + 0.5 - cx).hypot((i / w) as f32 + 0.5 - cy);
                        if d > cr && d <= cr + cw {
                            ring.push(i);
                        } else if d <= cr {
                            core.push(i);
                        }
                    }
                    if let Some(c) = cut.filter(|c| c.hold.is_finite()) {
                        let agar: Vec<usize> = ring.iter().copied().filter(|&i| g.world.e.dish.agar[i]).collect();
                        ring_back = Some((g.time() + c.hold, agar));
                    }

                    g.world.e.block(&ring);
                    let n0 = inside(&read_back(&device, &queue, &g.world.e.look), &core);
                    println!("  cut t {:.1} s: ring of {} fine cells round ({cx}, {cy}) r {cr}; {n0} walkers inside", g.time(), ring.len());
                    cut_inside = Some((core, n0.max(1)));
                }
            }
            // `cutsquad=X,Y[,T[,N]]`: at T the first N living soldiers (a
            // squad short of N is made up first) are set down in a row there (a staged scene, as the ring is: a
            // squad walking there through C's body dies on the way); from
            // then every one of them not near it is sent back, after the
            // hand's orders, so it stays. The rest stay with the hand.
            if let Some((sx, sy, st, n)) = cut.and_then(|c| c.squad) {
                let g = app.game.as_mut().unwrap();
                if g.time() >= st && squad_set.is_none() {
                    // With N given, a squad short of N is made up first,
                    // bought at the larder with food the scene adds.
                    if n != usize::MAX {
                        while g.alive() < n.min(g.rules.max_squad) {
                            g.bank += g.rules.soldier_cost;
                            if !g.buy() {
                                break;
                            }
                        }
                    }
                    let set: Vec<usize> = (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).take(n).collect();
                    let sc = g.world.scale();
                    for (j, &k) in set.iter().enumerate() {
                        (g.soldiers[k].x, g.soldiers[k].y) = (sx + (j as f32 - (set.len() as f32 - 1.0) / 2.0) * 1.5 * sc, sy);
                    }
                    squad_set = Some(set);
                }
                if let Some(set) = squad_set.as_ref() {
                    for &k in set {
                        let near = (g.soldiers[k].x - sx).hypot(g.soldiers[k].y - sy) < 2.0 * g.world.scale();
                        if g.soldiers[k].alive && !near && g.soldiers[k].order != Order::Move(sx, sy) {
                            g.apply(Cmd::Order(k, Order::Move(sx, sy)));
                        }
                    }
                }
            }
            if ring_back.as_ref().is_some_and(|(at, _)| app.game.as_ref().is_some_and(|g| g.time() >= *at)) {
                let (_, cells) = ring_back.take().unwrap();
                let g = app.game.as_mut().unwrap();
                g.world.e.unblock(&cells);
                println!("  cut t {:.1} s: the ring opens ({} cells back on the agar)", g.time(), cells.len());
            }
            // `load=T,NAME`: every squad loads NAME once T has come.
            if let Some((lt, cards)) = load_at.take_if(|(lt, _)| app.game.as_ref().is_some_and(|g| g.time() >= *lt)) {
                let g = app.game.as_mut().unwrap();
                let deck = rounds::Deck::new(&rounds::parse(&cards).unwrap_or_default());
                for q in 0..g.squads.len() {
                    g.apply(Cmd::Load(q as u8, Some(deck)));
                }
                println!("  load t {:.2} s: {} squads load {} (asked for {lt} s)", g.time(), g.squads.len(), LOADOUTS.iter().find(|l| l.1 == cards).map_or(cards.as_str(), |l| l.0));
            }
            // `tload=T,NAME`: every turret loads NAME once T has come.
            if let Some((lt, cards)) = tload_at.take_if(|(lt, _)| app.game.as_ref().is_some_and(|g| g.time() >= *lt)) {
                let g = app.game.as_mut().unwrap();
                let deck = rounds::Deck::new(&rounds::parse(&cards).unwrap_or_default());
                g.apply(Cmd::Load(slime_mould::mould::larder_world::TURRET_DECK, Some(deck)));
                println!("  tload t {:.2} s: the turrets load {} (asked for {lt} s)", g.time(), LOADOUTS.iter().find(|l| l.1 == cards).map_or(cards.as_str(), |l| l.0));
            }
            // `turret=X,Y,T`: standing whole once T has come.
            if let Some(g) = app.game.as_mut() {
                let now = g.time();
                for i in late.iter().copied().filter(|&i| now >= turrets[i].2).collect::<Vec<_>>() {
                    let (kind, at, _) = turrets[i];
                    stand(g, kind, at, renders.contains(&i))?;
                    println!("  {} t {now:.2} s: standing at {},{}", kind.name(), at.0, at.1);
                    late.retain(|&j| j != i);
                }
            }
            // `boom=T,X,Y`: the round set off there once T has come.
            for (j, (bt, bx, by, cards)) in booms.iter().enumerate() {
                let (bt, bx, by) = (*bt, *bx, *by);
                let g = app.game.as_mut().unwrap();
                if g.time() >= bt && !boomed.contains(&j) {
                    boomed.push(j);
                    let (k0, f0, s0) = (app.rounds.killed, app.rounds.flung, app.rounds.gassed);
                    match cards {
                        Some(c) => app.rounds.cast_cards_at(g, (bx, by), c),
                        None => app.rounds.cast_at(g, (bx, by)),
                    }
                    println!("  boom t {:.2} s at {bx},{by}: {} killed, {} flung, {} gassed", g.time(), app.rounds.killed - k0, app.rounds.flung - f0, app.rounds.gassed - s0);
                }
            }
            // `light=T,X,Y,R`: a plain fire lit there once T has come.
            for (j, &(lt, lx, ly, r)) in app.args.lights.iter().enumerate() {
                let g = app.game.as_mut().unwrap();
                if g.time() >= lt && !lit.contains(&j) {
                    lit.push(j);
                    let b = slime_mould::mould::squad::Burst { at: (lx, ly), r, burn: true, ..Default::default() };
                    g.world.burst(&b, &slime_mould::mould::squad::BurstRules::default());
                    println!("  fire t {:.2} s lit at {lx},{ly} over {r}", g.time());
                }
            }
            // `boss=T,X,Y`: a mini-boss forms there once T has come, on
            // the first free lump.
            for (j, &(bt, bx, by)) in bosses.iter().enumerate() {
                let g = app.game.as_mut().unwrap();
                if g.time() >= bt && !bossed.contains(&j) {
                    bossed.push(j);
                    let reach = g.rules.boss.reach;
                    if g.form_boss((bx, by), reach, slime_mould::mould::larder_world::BossCause::Staged) {
                        let b = g.bosses.live.last().unwrap();
                        println!("  boss t {:.2} s: a {} formed round {bx},{by}, grade {:.2}", g.time(), b.kind.name(), b.grade);
                    }
                }
            }
            let surges = app.game.as_ref().map_or(0, |g| g.surges);
            let raids = app.game.as_ref().map_or(0, |g| g.raids.sent.len());
            let boss_sec = app.game.as_ref().map_or(0, |g| g.time().floor() as u64);
            let bosses_ended = app.game.as_ref().map_or(0, |g| g.bosses.ended.len());
            let bosses_born: Vec<u64> = app.game.as_ref().map_or(Vec::new(), |g| g.bosses.live.iter().map(|b| b.born).collect());
            app.step(if real { t - at[fi - 1] } else { step });
            // Whole steps here (the window leaves a step running while it
            // draws): the pictures, the holes and the tape read the game as
            // they did before the step split.
            app.settle();
            // Each surge the game starts (food-driven surges, 11-15).
            if let Some(g) = app.game.as_ref().filter(|g| g.surges != surges) {
                println!("  surge {} starts at t {:.1} s (the slime has eaten {:.0} food)", g.surges, g.time(), g.slime_ate);
            }
            // Each raid the game sends (11-15).
            if let Some(g) = app.game.as_ref().filter(|g| g.raids.sent.len() != raids) {
                let d = g.raids.sent.last().unwrap();
                println!("  raid {} at t {:.1} s: {:?} at {:.0},{:.0}, reach {:.0} cells holding {} walkers, share {:.2}, size {:.0}% of the slime{}", g.raids.sent.len(), g.time(), d.aim, d.to.0, d.to.1, d.reach, d.held, d.share, d.size * 100.0, if d.last_stand { " (last stand)" } else { "" });
            }
            // Each mini-boss that forms by its rules' chances.
            if let Some(g) = app.game.as_ref() {
                for b in g.bosses.live.iter().filter(|b| !bosses_born.contains(&b.born) && b.cause != slime_mould::mould::larder_world::BossCause::Staged) {
                    let r = &g.rules.boss;
                    println!("  boss t {:.1} s: {:?} {} formed on lump {} at {:.0},{:.0} (crowd top {:.2}); grade {:.2}: {} blows a member, touches as {}, pace x{:.2}", g.time(), b.cause, b.kind.name(), b.lump, b.at.0, b.at.1, g.bosses.crowd_top, b.grade, r.armour_at(b.grade), 1 + r.bite_at(b.grade), r.haste_at(b.grade));
                }
            }
            // Each mini-boss once a game second, and each one that ends
            // (`bosses.rs`, 2026-11-16): its members, centre and how far its
            // nearest soldier is (world units, as `centre=`).
            if let Some(g) = app.game.as_ref().filter(|g| g.time().floor() as u64 != boss_sec) {
                // How tight each one is: its members' mean distance from
                // its centre over a packed disc's (a walker a fine cell, 1
                // packed), from the walkers read back (a look's cost only).
                let (ws, dw) = if g.bosses.live.is_empty() { (Vec::new(), Vec::new()) } else { (slime_mould::gpu::floats(&read_back(&device, &queue, &g.world.e.walkers)), g.world.e.dense_of()) };
                for b in &g.bosses.live {
                    let near = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x - b.centre.0).hypot(s.y - b.centre.1)).fold(f32::INFINITY, f32::min);
                    let ds: Vec<f32> = (0..dw.len().min(ws.len() / 4)).filter(|&k| (dw[k] >> slime_mould::mould::big::LUMP_SHIFT) & 7 == b.lump as u32 + 1).map(|k| (ws[4 * k] - b.centre.0).hypot(ws[4 * k + 1] - b.centre.1)).collect();
                    let hurt = (0..dw.len().min(ws.len() / 4)).filter(|&k| (dw[k] >> slime_mould::mould::big::LUMP_SHIFT) & 7 == b.lump as u32 + 1 && dw[k] >> slime_mould::mould::big::HURT_SHIFT > 0).count();
                    let packed = 2.0 / 3.0 * (ds.len() as f32 / std::f32::consts::PI).sqrt();
                    let tight = ds.iter().sum::<f32>() / ds.len().max(1) as f32 / packed.max(1.0);
                    println!("  boss t {:.0} s: lump {} {} {:?}, {} members (first {}, {hurt} hurt), at {:.0},{:.0}, spread {tight:.2}, stolen {:.1}; nearest soldier {near:.1}, {} alive", g.time(), b.lump, b.kind.name(), b.phase, b.members, b.first, b.centre.0, b.centre.1, b.stolen, g.alive());
                    // A ring (`big::Lump::ring`, a leech round its pump):
                    // its members' distance from what it sits on, and how
                    // many lie in each eighth round it (from the east, the
                    // way the angle turns).
                    if let (slime_mould::mould::larder_world::BossPhase::Sit(p, _), true) = (b.phase, b.kind == slime_mould::mould::larder_world::BossKind::Leech) {
                        let mut eighths = [0u32; 8];
                        let mut d: Vec<f32> = Vec::new();
                        for k in (0..dw.len().min(ws.len() / 4)).filter(|&k| (dw[k] >> slime_mould::mould::big::LUMP_SHIFT) & 7 == b.lump as u32 + 1) {
                            let (dx, dy) = (ws[4 * k] - p.0, ws[4 * k + 1] - p.1);
                            d.push(dx.hypot(dy));
                            eighths[((dy.atan2(dx) / std::f32::consts::TAU + 1.0) * 8.0) as usize % 8] += 1;
                        }
                        d.sort_by(f32::total_cmp);
                        let q = |f: f32| d.get(((d.len() as f32 - 1.0) * f) as usize).copied().unwrap_or(0.0);
                        let cr = g.buildings.iter().find(|w| w.alive && (w.at.0 - p.0).hypot(w.at.1 - p.1) < 1.0).and_then(|w| w.crystal).map_or(0.0, |c| g.world.crystal_r(c));
                        println!("    ring: crystal radius {cr:.1}, rig {:.1}, ring {:.1}; from the pump {:.1} / {:.1} / {:.1} (10% / median / 90%); by eighths {eighths:?}", 1.3 * cr, g.rules.boss.ring * cr, q(0.1), q(0.5), q(0.9));
                    }
                }
                for (b, e, _) in &g.bosses.ended[bosses_ended..] {
                    println!("  boss t {:.0} s: lump {} {} ended {e:?} ({} members of {}; stole {:.1}, {:.1} given back so far)", g.time(), b.lump, b.kind.name(), b.members, b.first, b.stolen, g.bosses.returned);
                }
            }
            if let Some((core, n0)) = cut_inside.as_ref() {
                let g = app.game.as_ref().unwrap();
                // A step can run several ticks, so a second is counted when
                // it starts, not on a tick that is a multiple of `tps`.
                let sec = g.time().floor() as u64;
                if sec != cut_sec {
                    cut_sec = sec;
                    let n = inside(&read_back(&device, &queue, &g.world.e.look), core);
                    println!("  cut t {sec} s: {n} walkers inside ({:.2} of the cut's)", n as f32 / *n0 as f32);
                }
            }
            // `watch=X,Y,R`: the walkers in the disc and the ring round it,
            // each game second (as the cut's).
            if let Some((wx, wy, wr)) = app.args.watch {
                let g = app.game.as_ref().unwrap();
                let sec = g.time().floor() as u64;
                if sec != watch_sec {
                    watch_sec = sec;
                    let w = g.world.e.dish.w;
                    let lk = read_back(&device, &queue, &g.world.e.look);
                    let (mut disc, mut ring) = (0u32, 0u32);
                    for (i, c) in lk.chunks_exact(4).enumerate() {
                        let d = ((i % w) as f32 + 0.5 - wx).hypot((i / w) as f32 + 0.5 - wy);
                        if d <= wr {
                            disc += c[1] as u32;
                        } else if d <= 2.0 * wr {
                            ring += c[1] as u32;
                        }
                    }
                    // With contagion on (a germ cast), the sick walkers
                    // (living, past the immune count) within 2R too, for
                    // the readers' films (2026-11-17). Appended, so the
                    // Mind films' parser still matches.
                    let e = &g.world.e;
                    let sick = (g.world.s.sick > 0).then(|| {
                        let n = e.n as usize;
                        let ill: Vec<u32> = read_back(&device, &queue, &e.sick).chunks_exact(4).take(n).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
                        let spec: Vec<u32> = read_back(&device, &queue, &e.spec).chunks_exact(4).take(n).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
                        let wk: Vec<f32> = read_back(&device, &queue, &e.walkers).chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect();
                        // A freed slot (growth) has energy below 0.
                        let en: Vec<f32> = read_back(&device, &queue, &e.energy).chunks_exact(4).take(n).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect();
                        (0..n).filter(|&j| ill[j] > g.world.s.immune && (spec[j] >> 9) & 3 != 3 && en[j] >= 0.0 && (wk[4 * j] - wx).hypot(wk[4 * j + 1] - wy) <= 2.0 * wr).count()
                    });
                    let sick = sick.map_or(String::new(), |c| format!(", {c} sick within {}", 2.0 * wr));
                    println!("  watch t {sec} s: {disc} walkers within {wr} of {wx},{wy}, {ring} from {wr} to {}{sick}", 2.0 * wr);
                }
            }
            // `crustlook=T`: crust from the look once a game second (no
            // engine rule; see `Args::crust_look`).
            if let Some(tmin) = app.args.crust_look {
                let g = app.game.as_ref().unwrap();
                let sec = g.time().floor() as u64;
                if sec != crust_sec {
                    crust_sec = sec;
                    let lk = read_back(&device, &queue, &g.world.e.look);
                    crust.resize(lk.len() / 4, 0.0);
                    crust_walk = lk.chunks_exact(4).map(|c| c[1]).collect();
                    for (v, c) in crust.iter_mut().zip(lk.chunks_exact(4)) {
                        *v = if c[1] > 0 && c[0] >= tmin.max(1) { (*v + 1.0 / 60.0).min(1.0) } else { *v * 0.9942 };
                    }
                }
            }
            // `holes=1`: how long a hit's hole stays open (owner, 04:38Z:
            // "maybe part of the issue is the slime/walkers move to fast").
            // The drawn body (the view's `body`: trail x living walkers in
            // the 5 x 5 round each cell) is averaged over each hit's strike
            // disc, and a hole counts as refilled when that is back to 80% of
            // the tick before the hit. A frame's readback a tick: slow.
            if time_holes {
                let g = app.game.as_ref().unwrap();
                let (w, h) = (g.world.e.dish.w, g.world.e.dish.h);
                let lk = read_back(&device, &queue, &g.world.e.look);
                let body = |lk: &[u8], x: i32, y: i32| -> f32 {
                    let cl = |x: i32, y: i32| (y.clamp(0, h as i32 - 1) as usize) * w + x.clamp(0, w as i32 - 1) as usize;
                    let mut n = 0u32;
                    for dy in -2..=2 { for dx in -2..=2 { n += lk[4 * cl(x + dx, y + dy) + 1] as u32; } }
                    lk[4 * cl(x, y)] as f32 / 255.0 * (1.0 - (-(n as f32) / 6.0).exp())
                };
                let disc = |lk: &[u8], cx: f32, cy: f32, r: f32| -> f32 {
                    let (mut sum, mut k) = (0.0, 0);
                    let ri = r.ceil() as i32;
                    for dy in -ri..=ri { for dx in -ri..=ri { if ((dx * dx + dy * dy) as f32) <= r * r { sum += body(lk, cx as i32 + dx, cy as i32 + dy); k += 1; } } }
                    sum / k as f32
                };
                let tick = g.world.ticks();
                let r = g.rules.strike * g.world.scale();
                for t in app.tracers.iter().filter(|t| t.age == 0.0 && t.hit) {
                    let before = if prev_lk.is_empty() { f32::NAN } else { disc(&prev_lk, t.to.0, t.to.1, r) };
                    holes.push((t.to, tick, before, None::<u64>));
                }
                for hl in holes.iter_mut() {
                    if hl.2.is_nan() { continue; }
                    let v = disc(&lk, hl.0 .0, hl.0 .1, r);
                    if hl.3.is_none() && tick > hl.1 && v >= 0.8 * hl.2.max(0.05) { hl.3 = Some(tick - hl.1); }
                }
                prev_lk = lk;
            }
            if app.tape.is_some() {
                let g = app.game.as_ref().unwrap();
                let alive: Vec<(f32, f32)> = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
                let n = alive.len().max(1) as f32;
                let centre = if alive.is_empty() { g.larder } else { (alive.iter().map(|p| p.0).sum::<f32>() / n, alive.iter().map(|p| p.1).sum::<f32>() / n) };
                app.hear(step, Ear { centre, half: (span.0 / 2.0, span.1 / 2.0) });
                // The mould's voice's inputs every 5 game seconds, beside
                // the recording.
                let g = app.game.as_ref().unwrap();
                let sec = (g.time() + 1e-3) as u64;
                if sec.is_multiple_of(5) && sec != voice_at {
                    voice_at = sec;
                    let s = app.listener.as_ref().unwrap().stats;
                    println!("  voice t {sec:>3} s: mould {}, growth {:+.4}/s, hunger {:.2}, eating {:.5}/s, kills {:.0}/s, inhales {}", g.mould, s.growth, s.hunger, s.eating, s.kills_per_sec, s.inhales);
                }
            }
        }
        if time_holes {
            let done: Vec<u64> = holes.iter().filter_map(|h| h.3).collect();
            let open = holes.iter().filter(|h| h.3.is_none()).count();
            let mut d = done.clone(); d.sort();
            let tps = app.game.as_ref().unwrap().rules.tps as f32;
            if !d.is_empty() { println!("  holes: shots that hit {} refilled {} (still open {open}); ticks to 80%: median {} ({:.2} s), q25 {}, q75 {}, max {}", holes.len(), d.len(), d[d.len() / 2], d[d.len() / 2] as f32 / tps, d[d.len() / 4], d[3 * d.len() / 4], d[d.len() - 1]); }
        }
        let g = app.game.as_ref().unwrap();
        let rd = &app.rounds;
        // Squads with their own cards (`load=`) and loaded turrets (`tload=`) count here too. A new count
        // goes last, so the reports' parsers (`raw/make_clips.py`) still match.
        if !rd.round.is_empty() || g.squads.iter().any(|s| s.deck.is_some()) || g.turret_deck.is_some() {
            println!("  rounds t {:>4.0} s: {} casts (blast, gas, germ, bomb {:?}), {} walkers killed, {} flung, {} not on this world, {} lodged, {} trail bursts, {} carried casts let out, {} walkers gassed, {} shoves, {} killed by turrets, {} casts beside another, {} batches of {} bursts, {} seeker turns, {} soldier hits in {} cells, {} ran out of range and {} hit a wall with no slime struck, {} pulls too near to pull, {} Mind casts, {} reader casts, {} flares", g.time(), rd.cast.iter().sum::<u32>(), &rd.cast[..4], rd.killed, rd.flung, rd.unsupported, rd.waiting(), rd.trailed, g.releases, rd.gassed, rd.pushes, g.turret_killed, g.multicast, rd.batches, rd.batched, g.seek_turns, g.hits, g.hit_cells, g.missed_range, g.missed_wall, rd.short, rd.cast[4], rd.cast[5], rd.cast[6]);
        }
        if g.shells_fired > 0 {
            let next = g.shells.last().map_or(String::new(), |s| format!(", the newest coming down at {:.0},{:.0}", s.to.0, s.to.1));
            println!("  mortar t {:>4.0} s: {} shells fired, {} landed, {} in the air, {} walkers killed by their strikes{next}", g.time(), g.shells_fired, g.shells_landed, g.shells.len(), g.mortar_killed);
        }
        // The view on the fight: the squad's middle, or the larder; or,
        // with `centre=X,Y`, one place for every frame, so a run of frames
        // plays as a clip from a still camera (the crater looks in motion,
        // 2026-11-12: a camera on the squad pans as it walks and jumps to
        // the larder when it dies).
        let alive: Vec<(f32, f32)> = g.soldiers.iter().filter(|s| s.alive).map(|s| (s.x, s.y)).collect();
        let squad = (!alive.is_empty()).then(|| (alive.iter().map(|p| p.0).sum::<f32>() / alive.len() as f32, alive.iter().map(|p| p.1).sum::<f32>() / alive.len() as f32));
        // The squad mode follows the controlled squad, as the window does.
        let squad = g.active.and_then(|q| g.middle(q)).filter(|_| g.rules.squads).or(squad);
        // A picture between two steps (a frame sequence at more frames
        // than steps; gun look, 2026-11-15): the game is at the step past
        // `t`, so the picture shows it as the window would `t` into it.
        // Bullets glide that far along their legs (`App::between`), and
        // flashes, kicks and hit lights are that much older; the notes go
        // back once it is taken.
        let step_s = g.rules.frame_ticks as f32 / g.rules.tps as f32;
        let into = (1.0 - (g.time() - t) / step_s).clamp(0.0, 1.0);
        let notes = (app.looks.clone(), app.tracers.clone());
        if !app.args.real {
            app.sub = Some(into);
            app.age_notes(into * step_s);
        }
        let g = app.game.as_ref().unwrap();
        // The oldest, live or still fading, so a film stays on a broken
        // one while its look drains rather than jumping to a newer one.
        let boss = g.bosses.live.iter().chain(g.bosses.fading.iter().map(|f| &f.0)).min_by_key(|b| b.born).filter(|_| app.args.follow_boss).map(|b| b.centre);
        let centre = boss.or(fixed.map(|f| (f.0, f.1))).or(squad).unwrap_or(g.larder);
        app.controls.selected = match g.active.filter(|_| g.rules.squads) {
            Some(q) => g.members(q),
            None if select => (0..g.soldiers.len()).collect(),
            None => Vec::new(),
        };
        if let Some(step) = app.args.card.get(fi).cloned() {
            card_step(&mut app, &step)?;
        }
        if app.sq.as_ref().is_some_and(|q| q.free) {
            app.controls.selected.clear();
        }
        app.hud.moved(hover);
        app.controls.cursor = hover;
        // `hand=buildq` with `hover=`: the turret is being placed under the
        // cursor (Q, then T), so the picture shows the placing ghost and its
        // reach ring (2026-11-14, the build-flow thread).
        if hand == "buildq" && hover.is_some() && app.hud.targeting().is_none() {
            app.build_page = true;
            app.prepare();
            let _ = app.hud.key(KeyCode::KeyT, true, false, &mut app.camera);
            app.controls.cursor = hover;
        }
        // `hand=throw` with `hover=`: a blast being aimed under the cursor
        // (the Tools page, then B), so the picture shows the throw's
        // ghost: reach, arc, area (2026-11-16, the throw-aim thread).
        // `hover=X,Y,X2,Y2` sweeps the cursor from the one to the other over
        // the frames, so a clip shows the aim leaving the reach.
        // `hand=throwgas` aims the sleep gas instead (S), for its colour.
        if hand.starts_with("throw") && hover.is_some() && app.hud.targeting().is_none() {
            app.tools_page = true;
            app.prepare();
            let key = if hand == "throwgas" { KeyCode::KeyS } else { KeyCode::KeyB };
            let _ = app.hud.key(key, true, false, &mut app.camera);
        }
        if let (Some(a), Some(b)) = (hover, hover_to) {
            let f = if at.len() > 1 { fi as f32 / (at.len() - 1) as f32 } else { 0.0 };
            let p = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
            app.hud.moved(Some(p));
            app.controls.cursor = Some(p);
        }
        // The HUD sets the camera's insets as it draws, and `fit` reads
        // them: without this the run's first "all" picture was zoomed in
        // past the world's edges (fit with no insets).
        app.prepare();
        for tag in ["all", "near"] {
            let zoom = if tag == "all" { app.camera.fit() } else { fixed.map_or(6.0, |f| f.2) };
            app.camera.zoom_to(zoom, app.camera.to_screen(app.camera.centre()));
            app.camera.centre_on(if tag == "all" { (app.camera.world.0 / 2.0, app.camera.world.1 / 2.0) } else { centre });
            if tag == "near" {
                let (sx, sy) = app.shake();
                (app.camera.x, app.camera.y) = (app.camera.x + sx, app.camera.y + sy);
            }
            app.prepare();
            let (view, cp) = (app.view.as_ref().unwrap(), app.canvas_pass.as_mut().unwrap());
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("shot") });
            view.record(&mut enc, &app.camera, None);
            cp.record(&mut enc, &view.out, view.stride);
            queue.submit([enc.finish()]);
            // Whole seconds keep their old names; a time between (a frame
            // sequence) is named in milliseconds.
            let path = if t.fract() == 0.0 { format!("{dir}/{}-{hand}-{:03}s-{tag}.png", file_part(&app.name), t as u32) } else { format!("{dir}/{}-{hand}-{:05}ms-{tag}.png", file_part(&app.name), (t * 1000.0) as u32) };
            view.picture().save(&path).map_err(|e| format!("{path}: {e}"))?;
            println!("{path}");
        }
        if app.args.crust_look.is_some() && !crust.is_empty() {
            // The crust a cell, as the world map (a pixel a cell): black to
            // white by crust; a cell with a walker on it now tinted orange,
            // so the share of crust under the body shows.
            let w = app.game.as_ref().unwrap().world.e.dish.w;
            let h = crust.len() / w;
            let img = image::RgbImage::from_fn(w as u32, h as u32, |x, y| {
                let i = y as usize * w + x as usize;
                let v = (crust[i].clamp(0.0, 1.0) * 255.0) as u8;
                if crust_walk[i] > 0 { image::Rgb([v.max(90), v / 2 + 40, 0]) } else { image::Rgb([v, v, v]) }
            });
            let path = format!("{dir}/{}-{hand}-{:05}ms-crust.png", file_part(&app.name), (t * 1000.0) as u32);
            img.save(&path).map_err(|e| format!("{path}: {e}"))?;
            let held = |lo: f32| crust.iter().filter(|&&v| v > lo).count();
            let under = crust.iter().zip(&crust_walk).filter(|(v, n)| **v > 0.3 && **n > 0).count();
            let (c05, c30, c95) = (held(0.05), held(0.3), held(0.95));
            // The engine's own crust (`crust=1`), beside the offline one.
            let ec = app.game.as_ref().unwrap().world.e.crust_of();
            if !ec.is_empty() {
                let eh = |lo: f32| ec.iter().filter(|&&v| v > lo).count();
                // Where the walkers stand now: on old ground (crust over
                // 0.3) or new (under 0.05), the E1b films' measure.
                let on = |f: &dyn Fn(f32) -> bool| ec.iter().zip(&crust_walk).filter(|(v, n)| f(**v) && **n > 0).count();
                println!("  engine crust t {t:.0} s: {} cells over 0.05, {} over 0.3, {} over 0.95; walkers' cells on crust over 0.3 {}, under 0.05 {}", eh(0.05), eh(0.3), eh(0.95), on(&|v| v > 0.3), on(&|v| v < 0.05));
                // The scar under walkers and the set walkers (E2a's films).
                let gw = &app.game.as_ref().unwrap().world;
                if gw.s.scar_keep > 0.0 && gw.s.alarm_keep > 0.0 {
                    let sc = gw.e.scar_of();
                    let mut under: Vec<f32> = sc.iter().zip(&crust_walk).filter(|(_, n)| **n > 0).map(|(v, _)| *v).collect();
                    under.sort_by(f32::total_cmp);
                    let q = |f: f32| under.get(((under.len() as f32 - 1.0) * f) as usize).copied().unwrap_or(0.0);
                    let set = gw.e.set_of().iter().filter(|&&w| w > 0).count();
                    let bw = app.game.as_ref().unwrap().bulwark.iter().filter(|&&b| b).count();
                    println!("  scar t {t:.0} s: under walkers p50 {:.3}, p90 {:.3}, p99 {:.3}, max {:.3}; {set} walkers set, bulwark {bw} cells", q(0.5), q(0.9), q(0.99), q(1.0));
                }
                // Fire and char (C2's films): cells burning, and char over
                // 0.3 (barren at `char_barren=0.3`).
                let e = &app.game.as_ref().unwrap().world.e;
                if e.plants {
                    let (_, _, fire, ch) = e.plants_of();
                    println!("  fire t {t:.0} s: {} cells burning, {} char over 0.3", fire.iter().filter(|&&v| v > 0.0).count(), ch.iter().filter(|&&v| v >= 0.3).count());
                }
            }
            println!("  crust t {t:.0} s: {c05} cells over 0.05, {c30} over 0.3 ({under} of those under walkers now, {:.0}%), {c95} over 0.95; walkers on {} cells", 100.0 * under as f32 / c30.max(1) as f32, crust_walk.iter().filter(|&&n| n > 0).count());
        }
        if !app.args.real {
            (app.looks, app.tracers) = notes;
        }
        app.sub = None;
        let g = app.game.as_ref().unwrap();
        let left: f32 = g.world.crystals().iter().map(|c| c.1).sum::<f32>() / app.charge0.iter().sum::<f32>().max(1.0);
        // The CPU work the window adds: the ground field (once a game
        // second) and the model and HUD layout (every frame).
        let cells = g.world.e.dish.w * g.world.e.dish.h;
        let t0 = Instant::now();
        let _ = ground_codes(g, &app.depth, Some(&vec![256.0; cells]), Some(&vec![100.0; cells]));
        let ground_ms = t0.elapsed().as_secs_f64() * 1e3;
        let t0 = Instant::now();
        for _ in 0..20 {
            let m = app.model(None, None);
            app.canvas.as_mut().unwrap().clear();
            app.hud.draw(&m, &app.controls, &app.camera, true, app.canvas.as_mut().unwrap(), &mut Marks::default());
        }
        println!("  CPU: ground field {ground_ms:.2} ms (once a game second), model and HUD {:.3} ms a frame", t0.elapsed().as_secs_f64() * 1e3 / 20.0);
        let g = app.game.as_ref().unwrap();
        if g.world.garden.is_some() {
            println!("  t {:.0} s: own mould {} (green), enemy {}", g.time(), g.world.counts_by_side().0, g.mould);
        }
        println!("  t {:.0} s: mould {}, soldiers {}, bank {:.0}, harvested {:.0}, rendered {:.0}, shots {}, killed {}, took {:.0}%, crystals left {:.3}{}{}", g.time(), g.mould, g.alive(), g.bank, g.harvested, g.rendered, g.shots, g.killed, g.take_share() * 100.0, left, g.end.map_or(String::new(), |e| format!(", ended: {}", e.band())), squad.map_or(String::new(), |(x, y)| format!("; squad at {x:.1},{y:.1}")));
        // What the slime has eaten, and the surges it built (11-15,
        // food-driven surges).
        println!("  surge: the slime ate {:.0} food ({:.1}/s); meter {:.1} of {:.0}, {} surges", g.slime_ate, g.slime_ate / g.time().max(1e-3), g.surge_meter, g.rules.surge_food, g.surges);
        // Condensed or thickened walkers (11-15, the thickening films):
        // how many, and the mould's mass (a condensed walker twice).
        if g.world.e.condenses {
            let e = &g.world.e;
            let words = e.dense_of();
            let energy = slime_mould::gpu::floats(&slime_mould::gpu::read_back(&e.device, &e.queue, &e.energy));
            let thick = words.iter().zip(&energy).filter(|&(&w, &v)| v >= 0.0 && w & slime_mould::mould::big::DENSE != 0).count();
            // Breakaways on their way (`roam`; specks of ordinary slime
            // since 11-16).
            let roaming = words.iter().zip(&energy).filter(|&(&w, &v)| v >= 0.0 && w & slime_mould::mould::big::ROAM != 0).count();
            println!("  dense: {thick} condensed or thick walkers, mass {} (readout {}); {roaming} breaking away", g.mould + thick, g.world.mass());
        }
        // What holds the live soldiers (2026-11-13, for the wading and
        // arming clips): walkers touching them, and their health.
        let live: Vec<_> = g.soldiers.iter().filter(|s| s.alive).collect();
        if !live.is_empty() {
            let n = live.len() as f32;
            println!("  squad: touching {:.1} mean, {} most; health {:.0} mean", live.iter().map(|s| s.touching as f32).sum::<f32>() / n, live.iter().map(|s| s.touching).max().unwrap_or(0), live.iter().map(|s| s.hp).sum::<f32>() / n);
        }
        for b in &g.buildings {
            println!("    {} at ({:.0}, {:.0}): hp {:.0} / {:.0}, built {:.1}, {}{}", b.kind.name(), b.at.0, b.at.1, b.hp, b.max_hp, b.built, if b.alive { "standing" } else { "worn down" }, if b.working { ", working" } else { "" });
        }
        if !g.buildings.is_empty() {
            println!("    pumped {:.0} food", g.pumped);
        }
    }
    if let (Some(path), Some((bank, tape))) = (wav, app.tape.as_ref()) {
        slime_mould::wav::write(Path::new(path), tape, 2, bank.sample_rate()).map_err(|e| format!("{path}: {e}"))?;
        let peak = tape.iter().fold(0.0f32, |m, v| m.max(v.abs()));
        let rms = (tape.iter().map(|v| v * v).sum::<f32>() / tape.len().max(1) as f32).sqrt();
        println!("{path}: {:.1} s, peak {:.3}, rms {:.4} ({:.1} dBFS)", tape.len() as f32 / 2.0 / bank.sample_rate() as f32, peak, rms, 20.0 * rms.max(1e-9).log10());
        let s = app.listener.as_ref().map(|l| l.stats).unwrap_or_default();
        println!(
            "  heard: shots {} (voiced {}), impacts {} (light {} mid {} heavy {}), kills {}, beats {}, hurts {}, fallen {}, banked {}, bought {}, walls {}, dry {}, cores {}, endings {}",
            s.shots, s.voiced, s.impacts, s.light, s.mid, s.heavy, s.kills, s.beats, s.hurts, s.fallen, s.banked, s.bought, s.walls, s.dry, s.cores, s.endings
        );
        println!(
            "  heard (C7): zips {}, sirens {}, blasts {}, pieces torn {} rejoined {} withered out {}, tells {} (charges {}); at the end {} pieces losing {:.0} walkers/s",
            s.zips, s.sirens, s.blasts, s.torn, s.rejoined, s.withered, s.tells, s.charges, s.pieces, s.withering
        );
        let b = bank.stats;
        println!("  played: grains {} (touch {} stolen {} kills {} wither {}, verbs {}), bursts {}, alerts {}, steals {}", b.grains, b.grains_by_texture[0], b.grains_by_texture[1], b.grains_by_texture[2], b.grains_by_texture[3], b.verb_grains, b.bursts, b.alerts, b.steals);
        let us = &app.hear_us;
        println!("  listener: mean {:.1} us, max {:.1} us a frame over {} frames", us.iter().sum::<f32>() / us.len().max(1) as f32, us.iter().fold(0.0f32, |a, &b| a.max(b)), us.len());
    }
    Ok(())
}

/// Notes for the soldiers the game made since the last call: one bought
/// (by the card or a replay) or the reserve's free recruit
/// (`Rules::reserve`), which `finish_step` makes.
pub(super) fn join_looks(g: &Game<GpuWorld>, looks: &mut Vec<Look>) {
    for s in &g.soldiers[looks.len()..] {
        looks.push(Look::new(s));
    }
}

/// The last step's shots as tracers, and their soldiers' flashes and
/// facing (before the next step walks them on). A recruit made in one step
/// of a frame can fire in the next step of the same frame (fast forward, or
/// a slow frame), before the frame's own `join_looks`, so its note joins
/// here (2026-11-12, after #114's panic on a recruit with no note).
pub(super) fn shot_marks(g: &Game<GpuWorld>, tracers: &mut Vec<Tracer>, looks: &mut Vec<Look>, listener: Option<&mut WorldListener>) {
    if let Some(l) = listener {
        l.shots(&g.last_shots);
    }
    join_looks(g, looks);
    let pace = g.rules.bullet_speed * g.world.scale();
    for (j, s) in g.last_shots.iter().enumerate() {
        // Each leg is where the bullet flew this frame (game state, not an
        // effect), so a bullet draws as a short streak moving out.
        let from = s.from;
        let to = s.hit.unwrap_or(s.to);
        // The drawn bullet glides along its leg at its own speed
        // (`App::between`), so its hit lights when it gets there.
        let delay = g.flown.get(j).filter(|_| pace > 0.0).map_or(0.0, |b| (to.0 - from.0).hypot(to.1 - from.1) / (pace * b.speed));
        tracers.push(Tracer { from, to, hit: s.hit.is_some(), age: 0.0, r: s.r, delay });
        if !s.fired {
            continue;
        }
        if let Some(k) = s.by.filter(|&k| k < looks.len() && g.soldiers.get(k).is_some_and(|q| q.alive)) {
            looks[k].flash = FLASH;
            looks[k].kick = KICK;
            looks[k].cast = s.cast;
            looks[k].shots = looks[k].shots.wrapping_add(1);
            looks[k].facing = (s.to.1 - s.from.1).atan2(s.to.0 - s.from.0);
            looks[k].turn_to = looks[k].facing;
        }
    }
    // A harvester taking from a crystal turns to face it and its hands
    // light (harvest look, 11-14); a shot this step still sets its facing
    // above, as firing wins.
    let cs = g.world.crystals();
    for t in &g.last_takes {
        if let (Some(k), Some(c)) = (t.by.filter(|&k| k < looks.len()), cs.get(t.crystal)) {
            looks[k].take = TAKE;
            if looks[k].flash <= 0.0 {
                looks[k].facing = (c.0 .1 - t.at.1).atan2(c.0 .0 - t.at.0);
                looks[k].turn_to = looks[k].facing;
            }
        }
    }
}

/// A note per building: a turret facing the mould's core when it is new,
/// a pump its crystal's middle (its hose into the crystal it drains).
fn join_turrets(g: &Game<GpuWorld>, turrets: &mut Vec<BuildingLook>) {
    let cs = g.world.crystals();
    for b in &g.buildings[turrets.len().min(g.buildings.len())..] {
        let to = match b.crystal.and_then(|c| cs.get(c)) {
            Some(c) if b.kind == Kind::Pump => c.0,
            _ => g.core,
        };
        turrets.push(BuildingLook { facing: (to.1 - b.at.1).atan2(to.0 - b.at.0), ..BuildingLook::default() });
    }
}

/// A turret that fired this step faces down its shot and kicks (turret
/// look, owner's playtest 11-14 08:15Z): the shot is the game's, from the
/// turret's own place.
pub(super) fn turret_fired(g: &Game<GpuWorld>, turrets: &mut Vec<BuildingLook>) {
    join_turrets(g, turrets);
    for s in g.turret_shots().filter(|s| s.fired) {
        // The game starts a turret's shot at its barrel tip
        // (`TURRET_MUZZLE` touch radii out, less for mould at its foot).
        let tip = TURRET_MUZZLE * g.rules.build.touch * g.world.scale() + 0.5;
        if let Some(k) = g.buildings.iter().position(|b| b.kind == Kind::Turret && b.alive && (b.at.0 - s.from.0).hypot(b.at.1 - s.from.1) < tip) {
            (turrets[k].facing, turrets[k].shot) = ((s.to.1 - s.from.1).atan2(s.to.0 - s.from.0), TURRET_SHOT);
        }
    }
    // A mortar that fired this step faces its newest shell's landing
    // point and sinks back (`Game::shells`, fired on the step's tick).
    let t = g.settled_ticks();
    for sh in g.shells.iter().filter(|sh| sh.fired + u64::from(g.rules.frame_ticks) > t) {
        if let Some(l) = turrets.get_mut(sh.mortar) {
            (l.facing, l.shot) = ((sh.to.1 - sh.from.1).atan2(sh.to.0 - sh.from.0), TURRET_SHOT);
        }
    }
}

/// Between shots a turret's head turns toward its aim (`Building::aim`,
/// what it would shoot), at `TURRET_TURN`; with nothing in reach it holds.
pub(super) fn turn_turrets(g: &Game<GpuWorld>, turrets: &mut Vec<BuildingLook>, dt: f32) {
    join_turrets(g, turrets);
    for (t, b) in turrets.iter_mut().zip(&g.buildings) {
        t.shot = (t.shot - dt).max(0.0);
        if let (Kind::Turret | Kind::Mortar, Some(aim)) = (b.kind, b.aim) {
            let want = (aim.1 - b.at.1).atan2(aim.0 - b.at.0);
            let mut d = want - t.facing;
            d = (d + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
            t.facing += d.clamp(-TURRET_TURN * dt, TURRET_TURN * dt);
        }
    }
}

/// The window's notes of a step it saw (after `shot_marks` and
/// `turret_fired`; building looks, 11-14): a siren that called starts its
/// call; a carrier whose load went into a drop-off flares that outpost or
/// the larder (before, the larder flared on any rise of the harvest, a
/// pump's draw or a load banked at an outpost included); a soldier the
/// slime bit takes the damage, and every `FLINCH_HP` of it flinches away
/// from the bite.
pub(super) fn step_notes(g: &Game<GpuWorld>, blooks: &mut Vec<BuildingLook>, looks: &mut Vec<Look>, banked: &mut f32) {
    join_turrets(g, blooks);
    join_looks(g, looks);
    for (n, b) in blooks.iter_mut().zip(&g.buildings) {
        if b.kind == Kind::Siren && b.working && b.on {
            n.call = SIREN_CALL;
        }
    }
    // Only banking empties a living soldier's pack, at the drop-off
    // nearest it (`Game::drop_offs`, as the game's own `drop_off`).
    let drops = g.drop_offs();
    for (l, s) in looks.iter_mut().zip(&g.soldiers) {
        if s.alive && l.carry > 0.0 && s.carry < l.carry * 0.5 {
            let d = |p: &(f32, f32)| (p.0 - s.x).hypot(p.1 - s.y);
            let home = drops.iter().min_by(|a, b| d(a).total_cmp(&d(b))).copied().unwrap_or(g.larder);
            match g.buildings.iter().position(|b| b.kind == Kind::Outpost && b.up() && b.at == home) {
                Some(k) => blooks[k].bank = BANKED,
                None => *banked = BANKED,
            }
        }
        if s.alive && s.touching > 0 && s.hp < l.hp {
            l.hurt += l.hp - s.hp;
            if l.hurt >= FLINCH_HP {
                l.hurt = (l.hurt - FLINCH_HP).min(FLINCH_HP);
                l.flinch = FLINCH;
                // From the slime it would shoot (`Soldier::aim`, the densest
                // mould in its sight), else from the mould's core.
                let from = s.aim.unwrap_or(g.core);
                l.bite = (from.1 - s.y).atan2(from.0 - s.x);
            }
        }
        (l.hp, l.carry) = (s.hp, s.carry);
    }
}

/// One step of `card=` before a frame's picture: what a player clicks on
/// the card or the world (the command card's films, 2026-11-15).
fn card_step(app: &mut App, step: &str) -> Result<(), String> {
    match step {
        "main" | "" => (app.build_page, app.tools_page) = (false, false),
        "build" => (app.build_page, app.tools_page) = (true, false),
        "tools" => (app.build_page, app.tools_page) = (false, true),
        // Esc: no squad commanded, so a click on a building selects it.
        "free" => app.let_go(),
        // A click on the newest building (as the squad mode's click does
        // with no squad commanded).
        "pick" => {
            app.let_go();
            app.building_sel = app.game.as_ref().and_then(|g| (0..g.buildings.len()).rev().find(|&b| g.buildings[b].alive));
        }
        "fire" => {
            app.hud_event(HudEvent::Command { id: FIRE })?;
        }
        "upgrade" | "demolish" | "switch" | "cancel" => {
            let id = match step {
                "upgrade" => UPGRADE,
                "demolish" => DEMOLISH,
                "switch" => SWITCH,
                _ => CANCEL_BUILD,
            };
            app.hud_event(HudEvent::Command { id })?;
        }
        _ => return Err(format!("card=: no step {step} (main, build, tools, free, pick, fire, upgrade, demolish, switch, cancel)")),
    }
    println!("  card: {step}");
    Ok(())
}

/// A building of `kind` standing whole at `at` (fine cells), paid for and
/// not raised: the `turret=` film keys. `render`: a level-2 turret.
fn stand(g: &mut Game<GpuWorld>, kind: Kind, at: (f32, f32), render: bool) -> Result<(), String> {
    let n = g.buildings.len();
    g.bank += g.build_price(kind, at);
    if !g.build(kind, at) {
        return Err(format!("{}={},{}: {}", kind.name(), at.0, at.1, g.build_blocked(kind, at).unwrap_or("blocked")));
    }
    // Raised whole, so a wall lays its cells in the world (setting `built`
    // by hand left a standing wall with no cells: the sapper film's first
    // run pressed on open ground).
    g.raise(n, 1.0);
    let b = &mut g.buildings[n];
    b.hp = b.max_hp;
    if render {
        b.level = 2;
    }
    Ok(())
}
