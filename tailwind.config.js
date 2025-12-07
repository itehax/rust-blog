/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs", "./preline/*.js", "./posts/**/*.md"],
  theme: {
    screens: {
      sm: '480px',
      md: '768px',
      lg: '1020px',
      xl: '1440px',
    },
    fontFamily: {
      sans: ['Anonymous Pro','IBM Plex Sans', 'sans-serif'],

    },
    extend: {
      //https://play.tailwindcss.com/VCZwwz1e3R
      animation: {
        text: 'text 5s ease infinite',
      },
      keyframes: {
        text: {
          '0%, 100%': {
            'background-size': '200% 200%',
            'background-position': 'left center',
          },
          '50%': {
            'background-size': '200% 200%',
            'background-position': 'right center',
          },
        },
      },



      typography: ({ theme }) => ({
        blog: {
          css: {
            '--tw-prose-body': "#CED4DA",
            '--tw-prose-headings': "#F8F9FA",
            '--tw-prose-lead': "#F8F9FA",
            '--tw-prose-links': "#F8F9FA",
            '--tw-prose-bold': "#F8F9FA",
            '--tw-prose-counters': "#F8F9FA",
            '--tw-prose-bullets': "#F8F9FA",
            '--tw-prose-hr': "#CED4DA",
            '--tw-prose-quotes': "#F8F9FA",
            '--tw-prose-quote-borders': "#CED4DA",
            '--tw-prose-captions': "#CED4DA",
            '--tw-prose-code': "#F8F9FA",
            '--tw-prose-pre-bg': "#14213D",
            '--tw-prose-th-borders': "#CED4DA",
            '--tw-prose-td-borders': "#F8F9FA",
            // '--tw-prose-invert-body': theme('colors.pink[200]'),
            // '--tw-prose-invert-headings': theme('colors.white'),
            // '--tw-prose-invert-lead': "#CED4DA",
            // '--tw-prose-invert-links': theme('colors.white'),
            // '--tw-prose-invert-bold': theme('colors.white'),
            // '--tw-prose-invert-counters': theme('colors.pink[400]'),
            // '--tw-prose-invert-bullets': theme('colors.pink[600]'),
            // '--tw-prose-invert-hr': "#CED4DA",
            // '--tw-prose-invert-quotes': theme('colors.pink[100]'),
            // '--tw-prose-invert-quote-borders': "#CED4DA",
            // '--tw-prose-invert-captions': theme('colors.pink[400]'),
            // '--tw-prose-invert-code': theme('colors.white'),
            // '--tw-prose-invert-pre-code': "#CED4DA",
            // '--tw-prose-invert-pre-bg': 'rgb(0 0 0 / 50%)',
            // '--tw-prose-invert-th-borders': theme('colors.pink[600]'),
            // '--tw-prose-invert-td-borders': "#CED4DA",
          },
        },
      }),
    },
  },
  plugins: [require("preline/plugin"), require("@tailwindcss/typography")],
}
