// Renders soldier.svg's layers to sprites.png, the atlas the view samples
// (src/world/view.rs, `SPRITES`): four 128 px tiles (body, helmet, flash,
// dead) in the top row, the same at 32 px in the row below (for small
// soldiers, so they don't shimmer). Run: node src/world/sprites/render.js
// from the repo root (needs Playwright's Chromium).
const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');
const dir = __dirname;
(async () => {
  const svg = fs.readFileSync(path.join(dir, 'soldier.svg'), 'utf8');
  const b = await chromium.launch();
  const p = await b.newPage({ viewport: { width: 512, height: 160 } });
  const only = (id) => svg.replace(/<g id="(\w+)">/g, (m, g) => (g === id ? m : `<g id="${g}" style="display:none">`));
  let html = '<html><body style="margin:0;background:transparent;position:relative;width:512px;height:160px">';
  ['body', 'helmet', 'flash', 'dead'].forEach((id, k) => {
    const one = only(id);
    html += `<div style="position:absolute;left:${k * 128}px;top:0;width:128px;height:128px">${one.replace('width="512" height="512"', 'width="128" height="128"')}</div>`;
    html += `<div style="position:absolute;left:${k * 32}px;top:128px;width:32px;height:32px">${one.replace('width="512" height="512"', 'width="32" height="32"')}</div>`;
  });
  html += '</body></html>';
  await p.setContent(html);
  await p.screenshot({ path: path.join(dir, 'sprites.png'), omitBackground: true, clip: { x: 0, y: 0, width: 512, height: 160 } });
  await b.close();
})();
