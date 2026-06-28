import type { Config } from 'tailwindcss'

export default {
  content: ['./src/**/*.{svelte,ts,html}'],
  theme: {
    extend: {
      fontFamily: {
        mono: ['JetBrains Mono', 'Fira Code', 'ui-monospace', 'monospace'],
      },
      colors: {
        surface: {
          0: '#0d1117',
          1: '#161b22',
          2: '#21262d',
          3: '#30363d',
        },
        accent: {
          blue: '#58a6ff',
          green: '#3fb950',
          yellow: '#d29922',
          red: '#f85149',
          purple: '#bc8cff',
          cyan: '#39c5cf',
        },
      },
    },
  },
  plugins: [],
} satisfies Config
