import { List } from '@/basic/react/list/components';
import { TodoCard } from '@/components/react';
import { Navigation, Social } from '@/components/site';

export const dynamic = 'force-dynamic';

const items = [
	{ ciccia: 'Imparare meglio i componenti server di Next' },
	{ ciccia: 'Separare chiaramente componenti server e client' },
	{ ciccia: 'Continuare con React senza sintassi Astro in mezzo' },
];

export default function HomePage() {
	return (
		<main className="page-shell stack">
			<Navigation />
			<section className="panel stack">
				<p className="muted">Next.js App Router</p>
				<h1 className="page-title">SSR con React, senza il layer Astro.</h1>
				<p className="page-subtitle">
					Le route in `src/app` sono server-rendered di default. I componenti interattivi
					restano React puri e dichiarano `use client` solo dove serve davvero.
				</p>
				<div className="inline-actions">
					<Social platform="github" username="benzaiten" />
				</div>
			</section>

			<section className="panel stack">
				<h2>Lista di cose da provare</h2>
				<List items={items} itemKey="ciccia" itemComponent={TodoCard} label="Todo demo" />
			</section>
		</main>
	);
}