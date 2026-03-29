'use client';

import { useState, useEffect } from 'react';
import { Tag } from '@/lib/types';
import { worksApi, tagsApi } from '@/lib/api-client';

interface TagManagerProps {
  workId: string;
  currentTags: Tag[];
}

export default function TagManager({ workId, currentTags }: TagManagerProps) {
  const [tags, setTags] = useState<Tag[]>(currentTags);
  const [allTags, setAllTags] = useState<Tag[]>([]);
  const [newTagName, setNewTagName] = useState('');
  const [loading, setLoading] = useState(false);
  const [showCreateForm, setShowCreateForm] = useState(false);

  useEffect(() => {
    tagsApi.list().then((all) => setAllTags(all as Tag[])).catch(console.error);
  }, []);

  const addTag = async (tagId: string) => {
    try {
      await worksApi.addTag(workId, tagId);
      const tag = allTags.find((t) => t.id === tagId);
      if (tag) setTags([...tags, tag]);
    } catch (error) {
      console.error('Errore aggiunta tag:', error);
    }
  };

  const removeTag = async (tagId: string) => {
    try {
      await worksApi.removeTag(workId, tagId);
      setTags(tags.filter((t) => t.id !== tagId));
    } catch (error) {
      console.error('Errore rimozione tag:', error);
    }
  };

  const createAndAddTag = async () => {
    if (!newTagName.trim()) return;
    setLoading(true);
    try {
      const slug = newTagName.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '');
      const newTag = await tagsApi.create({ name: newTagName, slug }) as Tag;
      setAllTags([...allTags, newTag]);
      await addTag(newTag.id);
      setNewTagName('');
      setShowCreateForm(false);
    } catch (error) {
      console.error('Errore creazione tag:', error);
    } finally {
      setLoading(false);
    }
  };

  const availableTags = allTags.filter((t) => !tags.some((ct) => ct.id === t.id));

  return (
    <div>
      {/* Tag correnti */}
      <div className="flex flex-wrap gap-2 mb-4">
        {tags.map((tag) => (
          <span
            key={tag.id}
            className="inline-flex items-center gap-1 px-3 py-1 rounded-full text-sm text-white"
            style={{ backgroundColor: tag.color ?? '#6B7280' }}
          >
            {tag.name}
            <button
              onClick={() => removeTag(tag.id)}
              className="ml-1 hover:opacity-75 text-white font-bold"
              aria-label={`Rimuovi tag ${tag.name}`}
            >
              ×
            </button>
          </span>
        ))}
        {tags.length === 0 && (
          <span className="text-gray-400 text-sm">Nessun tag assegnato</span>
        )}
      </div>

      {/* Aggiungi tag esistente */}
      {availableTags.length > 0 && (
        <div className="mb-3">
          <p className="text-sm text-gray-600 mb-2">Aggiungi tag esistente:</p>
          <div className="flex flex-wrap gap-2">
            {availableTags.map((tag) => (
              <button
                key={tag.id}
                onClick={() => addTag(tag.id)}
                className="text-sm px-3 py-1 border rounded-full hover:bg-gray-50 transition"
                style={{ borderColor: tag.color ?? '#6B7280', color: tag.color ?? '#6B7280' }}
              >
                + {tag.name}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Crea nuovo tag */}
      {showCreateForm ? (
        <div className="flex gap-2 mt-3">
          <input
            type="text"
            value={newTagName}
            onChange={(e) => setNewTagName(e.target.value)}
            placeholder="Nome nuovo tag (es. Isekai)"
            className="flex-1 border border-gray-300 rounded-lg px-3 py-1.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            onKeyDown={(e) => e.key === 'Enter' && createAndAddTag()}
          />
          <button
            onClick={createAndAddTag}
            disabled={loading}
            className="bg-blue-600 text-white px-3 py-1.5 rounded-lg text-sm hover:bg-blue-700 disabled:opacity-50"
          >
            Crea
          </button>
          <button
            onClick={() => setShowCreateForm(false)}
            className="border border-gray-300 text-gray-600 px-3 py-1.5 rounded-lg text-sm hover:bg-gray-50"
          >
            Annulla
          </button>
        </div>
      ) : (
        <button
          onClick={() => setShowCreateForm(true)}
          className="text-sm text-blue-600 hover:underline mt-2"
        >
          + Crea nuovo tag
        </button>
      )}
    </div>
  );
}
