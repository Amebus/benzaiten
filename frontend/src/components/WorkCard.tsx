import Link from 'next/link';
import { Work } from '@/lib/types';

interface WorkCardProps {
  work: Work;
}

const workTypeColors: Record<string, string> = {
  MANGA: 'bg-orange-100 text-orange-800',
  ANIME: 'bg-blue-100 text-blue-800',
  MOVIE: 'bg-purple-100 text-purple-800',
  MUSIC: 'bg-green-100 text-green-800',
};

export default function WorkCard({ work }: WorkCardProps) {
  const typeColor = workTypeColors[work.work_type] ?? 'bg-gray-100 text-gray-800';

  return (
    <Link href={`/catalog/${work.id}`}>
      <div className="bg-white rounded-xl shadow hover:shadow-md transition cursor-pointer overflow-hidden">
        {/* Copertina */}
        <div className="aspect-[2/3] bg-gradient-to-br from-gray-100 to-gray-200 flex items-center justify-center relative">
          <span className="text-5xl">
            {work.work_type === 'MANGA' ? '📖' :
             work.work_type === 'ANIME' ? '🎌' :
             work.work_type === 'MOVIE' ? '🎬' : '🎵'}
          </span>
          <div className="absolute top-2 left-2">
            <span className={`text-xs font-medium px-2 py-0.5 rounded-full ${typeColor}`}>
              {work.work_type}
            </span>
          </div>
        </div>

        {/* Informazioni */}
        <div className="p-4">
          <h3 className="font-semibold text-gray-900 truncate">{work.title}</h3>
          {work.original_title && (
            <p className="text-sm text-gray-500 truncate">{work.original_title}</p>
          )}
          {work.year && (
            <p className="text-xs text-gray-400 mt-1">{work.year}</p>
          )}

          {/* Tag */}
          {work.tags && work.tags.length > 0 && (
            <div className="flex flex-wrap gap-1 mt-2">
              {work.tags.slice(0, 3).map((tag) => (
                <span
                  key={tag.id}
                  className="text-xs px-2 py-0.5 rounded-full text-white"
                  style={{ backgroundColor: tag.color ?? '#6B7280' }}
                >
                  {tag.name}
                </span>
              ))}
              {work.tags.length > 3 && (
                <span className="text-xs text-gray-500">+{work.tags.length - 3}</span>
              )}
            </div>
          )}
        </div>
      </div>
    </Link>
  );
}
