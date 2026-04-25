import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';
import reactHooks from 'eslint-plugin-react-hooks';
import nextPlugin from '@next/eslint-plugin-next';
import { globalIgnores, defineConfig } from 'eslint/config';
import globals from 'globals';

export default defineConfig([
	globalIgnores(['node_modules/', '.next/', '.astro/', 'out/', 'dist/', '.git/']),

	eslint.configs.recommended,
	...tseslint.configs.recommended,

	{
		name: 'plugins/setup',
		plugins: {
			'@typescript-eslint': tseslint.plugin,
			'react-hooks': reactHooks,
			'@next/next': nextPlugin,
		},
	},

	{
		name: 'language-options/setup',
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node,
				...globals.es2024,
			},
		},
	},

	{
		name: 'typescript/setup',
		files: ['**/*.{ts,tsx}'],
		languageOptions: {
			parser: tseslint.parser,
			parserOptions: {
				ecmaVersion: 'latest',
				sourceType: 'module',
				ecmaFeatures: {
					jsx: true,
				},
			},
		},
		rules: {
			'@typescript-eslint/no-unused-vars': [
				'warn',
				{
					argsIgnorePattern: '^_',
					varsIgnorePattern: '^_',
				},
			],
			'@typescript-eslint/no-explicit-any': 'off',
		},
	},

	{
		name: 'react-next/setup',
		files: ['**/*.{jsx,tsx}'],
		rules: {
			...reactHooks.configs.recommended.rules,
			...nextPlugin.configs.recommended.rules,
			...nextPlugin.configs['core-web-vitals'].rules,
		},
	},
	{
		name: 'next-generated/setup',
		files: ['next-env.d.ts'],
		rules: {
			'@typescript-eslint/triple-slash-reference': 'off',
		},
	},
]);
