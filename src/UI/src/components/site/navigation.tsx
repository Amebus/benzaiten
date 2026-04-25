import Link from 'next/link';

const navigationItems = [
	{ href: '/', label: 'Home' },
	{ href: '/tictactoe', label: 'Tic Tac Toe' },
];

export default function Navigation() {
	return (
		<nav className="navigation" aria-label="Primary">
			{navigationItems.map((item) => (
				<Link key={item.href} href={item.href} className="nav-link">
					{item.label}
				</Link>
			))}
		</nav>
	);
}