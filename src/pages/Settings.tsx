import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useState, useEffect, useCallback } from 'react'

type Config = {
  hotkey: string
  model_path: string
  language: 'zh' | 'en' | 'auto'
}

const DEFAULT_CONFIG: Config = {
  hotkey: 'Ctrl+Shift+Space',
  model_path: '',
  language: 'zh',
}

// Convert a KeyboardEvent into a hotkey string like "Ctrl+Shift+A"
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
  const [status, setStatus] = useState('')
  const [recording, setRecording] = useState(false)

  useEffect(() => {
    invoke<Config>('get_config').then(setConfig).catch(() => {})
  }, [])

  useEffect(() => {
    const onBlur = () => (document.activeElement as HTMLElement)?.blur()
    window.addEventListener('blur', onBlur)
    return () => window.removeEventListener('blur', onBlur)
  }, [])

  const handleHotkeyKeyDown = useCallback((e: React.KeyboardEvent) => {
    e.preventDefault()
    const hotkey = eventToHotkey(e.nativeEvent)
    if (hotkey) setConfig(c => ({ ...c, hotkey }))
  }, [])

  async function save() {
    try {
      await invoke('save_config', { config })
      await invoke('register_hotkey', { hotkey: config.hotkey })
      setStatus('已保存')
      setTimeout(() => setStatus(''), 2000)
    } catch (e) {
      setStatus(`错误: ${e}`)
    }
  }

  return (
    <div style={{ padding: 24, display: 'flex', flexDirection: 'column', gap: 16 }}>
      <h2 style={{ fontSize: 18, fontWeight: 600 }}>SimpleVoice 设置</h2>

      <div style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
        <span>全局热键</span>
        <input
          readOnly
          value={recording ? '请按下组合键…' : config.hotkey}
          onFocus={() => setRecording(true)}
          onBlur={() => setRecording(false)}
          onKeyDown={handleHotkeyKeyDown}
          placeholder="点击后按下组合键"
          style={{ ...inputStyle, cursor: 'pointer', outline: recording ? '2px solid #3b82f6' : undefined }}
        />
        <span style={{ fontSize: 12, color: '#f59e0b' }}>
          ⚠️ 如果热键无响应，请检查是否有其他程序（如 PowerToys、输入法）占用了相同快捷键。
        </span>
      </div>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
        <span>模型路径</span>
        <div style={{ display: 'flex', gap: 8 }}>
          <input
            readOnly
            value={config.model_path}
            placeholder="请选择模型文件"
            style={{ ...inputStyle, flex: 1, cursor: 'default' }}
          />
          <button
            onClick={async () => {
              const file = await open({ filters: [{ name: 'GGML Model', extensions: ['bin'] }] })
              if (typeof file === 'string') setConfig(c => ({ ...c, model_path: file }))
            }}
            style={btnStyle}
          >浏览</button>
        </div>
      </label>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
        <span>语言</span>
        <select
          value={config.language}
          onChange={e => setConfig(c => ({ ...c, language: e.target.value as Config['language'] }))}
          style={inputStyle}
        >
          <option value="zh">中文</option>
          <option value="en">English</option>
          <option value="auto">自动检测</option>
        </select>
      </label>

      <button onClick={save} style={btnStyle}>保存</button>
      {status && <span style={{ color: status.startsWith('错误') ? '#f87171' : '#4ade80' }}>{status}</span>}
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
