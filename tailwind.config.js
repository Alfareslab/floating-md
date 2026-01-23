/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                'base': '#1C1D21',
                'surface': '#2E3440',
                'hover': '#3B4252',
                'border': '#3E4451',
                'accent': '#88C0D0',
                'text': {
                    primary: '#E5E9F0',
                    secondary: '#D8DEE9',
                    muted: '#4C566A',
                }
            },
            fontFamily: {
                'latin': ['Geist Sans', 'Inter', 'system-ui'],
                'arabic': ['IBM Plex Sans Arabic', 'Segoe UI'],
                'mono': ['Geist Mono', 'IBM Plex Mono', 'monospace'],
            },
        },
    },
    plugins: [],
}
