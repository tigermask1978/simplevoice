import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-shell'
import { getCurrentWindow } from '@tauri-apps/api/window'

const MODEL_URL = 'https://huggingface.co/ggerganov/whisper.cpp/tree/main'

export default function Onboarding() {
  const goToSettings = async () => {
    await invoke('open_settings')
    await getCurrentWindow().close()
  }

  return (
    <div style={{ padding: 32, display: 'flex', flexDirection: 'column', gap: 20, color: '#e0e0e0', height: '100%', boxSizing: 'border-box' }}>
      <h2 style={{ fontSize: 20, fontWeight: 600, margin: 0 }}>Welcome to SimpleVoice!</h2>

      <p style={{ margin: 0, lineHeight: 1.7, color: '#b0b0b0' }}>
        The app does not include any Whisper models.<br />
        Please download a Whisper GGUF model <span style={{ color: '#e0e0e0' }}>(base or small recommended)</span> first,
        then select the model file in Settings.
      </p>

      <div style={{ background: '#2a2a2a', border: '1px solid #444', borderRadius: 8, padding: '14px 16px', display: 'flex', flexDirection: 'column', gap: 6 }}>
        <span style={{ fontSize: 12, color: '#888' }}>Recommended download source</span>
        <a
          onClick={() => open(MODEL_URL)}
          style={{ color: '#3b82f6', cursor: 'pointer', fontSize: 13, wordBreak: 'break-all', textDecoration: 'none' }}
          onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
        >
          {MODEL_URL}
        </a>
      </div>

      <p style={{ margin: 0, lineHeight: 1.7, color: '#b0b0b0' }}>
        After downloading, select the model path in Settings and click <span style={{ color: '#e0e0e0' }}>Save</span> to start using the app.
      </p>

      <button onClick={goToSettings} style={btnStyle}>Open Settings</button>
    </div>
  )
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
  marginTop: 'auto',
}
