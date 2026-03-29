'use client';

import { useState } from 'react';
import { ReleaseStatus, MediaStatus } from '@/lib/types';

interface ReleaseStatusFormProps {
  workId: string;
  statuses: ReleaseStatus[];
}

const COUNTRIES = [
  { code: 'JP', name: 'Giappone 🇯🇵' },
  { code: 'IT', name: 'Italia 🇮🇹' },
  { code: 'US', name: 'USA 🇺🇸' },
];

const STATUS_LABELS: Record<MediaStatus, string> = {
  ONGOING: '🟢 In corso',
  COMPLETED: '✅ Completato',
  DROPPED: '❌ Interrotto',
  ANNOUNCED: '📢 Annunciato',
};

export default function ReleaseStatusForm({ workId, statuses }: ReleaseStatusFormProps) {
  const [currentStatuses, setCurrentStatuses] = useState(statuses);

  const getStatus = (countryCode: string) =>
    currentStatuses.find((s) => s.country_code === countryCode);

  const saveStatus = async (countryCode: string, status: MediaStatus) => {
    try {
      const res = await fetch(`/api/proxy/works/${workId}/release-status`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ country_code: countryCode, status }),
      });
      if (res.ok) {
        const updated = await res.json() as ReleaseStatus;
        setCurrentStatuses((prev) => {
          const existing = prev.findIndex((s) => s.country_code === countryCode);
          if (existing >= 0) {
            const newList = [...prev];
            newList[existing] = updated;
            return newList;
          }
          return [...prev, updated];
        });
      }
    } catch (error) {
      console.error('Errore salvataggio stato rilascio:', error);
    }
  };

  return (
    <div className="space-y-3">
      {COUNTRIES.map(({ code, name }) => {
        const status = getStatus(code);
        return (
          <div key={code} className="flex items-center justify-between">
            <span className="text-sm font-medium">{name}</span>
            <select
              value={status?.status ?? ''}
              onChange={(e) => e.target.value && saveStatus(code, e.target.value as MediaStatus)}
              className="border border-gray-300 rounded px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="">-- Non impostato --</option>
              {(Object.entries(STATUS_LABELS) as [MediaStatus, string][]).map(([value, label]) => (
                <option key={value} value={value}>{label}</option>
              ))}
            </select>
          </div>
        );
      })}
    </div>
  );
}
