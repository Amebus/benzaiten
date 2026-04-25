import { Counter, TicTacToe } from '@/components/react';
import { Navigation } from '@/components/site';

export const dynamic = 'force-dynamic';

export default function TicTacToePage() {
	return (
		<main className="page-shell stack">
			<Navigation />
			<section className="panel stack">
				<p className="muted">Client components dentro una pagina SSR</p>
				<h1 className="page-title">Tic Tac Toe</h1>
				<p className="page-subtitle">
					La pagina viene resa sul server, mentre gioco e counter vengono idratati sul client.
				</p>
			</section>
			<section className="panel stack">
				<TicTacToe />
				<div>
					<Counter />
				</div>
			</section>
		</main>
	);
}