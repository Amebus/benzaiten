import { getServerSession } from 'next-auth/next';
import { authOptions } from '@/lib/auth-options';
import Link from 'next/link';
import { Work } from '@/lib/types';
import TagManager from '@/components/TagManager';
import ReleaseStatusForm from '@/components/ReleaseStatusForm';
import type { Session } from 'next-auth';

async function getWork(
  session: Session | null,
  id: string
): Promise<Work | null> {
  if (!session?.accessToken) return null;
  const backendUrl = process.env.BACKEND_API_URL ?? 'http://localhost:8080';
  try {
    const res = await fetch(`${backendUrl}/api/works/${id}`, {
      headers: { Authorization: `Bearer ${session.accessToken}` },
      cache: 'no-store',
    });
    if (!res.ok) return null;
    return res.json();
  } catch {
    return null;
  }
}

export default async function WorkDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  const session = await getServerSession(authOptions);
  const work = await getWork(session, id);

  if (!work) {
    return (
      <div className="text-center py-16">
        <h1 className="text-2xl font-bold text-gray-900 mb-4">Opera non trovata</h1>
        <Link href="/catalog" className="text-blue-600 hover:underline">
          Torna al catalogo
        </Link>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto">
      <div className="flex justify-between items-start mb-6">
        <div>
          <div className="flex items-center gap-3 mb-2">
            <span className="bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded">
              {work.work_type}
            </span>
            {work.year && (
              <span className="text-gray-500">{work.year}</span>
            )}
          </div>
          <h1 className="text-3xl font-bold text-gray-900">{work.title}</h1>
          {work.original_title && (
            <p className="text-gray-500 mt-1">{work.original_title}</p>
          )}
        </div>
        <Link
          href={`/catalog/${work.id}/edit`}
          className="bg-gray-100 text-gray-700 px-4 py-2 rounded-lg hover:bg-gray-200 transition"
        >
          Modifica
        </Link>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Colonna principale */}
        <div className="lg:col-span-2 space-y-6">
          {work.synopsis && (
            <div className="bg-white p-6 rounded-xl shadow">
              <h2 className="text-lg font-semibold mb-3">Sinossi</h2>
              <p className="text-gray-700 leading-relaxed">{work.synopsis}</p>
            </div>
          )}

          {/* Tag */}
          <div className="bg-white p-6 rounded-xl shadow">
            <h2 className="text-lg font-semibold mb-3">Tag</h2>
            <TagManager workId={work.id} currentTags={work.tags ?? []} />
          </div>

          {/* Stato rilascio */}
          <div className="bg-white p-6 rounded-xl shadow">
            <h2 className="text-lg font-semibold mb-3">Stato Rilascio</h2>
            <ReleaseStatusForm workId={work.id} statuses={work.release_statuses ?? []} />
          </div>
        </div>

        {/* Colonna laterale */}
        <div className="space-y-4">
          {/* Immagine copertina placeholder */}
          <div className="bg-white p-4 rounded-xl shadow">
            <div className="aspect-[2/3] bg-gray-100 rounded-lg flex items-center justify-center">
              <span className="text-gray-400 text-4xl">📚</span>
            </div>
          </div>

          {/* Autori */}
          {work.people && work.people.length > 0 && (
            <div className="bg-white p-4 rounded-xl shadow">
              <h3 className="font-semibold mb-3">Autori & Staff</h3>
              <ul className="space-y-2">
                {work.people.map((wp) => (
                  <li key={`${wp.person_id}-${wp.role}`} className="flex justify-between text-sm">
                    <span className="font-medium">{wp.person.name}</span>
                    <span className="text-gray-500">{wp.role}</span>
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
