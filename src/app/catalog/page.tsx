type WorkType = 'MANGA' | 'ANIME' | 'MOVIE' | 'MUSIC';

interface Work {
	id: number;
	title: string;
	type: WorkType;
	year: number;
	description: string;
}

const WORKS: Work[] = [
	{ id: 1, title: 'Berserk', type: 'MANGA', year: 1989, description: 'Dark fantasy di Kentaro Miura.' },
	{ id: 2, title: 'Vinland Saga', type: 'MANGA', year: 2005, description: 'Epopea vichinga di Makoto Yukimura.' },
	{ id: 3, title: 'Neon Genesis Evangelion', type: 'ANIME', year: 1995, description: 'Mecha psicologico di Anno Hideaki.' },
	{ id: 4, title: 'Cowboy Bebop', type: 'ANIME', year: 1998, description: 'Space western jazz di Shinichirō Watanabe.' },
	{ id: 5, title: 'Akira', type: 'MOVIE', year: 1988, description: 'Capolavoro cyberpunk di Katsuhiro Otomo.' },
	{ id: 6, title: 'Ghost in the Shell', type: 'MOVIE', year: 1995, description: 'Filosofia e identità nel futuro.' },
	{ id: 7, title: 'Nujabes — Modal Soul', type: 'MUSIC', year: 2005, description: 'Lo-fi hip-hop che ha definito un genere.' },
	{ id: 8, title: 'Yoko Kanno — Cowboy Bebop OST', type: 'MUSIC', year: 1998, description: 'Colonna sonora jazz-fusion iconica.' },
];

const WORK_TYPES = ['ALL', 'MANGA', 'ANIME', 'MOVIE', 'MUSIC'] as const;

interface CatalogPageProps {
	searchParams: Promise<{ work_type?: string }>;
}

export default async function CatalogPage({ searchParams }: CatalogPageProps) {
	const { work_type } = await searchParams;
	const activeType = work_type ?? 'ALL';
	const works = activeType === 'ALL' ? WORKS : WORKS.filter((w) => w.type === activeType);

	return (
		<main className="page-shell stack">
			<div className="flex justify-between items-center">
				<h1 className="page-title">Catalogo</h1>
			</div>

			{/* Filtri */}
			<div className="flex flex-wrap gap-3">
				{WORK_TYPES.map((type) => {
					const href = type === 'ALL' ? '/catalog' : `/catalog?work_type=${type}`;
					const isActive = activeType === type;
					return (
						<a
							key={type}
							href={href}
							className={`px-4 py-2 rounded-full border text-sm font-medium transition ${
								isActive
									? 'bg-[var(--accent)] text-white border-[var(--accent)]'
									: 'border-[var(--border)] text-[var(--muted)] hover:border-[var(--accent)] hover:text-[var(--accent-strong)]'
							}`}
						>
							{type === 'ALL' ? 'Tutti' : type}
						</a>
					);
				})}
			</div>

			{/* Grid opere */}
			<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
				{works.map((work) => (
					<article key={work.id} className="panel stack">
						<div className="flex justify-between items-start">
							<span className="text-xs font-semibold uppercase tracking-widest text-[var(--muted)]">
								{work.type}
							</span>
							<span className="text-xs text-[var(--muted)]">{work.year}</span>
						</div>
						<h2 className="text-lg font-bold leading-tight">{work.title}</h2>
						<p className="text-sm text-[var(--muted)]">{work.description}</p>
					</article>
				))}
				{works.length === 0 && (
					<p className="col-span-full text-center text-[var(--muted)] py-8">
						Nessuna opera trovata.
					</p>
				)}
			</div>
		</main>
	);
}