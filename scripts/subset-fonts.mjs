/**
 * 从《光点之旅》项目的“未来圆系列字体”生成启动器用的 woff2 子集。
 * 只保留界面实际用到的字符，避免整包 20MB+ 的 CJK 字体进入安装包。
 *
 * 用法：node scripts/subset-fonts.mjs <未来圆字体目录>
 * 例如：node scripts/subset-fonts.mjs "E:/…/光点之旅 4.0+/未来圆系列字体"
 * 产物写入 src/assets/fonts/（提交入库；发布时请确认字体授权允许随应用分发）。
 */
import subsetFont from 'subset-font';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const srcDir = process.argv[2];
if (!srcDir) {
  console.error('用法：node scripts/subset-fonts.mjs <未来圆字体目录>');
  process.exit(1);
}

const outDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../src/assets/fonts');
fs.mkdirSync(outDir, { recursive: true });

/* 与启动器界面文案保持一致；ASCII 全量覆盖路径显示的需要 */
const uiText = `光点之旅启动器开始游戏设置全屏窗口化方式目录选择浏览找到未就绪正在退出失败缺少指定将导出的网页版本件含放入同文件夹或在此处其自动识别已请先重试项恢复默认关闭小打开启动游戏未尚选择件恢index.htmlGDevelopMOGROWANGSTUDIOTOLPLAUNCHERTOUROFLIGHTPOINT`;

const text = `${uiText}\u0020-\u007E：。，！？·（）《》—…、“”‘’`;

const faces = [
  { file: 'Regular.ttf', weight: 400, out: 'tolp-round-regular.woff2' },
  { file: 'Medium.ttf', weight: 500, out: 'tolp-round-medium.woff2' },
];

for (const face of faces) {
  const input = fs.readFileSync(path.join(srcDir, face.file));
  const subset = await subsetFont(input, text, { targetFormat: 'woff2' });
  fs.writeFileSync(path.join(outDir, face.out), subset);
  console.log(`${face.out}  ${(subset.length / 1024).toFixed(1)} KB`);
}
