import type { Config } from 'tailwindcss';
import daisyui from 'daisyui';

export default {
	content: ['./src/routes/**/*.{svelte,js,ts}', './src/app.html'],
	theme: {
		extend: {}
	},
	daisyui: {
		themes: ['light', 'dark', 'valentine']
	},
	plugins: [daisyui]
} satisfies Config;
