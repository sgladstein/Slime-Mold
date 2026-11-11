---
id: reginato-2024
title: Explorers and networkers (agent model of Physarum foraging)
authors: Reginato, Proverbio and Giordano
year: 2024
url: https://arxiv.org/abs/2412.19790
read: abstract and model description, by a research helper on 2026-11-11; the GAMA code on Zenodo was not reached
copy: none
licence: arXiv preprint; code licence not checked
used-in: ../../Reports/outside-rules-2026-11-11.md
---

# Reginato, Proverbio and Giordano 2024

An agent model of foraging in which each agent has one of two states, as the
helper read it
([survey](../../Reports/outside-rules-2026-11-11/code-survey.md)).

## Key facts for us

- Explorers steer toward *lower* concentration with +/- pi/8 jitter, and
  count the steps they take without food.
- After 100 + N(0, 10) steps an explorer becomes a networker: it follows
  the trail and deposits 1. Evaporation is 0.7.
- An agent dies at 250 steps, or when it has more than 18 neighbours. Agents
  duplicate on food.
- We built the explorer stage without food, death or duplication, as
  `BigParams::explore_steps` and `explore_deposit`.
