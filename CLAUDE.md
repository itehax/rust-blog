# Project: itehax-website (Rust Blog)

## Stack
- **Framework:** Leptos 0.6.15 (SSR + hydration, nightly features)
- **Server:** Axum 0.7 + Tokio
- **Styling:** Tailwind CSS 3 + @tailwindcss/typography (prose "blog" variant)
- **Markdown:** pulldown-cmark + gray_matter (YAML frontmatter)
- **Syntax highlighting:** highlight.js (loaded client-side)
- **Math:** KaTeX (CDN)

## Commands
- `cargo leptos watch` — dev server
- `npm run watch` — Tailwind CSS watch mode (run alongside dev server)
- `npm run build` — compile Tailwind CSS once

## Key Files
- `src/components/post.rs` — Article layout, PostCard, TableOfContents
- `src/server_functions/posts.rs` — Markdown parsing, post loading, ToC generation
- `src/app.rs` — Route definitions
- `tailwind.config.js` — Tailwind config with custom "blog" prose variant
- `style/input.css` — Tailwind directives + base styles
- `style/output/output.css` — Compiled CSS (generated, do not hand-edit)
- `posts/blog/*.md` — Blog post markdown files
- `posts/writing/*.md` — Writing section posts

## Conventions
- Dark theme: bg `#0D1117`, surface `#161B22`, text `#E6EDF3`/`#C9D1D9`, muted `#8B949E`, borders `#30363D`, accent `#58A6FF`
- Article content rendered via `inner_html` from pulldown-cmark output
- Only h2 headings are collected for Table of Contents
- Post routes: `/blog/:post`, `/writing/:post`
- Fonts: 'Anonymous Pro', 'IBM Plex Sans'
