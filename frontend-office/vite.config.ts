import { fileURLToPath, URL } from "url";
import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react-swc'
import Generouted from '@generouted/react-router/plugin'

// https://vitejs.dev/config/
export default ({ mode }) => {
  process.env = {...process.env, ...loadEnv(mode, process.cwd())};

  return defineConfig({
    plugins: [react(), Generouted()],
    resolve: {
      alias: [
        { find: '@', replacement: fileURLToPath(new URL('./src', import.meta.url)) }
      ]
    },
    server: {
      proxy: {
        '/api': {
          target: 'http://localhost:2900',
          changeOrigin: true,
          secure: false,
        }
      }
    },
    base: "/office"
  })
}
