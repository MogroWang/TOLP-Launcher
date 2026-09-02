/**
 * 生成应用图标：以游戏 UI 的“光点”（紫色光球）+ 播放键为母题。
 * 产物写入 src-tauri/icons/（icon.ico / icon.png 等），提交入库，CI 不重复生成。
 *
 * 用法：npm run icons
 */
import { Resvg } from '@resvg/resvg-js';
import png2icons from 'png2icons';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const outDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../src-tauri/icons');
fs.mkdirSync(outDir, { recursive: true });

const svg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
  <defs>
    <radialGradient id="orb" cx="0.36" cy="0.3" r="0.85">
      <stop offset="0" stop-color="#efd9fc"/>
      <stop offset="0.35" stop-color="#cb8df4"/>
      <stop offset="0.72" stop-color="#a855f7"/>
      <stop offset="1" stop-color="#7f35c9"/>
    </radialGradient>
    <radialGradient id="sheen" cx="0.36" cy="0.28" r="0.5">
      <stop offset="0" stop-color="#ffffff" stop-opacity="0.75"/>
      <stop offset="1" stop-color="#ffffff" stop-opacity="0"/>
    </radialGradient>
  </defs>
  <circle cx="256" cy="256" r="236" fill="url(#orb)"/>
  <circle cx="256" cy="256" r="236" fill="url(#sheen)"/>
  <path d="M192 196 Q192 180 207 188 L332 247 Q346 256 332 265 L207 324 Q192 332 192 316 Z" fill="#fff"/>
</svg>`;

function renderPng(size) {
  const resvg = new Resvg(svg, { fitTo: { mode: 'width', value: size } });
  return resvg.render().asPng();
}

const png256 = renderPng(256);
const ico = png2icons.createICO(png256, png2icons.BICUBIC2, 0, false, true);
fs.writeFileSync(path.join(outDir, 'icon.ico'), ico);

for (const [name, size] of [
  ['icon.png', 512],
  ['128x128.png', 128],
  ['32x32.png', 32],
]) {
  fs.writeFileSync(path.join(outDir, name), renderPng(size));
}

console.log('图标已生成 → src-tauri/icons/');
