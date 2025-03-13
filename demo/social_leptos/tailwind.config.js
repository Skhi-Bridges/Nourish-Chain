module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
    "../**/*.html",
    "../**/*.jsx",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          500: '#3AB795', // Spirulina green
        },
        'social-twitter': '#1DA1F2',
        'social-discord': '#5865F2',
        'social-telegram': '#0088cc',
        'social-youtube': '#FF0000',
        midnight: {
          700: '#1a1f2e',
          800: '#151a27',
          900: '#0f131e',
        },
        spirulina: {
          400: '#4ac9a8',
          500: '#3AB795',
          600: '#2d9277',
        },
        gold: {
          400: '#f3d19e',
          500: '#f1c478',
          600: '#e9b355',
          700: '#d1943a',
        },
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      },
    }
  },
  plugins: [],
}
