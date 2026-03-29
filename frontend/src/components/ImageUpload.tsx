'use client';

import { useState, useRef } from 'react';

interface ImageUploadProps {
  workId: string;
  kind?: 'COVER' | 'BANNER' | 'SCREENSHOT';
  onUploaded?: (imageUrl: string) => void;
}

export default function ImageUpload({ workId, kind = 'COVER', onUploaded }: ImageUploadProps) {
  const [uploading, setUploading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [preview, setPreview] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    // Preview locale
    const reader = new FileReader();
    reader.onload = (ev) => setPreview(ev.target?.result as string);
    reader.readAsDataURL(file);

    setUploading(true);
    setError(null);

    try {
      const formData = new FormData();
      formData.append('file', file);
      formData.append('kind', kind);

      const res = await fetch(`/api/proxy/works/${workId}/images`, {
        method: 'POST',
        body: formData,
      });

      if (!res.ok) {
        throw new Error('Errore durante il caricamento');
      }

      const data = await res.json();
      onUploaded?.(data.url ?? data.s3_key ?? '');
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : 'Errore durante il caricamento');
      setPreview(null);
    } finally {
      setUploading(false);
    }
  };

  return (
    <div>
      <div
        className="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center cursor-pointer hover:border-blue-400 transition"
        onClick={() => inputRef.current?.click()}
      >
        {preview ? (
          // eslint-disable-next-line @next/next/no-img-element
          <img src={preview} alt="Anteprima" className="max-h-48 mx-auto rounded" />
        ) : (
          <div>
            <p className="text-4xl mb-2">📁</p>
            <p className="text-sm text-gray-500">
              {uploading ? 'Caricamento...' : 'Clicca per caricare un\'immagine'}
            </p>
          </div>
        )}
      </div>

      <input
        ref={inputRef}
        type="file"
        accept="image/*"
        onChange={handleFileChange}
        className="hidden"
        disabled={uploading}
      />

      {error && (
        <p className="text-red-600 text-sm mt-2">{error}</p>
      )}
    </div>
  );
}
