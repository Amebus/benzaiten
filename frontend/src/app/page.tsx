import Link from 'next/link';
import { getServerSession } from 'next-auth/next';
import { authOptions } from '@/lib/auth-options';

export default async function HomePage() {
  const session = await getServerSession(authOptions);

  return (
    <div className="text-center py-16">
      <h1 className="text-4xl font-bold text-gray-900 mb-4">
        弁財天 Benzaiten
      </h1>
      <p className="text-xl text-gray-600 mb-8">
        Il tuo catalogo personale di Manga, Anime e Media
      </p>
      {session ? (
        <div className="flex gap-4 justify-center">
          <Link
            href="/catalog"
            className="bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700 transition"
          >
            Vai al Catalogo
          </Link>
          <Link
            href="/library"
            className="bg-green-600 text-white px-6 py-3 rounded-lg hover:bg-green-700 transition"
          >
            La Mia Libreria
          </Link>
        </div>
      ) : (
        <Link
          href="/login"
          className="bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700 transition"
        >
          Accedi
        </Link>
      )}
    </div>
  );
}
