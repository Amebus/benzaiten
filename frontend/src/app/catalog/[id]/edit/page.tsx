'use client';

import { useState, useEffect, use } from 'react';
import { useRouter } from 'next/navigation';
import { worksApi } from '@/lib/api-client';
import { Work, WorkType } from '@/lib/types';

interface EditWorkPageProps {
  params: Promise<{ id: string }>;
}

export default function EditWorkPage({ params }: EditWorkPageProps) {
  const { id } = use(params);
  const router = useRouter();
  const [loading, setLoading] = useState(false);
  const [fetching, setFetching] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [form, setForm] = useState({
    work_type: 'MANGA' as WorkType,
    title: '',
    original_title: '',
    synopsis: '',
    year: '',
  });

  useEffect(() => {
    worksApi.get(id)
      .then((data) => {
        const work = data as Work;
        setForm({
          work_type: work.work_type,
          title: work.title,
          original_title: work.original_title ?? '',
          synopsis: work.synopsis ?? '',
          year: work.year ? String(work.year) : '',
        });
      })
      .catch(() => setError('Impossibile caricare l\'opera'))
      .finally(() => setFetching(false));
  }, [id]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      await worksApi.update(id, {
        ...form,
        year: form.year ? parseInt(form.year) : undefined,
        original_title: form.original_title || undefined,
        synopsis: form.synopsis || undefined,
      });
      router.push(`/catalog/${id}`);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : 'Errore durante il salvataggio');
    } finally {
      setLoading(false);
    }
  };

  if (fetching) {
    return (
      <div className="flex justify-center py-16">
        <p className="text-gray-500">Caricamento...</p>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Modifica Opera</h1>

      <form onSubmit={handleSubmit} className="bg-white p-6 rounded-xl shadow space-y-4">
        {error && (
          <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
            {error}
          </div>
        )}

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Tipo</label>
          <select
            value={form.work_type}
            onChange={(e) => setForm({ ...form, work_type: e.target.value as WorkType })}
            className="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="MANGA">Manga</option>
            <option value="ANIME">Anime</option>
            <option value="MOVIE">Film</option>
            <option value="MUSIC">Musica</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Titolo *</label>
          <input
            type="text"
            required
            value={form.title}
            onChange={(e) => setForm({ ...form, title: e.target.value })}
            className="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="es. Naruto"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Titolo Originale</label>
          <input
            type="text"
            value={form.original_title}
            onChange={(e) => setForm({ ...form, original_title: e.target.value })}
            className="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="es. ナルト"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Anno</label>
          <input
            type="number"
            value={form.year}
            onChange={(e) => setForm({ ...form, year: e.target.value })}
            className="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="es. 1999"
            min="1900"
            max="2100"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Sinossi</label>
          <textarea
            value={form.synopsis}
            onChange={(e) => setForm({ ...form, synopsis: e.target.value })}
            className="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            rows={4}
            placeholder="Breve descrizione dell'opera..."
          />
        </div>

        <div className="flex gap-4">
          <button
            type="button"
            onClick={() => router.back()}
            className="flex-1 border border-gray-300 text-gray-700 py-2 rounded-lg hover:bg-gray-50 transition"
          >
            Annulla
          </button>
          <button
            type="submit"
            disabled={loading}
            className="flex-1 bg-blue-600 text-white py-2 rounded-lg hover:bg-blue-700 transition disabled:opacity-50"
          >
            {loading ? 'Salvataggio...' : 'Salva Modifiche'}
          </button>
        </div>
      </form>
    </div>
  );
}
