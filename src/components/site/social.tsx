interface SocialProps {
	platform: string;
	username: string;
}

export default function Social({ platform, username }: SocialProps) {
	const normalizedPlatform = platform.trim().toLowerCase();
	const href = `https://www.${normalizedPlatform}.com/${username}`;

	return (
		<a href={href} className="social-link" target="_blank" rel="noreferrer">
			{href}
		</a>
	);
}