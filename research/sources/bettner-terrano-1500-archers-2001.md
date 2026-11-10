---
id: bettner-terrano-1500-archers-2001
title: 1500 Archers on a 28.8: Network Programming in Age of Empires and Beyond
authors: Paul Bettner and Mark Terrano
year: 2001-03-22 (Gamasutra; GDC 2001)
url: https://www.gamedeveloper.com/programming/1500-archers-on-a-28-8-network-programming-in-age-of-empires-and-beyond
read: most of the article verbatim (2026-11-10); the end only via a summary, not used
copy: none
licence: publisher's copyright
used-in: ../../Reports/rts-research-2026-11-10.md
---

# 1500 Archers on a 28.8: Network Programming in Age of Empires and Beyond

Synchronous simulation: every machine runs the game, only commands are sent,
scheduled two communication turns ahead. Sending unit state capped the game at
about 250 moving units. Out-of-sync bugs were the hardest; world, pathfinding
and targeting were checksummed; random numbers seeded and synced. "Test with
modems (and, if you are lucky, modem simulators) as early as possible."
