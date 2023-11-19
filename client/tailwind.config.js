/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {
        fontFamily: {
            'oldtimey': ['OldTimey', 'serif']
        }
      },
    },
    plugins: [],
  }