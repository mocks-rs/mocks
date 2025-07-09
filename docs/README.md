# mocks Documentation

[![Built with Starlight](https://astro.badg.es/v2/built-with-starlight/tiny.svg)](https://starlight.astro.build)

This is the documentation site for the mocks project, built with [Starlight](https://starlight.astro.build/).

## Development

### Prerequisites

- Node.js 18+ 
- npm

### Setup

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Or use the docs-specific command
npm run docs:dev
```

The documentation site will be available at `http://localhost:4321`.

### Building

```bash
# Build for production
npm run build

# Or use the docs-specific command
npm run docs:build
```

The built site will be in the `dist/` directory.

### Preview

```bash
# Preview the built site
npm run preview

# Or use the docs-specific command
npm run docs:preview
```

## 🚀 Project Structure

```
docs/
├── src/
│   ├── content/
│   │   └── docs/           # Documentation pages
│   │       ├── index.mdx   # Homepage
│   │       ├── installation.md
│   │       ├── quick-start.md
│   │       ├── api-reference.md
│   │       ├── examples.md
│   │       └── troubleshooting.md
│   └── styles/
│       └── custom.css      # Custom styling
├── public/                 # Static assets
├── astro.config.mjs       # Astro configuration
└── package.json
```

## Adding Content

To add new documentation pages:

1. Create a new `.md` or `.mdx` file in `src/content/docs/`
2. Add frontmatter with `title` and `description`
3. Update the sidebar configuration in `astro.config.mjs`

Example:

```markdown
---
title: My New Page
description: Description of the page
---

# My New Page

Content goes here...
```

## 🧞 Commands

All commands are run from the root of the project, from a terminal:

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `npm install`             | Installs dependencies                            |
| `npm run dev`             | Starts local dev server at `localhost:4321`      |
| `npm run build`           | Build your production site to `./dist/`          |
| `npm run preview`         | Preview your build locally, before deploying     |
| `npm run docs:dev`        | Start documentation dev server                   |
| `npm run docs:build`      | Build documentation site                         |
| `npm run docs:preview`    | Preview documentation build                      |

## Deployment

### Static Site Hosting

The documentation can be deployed to any static site hosting service:

- **GitHub Pages**: Push to `gh-pages` branch
- **Netlify**: Connect to your repository and set build command to `npm run build`
- **Vercel**: Connect to your repository and set build command to `npm run build`
- **Cloudflare Pages**: Connect to your repository and set build command to `npm run build`

## 👀 Want to learn more?

Check out [Starlight’s docs](https://starlight.astro.build/), read [the Astro documentation](https://docs.astro.build), or jump into the [Astro Discord server](https://astro.build/chat).
