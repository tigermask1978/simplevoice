import { invoke } from '@tauri-apps/api/core'
import { open, confirm } from '@tauri-apps/plugin-dialog'
import { useState, useEffect, useCallback } from 'react'
import { useI18n } from '../i18n'

type Config = {
  hotkey: string
  model_path: string
  language: 'zh' | 'en' | 'ja' | 'ko' | 'auto'
}

const DEFAULT_CONFIG: Config = {
  hotkey: 'Ctrl+Shift+Space',
  model_path: '',
  language: 'zh',
}

function eventToHotkey(e: KeyboardEvent): string | null {
  const modifiers: string[] = []
  if (e.ctrlKey) modifiers.push('Ctrl')
  if (e.shiftKey) modifiers.push('Shift')
  if (e.altKey) modifiers.push('Alt')
  if (e.metaKey) modifiers.push('Super')

  const ignored = new Set(['Control', 'Shift', 'Alt', 'Meta'])
  if (ignored.has(e.key)) return null

  const key = e.code.startsWith('Key')
    ? e.code.slice(3)
    : e.code.startsWith('Digit')
    ? e.code.slice(5)
    : e.key

  if (modifiers.length === 0) return null
  return [...modifiers, key].join('+')
}

export default function Settings() {
  const [config, setConfig] = useState<Config>(DEFAULT_CONFIG)
  const [savedConfig, setSavedConfig] = useState<Config>(DEFAULT_CONFIG)
  const [status, setStatus] = useState('')
  const [recording, setRecording] = useState(false)
  const t = useI18n(config.language)

  useEffect(() => {
    invoke<Config>('get_config').then(c => {
      setConfig(c)
      setSavedConfig(c)
      invoke('update_tray_lang', { lang: c.language }).catch(() => {})
    }).catch(() => {})
  }, [])

  useEffect(() => {
    invoke('update_tray_lang', { lang: config.language }).catch(() => {})
  }, [config.language])

  useEffect(() => {
    const onBlur = async () => {
      (document.activeElement as HTMLElement)?.blur()
      if (JSON.stringify(config) !== JSON.stringify(savedConfig)) {
        const yes = await confirm(t.unsavedPrompt, { title: 'SimpleVoice', kind: 'info' })
        if (yes) {
          await doSave(config)
        } else {
          setConfig(savedConfig)
        }
      }
    }
    window.addEventListener('blur', onBlur)
    return () => window.removeEventListener('blur', onBlur)
  }, [config, savedConfig, t])

  const handleHotkeyKeyDown = useCallback((e: React.KeyboardEvent) => {
    e.preventDefault()
    const hotkey = eventToHotkey(e.nativeEvent)
    if (hotkey) setConfig(c => ({ ...c, hotkey }))
  }, [])

  async function doSave(cfg: Config) {
    try {
      await invoke('save_config', { config: cfg })
      await invoke('register_hotkey', { hotkey: cfg.hotkey })
      setSavedConfig(cfg)
      setStatus('')
      const { getCurrentWindow } = await import('@tauri-apps/api/window')
      await getCurrentWindow().hide()
      await invoke('show_tray_notification')
    } catch (e) {
      const msg = String(e).includes('invalid_model') ? t.invalidModel : `${t.errorPrefix}${e}`
      setStatus(msg)
    }
  }

  const save = () => doSave(config)

  return (
    <div style={{ padding: 24, display: 'flex', flexDirection: 'column', gap: 16 }}>
      <h2 style={{ fontSize: 18, fontWeight: 600 }}>{t.title}</h2>

      <div style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
        <span>{t.hotkeyLabel}</span>
        <input
          readOnly
          value={config.hotkey}
          onFocus={() => setRecording(true)}
          onBlur={() => setRecording(false)}
          onKeyDown={handleHotkeyKeyDown}
          placeholder={t.hotkeyPlaceholder}
          style={{ ...inputStyle, cursor: 'pointer', outline: recording ? '2px solid #3b82f6' : undefined }}
        />
        <span style={{ fontSize: 12, color: '#f59e0b' }}>{t.hotkeyWarning}</span>
      </div>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
        <span>{t.modelLabel}</span>
        <div style={{ display: 'flex', gap: 8 }}>
          <input
            readOnly
            value={config.model_path}
            placeholder={t.modelPlaceholder}
            style={{ ...inputStyle, flex: 1, cursor: 'default' }}
          />
          <button
            onClick={async () => {
              const file = await open({ filters: [{ name: 'GGML Model', extensions: ['bin'] }] })
              if (typeof file === 'string') setConfig(c => ({ ...c, model_path: file }))
            }}
            style={btnStyle}
          >{t.browse}</button>
        </div>
      </label>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
        <span>{t.languageLabel}</span>
        <select
          value={config.language}
          onChange={e => setConfig(c => ({ ...c, language: e.target.value as Config['language'] }))}
          style={inputStyle}
        >
          <option value="zh">{t.langZh}</option>
          <option value="en">{t.langEn}</option>
          <option value="ja">{t.langJa}</option>
          <option value="ko">{t.langKo}</option>
          <option value="auto">{t.langAuto}</option>
        </select>
      </label>

      <button onClick={save} style={btnStyle}>{t.save}</button>
      {status && <span style={{ color: status.startsWith(t.errorPrefix) ? '#f87171' : '#4ade80' }}>{status}</span>}
    </div>
  )
}

const inputStyle: React.CSSProperties = {
  background: '#2a2a2a',
  border: '1px solid #444',
  borderRadius: 6,
  padding: '6px 10px',
  color: '#e0e0e0',
  fontSize: 14,
}

const btnStyle: React.CSSProperties = {
  background: '#3b82f6',
  color: '#fff',
  border: 'none',
  borderRadius: 6,
  padding: '8px 16px',
  cursor: 'pointer',
  fontSize: 14,
  alignSelf: 'flex-start',
}
