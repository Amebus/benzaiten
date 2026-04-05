import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';
import react from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import astroPlugin from 'eslint-plugin-astro';
import { globalIgnores, defineConfig } from 'eslint/config';
import globals from 'globals';

export default defineConfig([
	// Global ignores
	globalIgnores(['node_modules/', 'dist/', '.astro/', '.git/']),

	// Base ESLint recommended rules
	eslint.configs.recommended,

	// TypeScript ESLint configs
	...tseslint.configs.recommended,

	// Register all plugins
	{
		name: 'plugins/setup',
		plugins: {
			'@typescript-eslint': tseslint.plugin,
			react,
			'react-hooks': reactHooks,
			astro: astroPlugin,
		},
	},

	// Global language options
	{
		name: 'language-options/setup',
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.es2020,
			},
		},
	},

	// TypeScript configuration
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

	// React configuration
	{
		name: 'react/setup',
		files: ['**/*.{jsx,tsx}'],
		settings: {
			react: {
				version: 'detect',
			},
		},
		rules: {
			'react/react-in-jsx-scope': 'off',
			'react/prop-types': 'off',
			'react-hooks/rules-of-hooks': 'error',
			'react-hooks/exhaustive-deps': 'warn',
		},
	},

	// Astro configuration
	{
		name: 'astro/setup',
		files: ['**/*.astro'],
		languageOptions: {
			parser: astroPlugin.parser,
			parserOptions: {
				parser: tseslint.parser,
				extraFileExtensions: ['.astro'],
				sourceType: 'module',
			},
		},
	},
]);
