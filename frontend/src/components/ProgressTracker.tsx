'use client';

import { useState } from 'react';
import { libraryApi } from '@/lib/api-client';

interface ProgressTrackerProps {
  itemId: string;
  currentEpisode: number;
  totalEpisodes?: number;
}

export default function ProgressTracker({ itemId, currentEpisode, totalEpisodes }: ProgressTrackerProps) {
  const [current, setCurrent] = useState(currentEpisode);
  const [saving, setSaving] = useState(false);

  const progress = totalEpisodes ? (current / totalEpisodes) * 100 : 0;

  const updateProgress = async (newValue: number) => {
    setSaving(true);
    try {
      await libraryApi.update(itemId, { current_episode: newValue });
      setCurrent(newValue);
    } catch (error) {
      console.error('Errore aggiornamento progresso:', error);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="mt-4">
      <div className="flex items-center gap-3">
        <span className="text-sm text-gray-600">Progresso:</span>
        <button
          onClick={() => updateProgress(Math.max(0, current - 1))}
          disabled={current <= 0 || saving}
          className="w-6 h-6 rounded-full border border-gray-300 hover:bg-gray-100 flex items-center justify-center disabled:opacity-40"
        >
          -
        </button>
        <span className="font-medium text-sm">
          {current}{totalEpisodes ? `/${totalEpisodes}` : ''}
        </span>
        <button
          onClick={() => updateProgress(current + 1)}
          disabled={saving || (totalEpisodes !== undefined && current >= totalEpisodes)}
          className="w-6 h-6 rounded-full border border-gray-300 hover:bg-gray-100 flex items-center justify-center disabled:opacity-40"
        >
          +
        </button>
        {saving && <span className="text-xs text-gray-400">Salvataggio...</span>}
      </div>

      {totalEpisodes && (
        <div className="mt-2">
          <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
            <div
              className="h-full bg-blue-500 rounded-full transition-all"
              style={{ width: `${Math.min(100, progress)}%` }}
            />
          </div>
          <p className="text-xs text-gray-500 mt-1">{Math.round(progress)}% completato</p>
        </div>
      )}
    </div>
  );
}
