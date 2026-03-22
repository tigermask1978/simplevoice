import { invoke } from '@tauri-apps/api/core'
import { useState, useEffect } from 'react'

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

export default function Settings() {
  const [config, setConfig] = useState<Config>(DEFAULT_CONFIG)
  const [status, setStatus] = useState('')

  useEffect(() => {
    invoke<Config>('get_config').then(setConfig).catch(() => {})
  }, [])

  async function save() {
    try {
      await invoke('save_config', { config })
      setStatus('已保存')
      setTimeout(() => setStatus(''), 2000)
    } catch (e) {
      setStatus(`错误: ${e}`)
    }
  }

  return (
    <div style={{ padding: 24, display: 'flex', flexDirection: 'column', gap: 16 }}>
      <h2 style={{ fontSize: 18, fontWeight: 600 }}>SimpleVoice 设置</h2>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
        <span>全局热键</span>
        <input
          value={config.hotkey}
          onChange={e => setConfig(c => ({ ...c, hotkey: e.target.value }))}
          style={inputStyle}
        />
      </label>

      <label style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
        <span>模型路径</span>
        <input
          value={config.model_path}
          onChange={e => setConfig(c => ({ ...c, model_path: e.target.value }))}
          placeholder="models/ggml-small.bin"
          style={inputStyle}
        />
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
