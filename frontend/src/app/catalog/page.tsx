import { getServerSession } from 'next-auth/next';
import { authOptions } from '@/lib/auth-options';
import Link from 'next/link';
import { Work } from '@/lib/types';
import WorkCard from '@/components/WorkCard';
import type { Session } from 'next-auth';

async function getWorks(
  session: Session | null,
  params: { work_type?: string; search?: string }
): Promise<Work[]> {
  if (!session?.accessToken) return [];

  const query = new URLSearchParams();
  if (params.work_type) query.set('work_type', params.work_type);
  if (params.search) query.set('search', params.search);

  const backendUrl = process.env.BACKEND_API_URL ?? 'http://localhost:8080';
  try {
    const res = await fetch(`${backendUrl}/api/works?${query}`, {
      headers: {
        Authorization: `Bearer ${session.accessToken}`,
      },
      cache: 'no-store',
    });
    if (!res.ok) return [];
    return res.json();
  } catch {
    return [];
  }
}

interface CatalogPageProps {
  searchParams: { work_type?: string; search?: string };
}

export default async function CatalogPage({ searchParams }: CatalogPageProps) {
  const session = await getServerSession(authOptions);
  const works = await getWorks(session, searchParams);

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Catalogo</h1>
        <Link
          href="/catalog/new"
          className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition"
        >
          + Aggiungi Opera
        </Link>
      </div>

      {/* Filtri */}
      <div className="flex gap-4 mb-6">
        {(['ALL', 'MANGA', 'ANIME', 'MOVIE', 'MUSIC'] as const).map((type) => (
          <Link
            key={type}
            href={type === 'ALL' ? '/catalog' : `/catalog?work_type=${type}`}
            className={`px-4 py-2 rounded-lg ${
              (searchParams.work_type ?? 'ALL') === type
                ? 'bg-blue-600 text-white'
                : 'bg-white text-gray-700 border hover:bg-gray-50'
            }`}
          >
            {type === 'ALL' ? 'Tutti' : type}
          </Link>
        ))}
      </div>

      {/* Grid opere */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {works.map((work) => (
          <WorkCard key={work.id} work={work} />
        ))}
        {works.length === 0 && (
          <p className="col-span-full text-center text-gray-500 py-8">
            Nessuna opera trovata. Inizia aggiungendo il tuo primo manga o anime!
          </p>
        )}
      </div>
    </div>
  );
}
