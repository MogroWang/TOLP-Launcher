import { ref } from 'vue';
import type { Language } from './tauri';

/** 当前界面语言；由启动器设置（launcher-settings.json）持久化 */
export const locale = ref<Language>('zh-CN');

const messages: Record<Language, Record<string, string>> = {
  'zh-CN': {
    'game.name': '光点之旅',

    'nav.launch': '快速启动',
    'nav.versions': '版本管理',
    'nav.settings': '启动器设置',
    'nav.about': '关于',

    'titlebar.minimize': '最小化',
    'titlebar.close': '关闭',

    'mode.fullscreen': '全屏',
    'mode.windowed': '窗口化',
    'app.launchMode': '启动方式',
    'app.navLabel': '启动器导航',

    'ql.versionLabel': '启动版本',
    'ql.builtinDesc': '光点之旅 · 内部开发版本',
    'ql.dataVersionDesc': '来自数据文件夹的版本',
    'ql.custom': '自定义启动',
    'ql.customDesc': '使用指定游戏目录启动',
    'ql.launch': '启动游戏',
    'ql.launching': '正在启动…',
    'ql.cancel': '取消运行',
    'ql.notFound': '未找到游戏文件',
    'ql.running': '游戏正在运行',
    'ql.ready': '游戏已就绪',
    'ql.pickFolder': '选择文件夹',

    'status.noFolder': '尚未找到游戏文件夹',
    'status.missingIndex': '游戏目录中缺少 index.html',
    'status.desktopOnly': '请在桌面应用中运行',

    'vm.title': '版本管理',
    'vm.subtitle': '自动识别数据文件夹中的游戏版本。',
    'vm.rescan': '重新扫描',
    'vm.dataDir': '数据文件夹：',
    'vm.devBuild': '内部开发版本',
    'vm.from': '来自',
    'vm.srcDataFolder': '数据文件夹 versions/',
    'vm.srcCustom': '自定义位置',
    'vm.srcDefaultGame': '默认 game 文件夹',
    'vm.official': '官方版本',
    'vm.ready': '已就绪',
    'vm.unrecognized': '未识别',
    'vm.notReady': '未就绪',
    'vm.setCurrent': '设为当前',
    'vm.current': '当前版本',
    'vm.customLocation': '自定义位置…',
    'vm.clearCustom': '清除自定义位置',
    'vm.missingIndexHint': '所选目录中缺少 index.html',
    'vm.noVersionNum': '未识别版本号',
    'vm.scanning': '正在扫描版本文件夹…',
    'vm.emptyTip':
      '版本文件夹中尚未识别到任何版本：把游戏放入数据文件夹的 versions/<版本名>/（需包含 index.html），或点击上方「自定义位置」指定游戏所在文件夹。',
    'vm.autoTip':
      '版本会自动从数据文件夹的 versions/ 中识别；游戏存档保存在数据文件夹的 saves/ 中。更多历史版本与在线下载即将推出。',
    'dialog.pickVersionDir': '选择游戏所在文件夹（需包含 index.html）',

    'ls.title': '启动器设置',
    'ls.subtitle': '配置启动方式与游戏目录，更改会立即保存。',
    'ls.general': '通用',
    'ls.language': '语言',
    'ls.languageDesc': '切换后立即生效',
    'ls.launchGroup': '启动',
    'ls.launchModeDesc': '游戏窗口以全屏或设定尺寸的窗口打开',
    'ls.windowSize': '窗口大小',
    'ls.windowSizeDesc': '游戏窗口的显示尺寸，可选预设或自定义',
    'ls.sizeSmall': '小窗口',
    'ls.sizeRecommended': '推荐',
    'ls.sizeLarge': '大窗口',
    'ls.customSize': '自定义尺寸',
    'ls.customSizeDesc': '拖动滑块调节宽高',
    'ls.width': '宽度',
    'ls.height': '高度',
    'ls.gameDir': '游戏目录',
    'ls.gameDirDesc': '自定义启动时使用此目录，需包含 index.html',
    'ls.statusReady': '已就绪',
    'ls.statusMissing': '未找到游戏',
    'ls.dirDefault': '默认使用启动器同目录的 game 文件夹',
    'ls.choose': '选择…',
    'ls.resetDir': '恢复默认',
    'ls.hint1': '选择内置版本时优先使用数据文件夹 ',
    'ls.hint2':
      '，未识别到时可在版本管理中指定自定义位置；把 GDevelop 导出的网页版游戏（含 index.html 的文件夹）放入同目录 ',
    'ls.hint3': ' 文件夹，即可作为默认游戏。游戏存档保存在数据文件夹的 ',
    'ls.hint4': ' 子目录中。',
    'dialog.pickGameDir': '选择游戏目录（需包含 index.html）',

    'ab.title': '关于',
    'ab.subtitle': 'TOLP Launcher 与游戏光点之旅的信息。',
    'ab.info': '信息',
    'ab.game': '游戏',
    'ab.developer': '开发者',
    'ab.dataFolder': '数据文件夹',
    'ab.footnote':
      '为光点之旅打造的桌面启动器，基于 Tauri 构建 · 便携版设计，设置与数据保存于启动器同目录，不写入系统注册表。',
  },
  en: {
    'game.name': 'Tour of Light Point',

    'nav.launch': 'Quick Launch',
    'nav.versions': 'Versions',
    'nav.settings': 'Settings',
    'nav.about': 'About',

    'titlebar.minimize': 'Minimize',
    'titlebar.close': 'Close',

    'mode.fullscreen': 'Fullscreen',
    'mode.windowed': 'Windowed',
    'app.launchMode': 'Launch mode',
    'app.navLabel': 'Launcher navigation',

    'ql.versionLabel': 'Game version',
    'ql.builtinDesc': 'Tour of Light Point · internal dev build',
    'ql.dataVersionDesc': 'Version from the data folder',
    'ql.custom': 'Custom launch',
    'ql.customDesc': 'Launch from a chosen game folder',
    'ql.launch': 'Launch Game',
    'ql.launching': 'Launching…',
    'ql.cancel': 'Cancel Run',
    'ql.notFound': 'Game files not found',
    'ql.running': 'Game is running',
    'ql.ready': 'Game ready',
    'ql.pickFolder': 'Choose folder',

    'status.noFolder': 'No game folder found yet',
    'status.missingIndex': 'index.html is missing from the game folder',
    'status.desktopOnly': 'Please run in the desktop app',

    'vm.title': 'Versions',
    'vm.subtitle': 'Game versions in the data folder are detected automatically.',
    'vm.rescan': 'Rescan',
    'vm.dataDir': 'Data folder: ',
    'vm.devBuild': 'internal dev build',
    'vm.from': 'from ',
    'vm.srcDataFolder': 'data folder versions/',
    'vm.srcCustom': 'custom location',
    'vm.srcDefaultGame': 'default game folder',
    'vm.official': 'Official',
    'vm.ready': 'Ready',
    'vm.unrecognized': 'Not detected',
    'vm.notReady': 'Not ready',
    'vm.setCurrent': 'Set current',
    'vm.current': 'Current',
    'vm.customLocation': 'Custom location…',
    'vm.clearCustom': 'Clear custom location',
    'vm.missingIndexHint': 'index.html is missing in the selected folder',
    'vm.noVersionNum': 'Unknown version',
    'vm.scanning': 'Scanning the versions folder…',
    'vm.emptyTip':
      'No versions detected in the versions folder yet: put a game into versions/<name>/ of the data folder (must contain index.html), or use "Custom location…" above to point at the game folder.',
    'vm.autoTip':
      'Versions are detected automatically from the data folder\'s versions/; game saves are kept in the data folder\'s saves/. More historical versions and online downloads are coming soon.',
    'dialog.pickVersionDir': 'Choose the game folder (must contain index.html)',

    'ls.title': 'Settings',
    'ls.subtitle': 'Configure how the game starts. Changes are saved instantly.',
    'ls.general': 'General',
    'ls.language': 'Language',
    'ls.languageDesc': 'Applies instantly',
    'ls.launchGroup': 'Launch',
    'ls.launchModeDesc': 'The game opens fullscreen or in a window of the chosen size',
    'ls.windowSize': 'Window size',
    'ls.windowSizeDesc': 'Display size of the game window — pick a preset or customize',
    'ls.sizeSmall': 'Small',
    'ls.sizeRecommended': 'Recommended',
    'ls.sizeLarge': 'Large',
    'ls.customSize': 'Custom size',
    'ls.customSizeDesc': 'Drag the sliders to adjust',
    'ls.width': 'Width',
    'ls.height': 'Height',
    'ls.gameDir': 'Game folder',
    'ls.gameDirDesc': 'Used for custom launches; must contain index.html',
    'ls.statusReady': 'Ready',
    'ls.statusMissing': 'Not found',
    'ls.dirDefault': 'Defaults to the "game" folder next to the launcher',
    'ls.choose': 'Choose…',
    'ls.resetDir': 'Reset',
    'ls.hint1': 'Built-in versions resolve to the data folder ',
    'ls.hint2':
      ' first; if not detected, pick a custom location in Versions. Drop a GDevelop web export (a folder containing index.html) into the sibling ',
    'ls.hint3': ' folder to use it as the default game. Game saves are kept in the ',
    'ls.hint4': ' subfolder of the data folder.',
    'dialog.pickGameDir': 'Choose the game folder (must contain index.html)',

    'ab.title': 'About',
    'ab.subtitle': 'About TOLP Launcher and Tour of Light Point.',
    'ab.info': 'Info',
    'ab.game': 'Game',
    'ab.developer': 'Developer',
    'ab.dataFolder': 'Data folder',
    'ab.footnote':
      'A desktop launcher for Tour of Light Point, built with Tauri. Portable by design: settings and data live next to the launcher — nothing is written to the system registry.',
  },
};

/** 取当前语言的文案；缺 key 时回落中文，再缺则原样返回 key。
 *  语言值可能来自设置文件（可能被手改为非法值），用可选链兜底避免白屏 */
export function t(key: string): string {
  return messages[locale.value]?.[key] ?? messages['zh-CN'][key] ?? key;
}

/**
 * 翻译后端返回的动态文案（GameStatus.reason 等）。
 * 后端固定输出中文，这里按原文映射到当前语言；未收录的（如错误详情）原样返回。
 */
const messageMap: Record<string, string> = {
  '尚未找到游戏文件夹': 'status.noFolder',
  '游戏目录中缺少 index.html': 'status.missingIndex',
  '请在桌面应用中运行': 'status.desktopOnly',
};

export function translateMessage(text: string): string {
  const key = messageMap[text];
  return key ? t(key) : text;
}

/** 切换界面语言并同步 html lang 属性；非法语言值（如设置文件被手改）回落简体中文 */
export function setLocale(lang: Language): void {
  locale.value = lang in messages ? lang : 'zh-CN';
  document.documentElement.lang = locale.value;
}
