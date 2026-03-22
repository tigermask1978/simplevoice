import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'

type RecordingState = 'idle' | 'recording' | 'transcribing'

export function useRecording() {
  const [state, setState] = useState<RecordingState>('idle')
  const [lastText, setLastText] = useState('')
  const [error, setError] = useState('')

  useEffect(() => {
    const unlisten = listen<string>('recording-state', e => {
      setState(e.payload as RecordingState)
    })
    const unlistenText = listen<string>('transcription-result', e => {
      setLastText(e.payload)
    })
    const unlistenErr = listen<string>('transcription-error', e => {
      setError(e.payload)
      setTimeout(() => setError(''), 3000)
    })
    return () => {
      unlisten.then(f => f())
      unlistenText.then(f => f())
      unlistenErr.then(f => f())
    }
  }, [])

  async function startRecording() {
    await invoke('start_recording')
  }

  async function stopRecording() {
    await invoke('stop_recording')
  }

  return { state, lastText, error, startRecording, stopRecording }
}
