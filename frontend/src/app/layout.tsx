import type { Metadata } from 'next';
import './globals.css';
import { getServerSession } from 'next-auth/next';
import { authOptions } from '@/lib/auth-options';
import Navigation from '@/components/Navigation';
import Providers from './providers';

export const metadata: Metadata = {
  title: 'Benzaiten - Catalogo Manga & Anime',
  description: 'Gestisci la tua collezione di manga, anime e media',
};

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await getServerSession(authOptions);

  return (
    <html lang="it">
      <body className="font-sans antialiased">
        <Providers session={session}>
          <div className="min-h-screen bg-gray-50">
            <Navigation />
            <main className="container mx-auto px-4 py-8">
              {children}
            </main>
          </div>
        </Providers>
      </body>
    </html>
  );
}
