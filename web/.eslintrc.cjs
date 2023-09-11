module.exports = {
	root: true,
	env: {
		browser: true,
		es2017: true,
		node: true
	},
	overrides: [
		{
			files: ['*.svelte'],
			parser: 'svelte-eslint-parser',
			parserOptions: {
				parser: '@typescript-eslint/parser'
			}
		},
		{
			files: ['*.ts'],
			parser: '@typescript-eslint/parser'
		},
		{
			files: ['src/**/*'],
			plugins: ['@typescript-eslint', 'sonarjs'],
			parserOptions: {
				sourceType: 'module',
				ecmaVersion: 2020,
				extraFileExtensions: ['.svelte'],
				project: true,
				tsconfigRootDir: __dirname
			},
			extends: [
				'eslint:recommended',
				'plugin:@typescript-eslint/strict-type-checked',
				'plugin:@typescript-eslint/stylistic-type-checked',
				'plugin:sonarjs/recommended',
				'plugin:svelte/recommended',
				'prettier'
			]
		}
	]
};
