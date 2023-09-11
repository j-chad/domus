import type { Config } from 'tailwindcss';
import daisyui from 'daisyui';

export default {
	content: ['./src/routes/**/*.{svelte,js,ts}'],
	theme: {
		extend: {}
	},
	daisyui: {
		themes: ['light', 'dark', 'valentine']
	},
	plugins: [daisyui]
} satisfies Config;
