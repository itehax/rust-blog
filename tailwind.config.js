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
      sans: ['Anonymous Pro', 'IBM Plex Sans', 'sans-serif'],

    },
    extend: {
      //https://play.tailwindcss.com/VCZwwz1e3R
      animation: {
        text: 'text 5s ease infinite',
        typewriter: "typing 4s steps(60, end) forwards, blink 1s step-end infinite"
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
            fontSize: '1.125rem',
            lineHeight: '1.8',
            p: {
              marginBottom: '1.25em',
            },
            h2: {
              marginTop: '2em',
              marginBottom: '0.75em',
            },
            h3: {
              marginTop: '1.6em',
              marginBottom: '0.5em',
            },
            li: {
              marginTop: '0.4em',
              marginBottom: '0.4em',
            },
            blockquote: {
              padding: '1em 1.5em',
              borderLeftWidth: '3px',
            },
            pre: {
              padding: '1.2em',
              borderRadius: '0.5rem',
              marginTop: '1.5em',
              marginBottom: '1.5em',
            },
            code: {
              fontSize: '0.9em',
            },
            a: {
              textDecorationColor: 'rgba(230, 237, 243, 0.3)',
              textUnderlineOffset: '3px',
            },
            'img, figure': {
              marginTop: '2em',
              marginBottom: '2em',
            },
            hr: {
              marginTop: '2.5em',
              marginBottom: '2.5em',
            },
            //Dark mode (Default)
            '--tw-prose-body': "#C9D1D9",
            '--tw-prose-headings': "#E6EDF3",
            '--tw-prose-lead': "#E6EDF3",
            '--tw-prose-links': "#E6EDF3",
            '--tw-prose-bold': "#E6EDF3",
            '--tw-prose-counters': "#8B949E",
            '--tw-prose-bullets': "#8B949E",
            '--tw-prose-hr': "#30363D",
            '--tw-prose-quotes': "#C9D1D9",
            '--tw-prose-quote-borders': "#30363D",
            '--tw-prose-captions': "#8B949E",
            '--tw-prose-code': "#E6EDF3",
            '--tw-prose-pre-code': "#C9D1D9",
            '--tw-prose-pre-bg': "#161B22",
            '--tw-prose-th-borders': "#30363D",
            '--tw-prose-td-borders': "#30363D",

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
