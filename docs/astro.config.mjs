// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://mocks-rs.github.io',
	base: '/mocks',
	server: {
		host: '0.0.0.0',
		port: 4321
	},
	integrations: [
		starlight({
			title: 'mocks',
			description: 'Get a mock REST APIs with zero coding within seconds.',
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/mocks-rs/mocks' },
			],
			sidebar: [
				{
					label: 'Getting Started',
					items: [
						{ label: 'Introduction', slug: 'index' },
						{ label: 'Installation', slug: 'installation' },
						{ label: 'Quick Start', slug: 'quick-start' },
					],
				},
				{
					label: 'Reference',
					items: [
						{ label: 'API Reference', slug: 'api-reference' },
						{ label: 'Examples', slug: 'examples' },
					],
				},
				{
					label: 'Help',
					items: [
						{ label: 'Troubleshooting', slug: 'troubleshooting' },
						{ label: 'Uninstallation', slug: 'uninstallation' },
					],
				},
			],
			customCss: ['./src/styles/custom.css'],
		}),
	],
});
