import ReactDOM from 'react-dom/client'
import './index.css'
import { Routes } from '@generouted/react-router'
import { ThemeProvider } from './providers/theme-provider'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <ThemeProvider defaultTheme='dark' storageKey='vite-ui-theme'>
    <Routes />
  </ThemeProvider>
)
