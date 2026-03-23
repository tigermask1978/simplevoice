const translations = {
  zh: {
    title: 'SimpleVoice 设置',
    hotkeyLabel: '全局热键',
    hotkeyPlaceholder: '点击后按下组合键',
    hotkeyWarning: '⚠️ 如果热键无响应，请检查是否有其他程序（如 PowerToys、输入法）占用了相同快捷键。',
    modelLabel: '模型路径',
    modelPlaceholder: '请选择模型文件',
    browse: '浏览',
    languageLabel: '语言',
    save: '保存',
    saved: '已保存',
    errorPrefix: '错误: ',
    unsavedPrompt: '有未保存的更改，是否立即保存？',
    langZh: '中文',
    langEn: 'English',
    langJa: '日本語',
    langAuto: '自动检测',
  },
  en: {
    title: 'SimpleVoice Settings',
    hotkeyLabel: 'Global Hotkey',
    hotkeyPlaceholder: 'Click then press a key combo',
    hotkeyWarning: '⚠️ If the hotkey is unresponsive, check whether another app (e.g. PowerToys, IME) has claimed the same shortcut.',
    modelLabel: 'Model Path',
    modelPlaceholder: 'Select a model file',
    browse: 'Browse',
    languageLabel: 'Language',
    save: 'Save',
    saved: 'Saved',
    errorPrefix: 'Error: ',
    unsavedPrompt: 'You have unsaved changes. Save now?',
    langZh: '中文',
    langEn: 'English',
    langJa: '日本語',
    langAuto: 'Auto-detect',
  },
  ja: {
    title: 'SimpleVoice 設定',
    hotkeyLabel: 'グローバルホットキー',
    hotkeyPlaceholder: 'クリックしてキーを押してください',
    hotkeyWarning: '⚠️ ホットキーが反応しない場合、他のアプリ（PowerToys、IMEなど）が同じショートカットを使用していないか確認してください。',
    modelLabel: 'モデルパス',
    modelPlaceholder: 'モデルファイルを選択',
    browse: '参照',
    languageLabel: '言語',
    save: '保存',
    saved: '保存しました',
    errorPrefix: 'エラー: ',
    unsavedPrompt: '未保存の変更があります。今すぐ保存しますか？',
    langZh: '中文',
    langEn: 'English',
    langJa: '日本語',
    langAuto: '自動検出',
  },
} as const

export function useI18n(lang: 'zh' | 'en' | 'ja' | 'auto') {
  const resolved = lang === 'auto'
    ? navigator.language.startsWith('zh') ? 'zh'
    : navigator.language.startsWith('ja') ? 'ja'
    : 'en'
    : lang
  return translations[resolved]
}
