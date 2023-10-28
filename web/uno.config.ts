import { defineConfig, presetUno } from 'unocss';
import { presetDaisy } from 'unocss-preset-daisy';

export default defineConfig({
	presets: [
		presetUno(),
		presetDaisy({
			themes: ['light', 'dark', 'valentine'],
			logs: false
		})
	]
});
