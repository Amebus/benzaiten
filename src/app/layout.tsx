import type { Metadata } from 'next';
import type { ReactNode } from 'react';

import '@/styles/globals/tailwind.css';
import '@/styles/globals/index.scss';

export const metadata: Metadata = {
	title: 'Benzaiten',
	description: 'Learning React with SSR on Next.js.',
};

interface RootLayoutProps {
	children: ReactNode;
}

export default function RootLayout({ children }: RootLayoutProps) {
	return (
		<html lang="it">
			<body>{children}</body>
		</html>
	);
}