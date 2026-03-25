import { getCurrentWindow } from '@tauri-apps/api/window'
import Onboarding from './pages/Onboarding'
import Settings from './pages/Settings'

export default function App() {
  return getCurrentWindow().label === 'onboarding' ? <Onboarding /> : <Settings />
}
