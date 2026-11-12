// Renders the sprite layers to sprites.png, the atlas the view samples
// (src/world/view.rs, `SPRITES`): 128 px tiles in the top row, the same at
// 32 px in the row below (for small figures, so they don't shimmer).
// Tiles 0-3 are soldier.svg's body, helmet, flash and dead; 4-11 are
// buildings.svg's turret, lure, pump and outpost, each body then its lamp
// (`*-lit`). Run: node src/world/sprites/render.js from the repo root (needs
// Playwright's Chromium).
const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');
const dir = __dirname;
const TILES = [
  ['soldier.svg', 'body'], ['soldier.svg', 'helmet'], ['soldier.svg', 'flash'], ['soldier.svg', 'dead'],
  ['buildings.svg', 'turret'], ['buildings.svg', 'turret-lit'], ['buildings.svg', 'lure'], ['buildings.svg', 'lure-lit'],
  ['buildings.svg', 'pump'], ['buildings.svg', 'pump-lit'], ['buildings.svg', 'outpost'], ['buildings.svg', 'outpost-lit'],
];
const W = TILES.length * 128;
(async () => {
  const b = await chromium.launch();
  const p = await b.newPage({ viewport: { width: W, height: 160 } });
  // Only the named layer shows (the sandbag group inside the outpost has no id).
  const only = (svg, id) => svg.replace(/<g id="([\w-]+)">/g, (m, g) => (g === id ? m : `<g id="${g}" style="display:none">`));
  let html = `<html><body style="margin:0;background:transparent;position:relative;width:${W}px;height:160px">`;
  TILES.forEach(([file, id], k) => {
    const one = only(fs.readFileSync(path.join(dir, file), 'utf8'), id);
    html += `<div style="position:absolute;left:${k * 128}px;top:0;width:128px;height:128px">${one.replace('width="512" height="512"', 'width="128" height="128"')}</div>`;
    html += `<div style="position:absolute;left:${k * 32}px;top:128px;width:32px;height:32px">${one.replace('width="512" height="512"', 'width="32" height="32"')}</div>`;
  });
  html += '</body></html>';
  await p.setContent(html);
  await p.screenshot({ path: path.join(dir, 'sprites.png'), omitBackground: true, clip: { x: 0, y: 0, width: W, height: 160 } });
  await b.close();
})();
