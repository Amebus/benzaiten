import { getServerSession } from 'next-auth/next';
import { authOptions } from '@/lib/auth-options';
import Link from 'next/link';
import { LibraryItem } from '@/lib/types';
import ProgressTracker from '@/components/ProgressTracker';
import type { Session } from 'next-auth';

async function getLibrary(
  session: Session | null
): Promise<LibraryItem[]> {
  if (!session?.accessToken) return [];
  const backendUrl = process.env.BACKEND_API_URL ?? 'http://localhost:8080';
  try {
    const res = await fetch(`${backendUrl}/api/library`, {
      headers: { Authorization: `Bearer ${session.accessToken}` },
      cache: 'no-store',
    });
    if (!res.ok) return [];
    return res.json();
  } catch {
    return [];
  }
}

export default async function LibraryPage() {
  const session = await getServerSession(authOptions);
  const items = await getLibrary(session);

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">La Mia Libreria</h1>
        <Link
          href="/catalog"
          className="text-blue-600 hover:underline text-sm"
        >
          Vai al catalogo per aggiungere opere
        </Link>
      </div>

      {items.length === 0 ? (
        <div className="text-center py-16 bg-white rounded-xl shadow">
          <p className="text-gray-500 text-lg mb-4">La tua libreria è vuota</p>
          <Link
            href="/catalog"
            className="bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700 transition"
          >
            Sfoglia il Catalogo
          </Link>
        </div>
      ) : (
        <div className="space-y-4">
          {items.map((item) => (
            <div key={item.id} className="bg-white p-6 rounded-xl shadow">
              <div className="flex justify-between items-start">
                <div className="flex-1">
                  <h3 className="text-lg font-semibold">
                    <Link href={`/catalog/${item.work_id}`} className="hover:text-blue-600">
                      {item.work?.title ?? `Opera #${item.work_id}`}
                    </Link>
                  </h3>
                  {item.variant_notes && (
                    <p className="text-sm text-gray-500 mt-1">
                      Variant: {item.variant_notes}
                    </p>
                  )}
                </div>
                <div className="text-right">
                  {item.personal_rating && (
                    <span className="text-yellow-500 font-bold">
                      ⭐ {item.personal_rating}/10
                    </span>
                  )}
                  {item.purchase_price && (
                    <p className="text-sm text-gray-500">
                      €{item.purchase_price.toFixed(2)}
                    </p>
                  )}
                </div>
              </div>

              <ProgressTracker
                itemId={item.id}
                currentEpisode={item.current_episode ?? 0}
                totalEpisodes={item.total_episodes}
              />

              {item.owned_volumes && item.owned_volumes.length > 0 && (
                <div className="mt-3">
                  <p className="text-sm text-gray-600">
                    Volumi posseduti: {item.owned_volumes.join(', ')}
                  </p>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
