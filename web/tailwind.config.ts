import type { Config } from 'tailwindcss';
import daisyui from 'daisyui';

export default {
	content: ['./src/routes/**/*.{svelte,js,ts}', './src/app.html'],
	theme: {
		extend: {
			fontFamily: {
				display: ['Poppins', 'sans-serif'],
				body: ['Poppins', 'sans-serif']
			}
		},
	},
	daisyui: {
		themes: ['light', 'dark', 'valentine', 'pastel', 'cupcake']
	},
	plugins: [daisyui]
} satisfies Config;
