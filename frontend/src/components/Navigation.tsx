'use client';

import Link from 'next/link';
import { useSession, signIn, signOut } from 'next-auth/react';
import { usePathname } from 'next/navigation';

export default function Navigation() {
  const { data: session } = useSession();
  const pathname = usePathname();

  const isActive = (path: string) => pathname === path || pathname?.startsWith(path + '/');

  return (
    <nav className="bg-white border-b border-gray-200 shadow-sm">
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <Link href="/" className="font-bold text-xl text-gray-900">
            弁財天 Benzaiten
          </Link>

          {/* Navigazione principale */}
          {session && (
            <div className="flex items-center gap-6">
              <Link
                href="/catalog"
                className={`text-sm font-medium transition ${
                  isActive('/catalog')
                    ? 'text-blue-600'
                    : 'text-gray-600 hover:text-gray-900'
                }`}
              >
                Catalogo
              </Link>
              <Link
                href="/library"
                className={`text-sm font-medium transition ${
                  isActive('/library')
                    ? 'text-blue-600'
                    : 'text-gray-600 hover:text-gray-900'
                }`}
              >
                Libreria
              </Link>
            </div>
          )}

          {/* Auth */}
          <div>
            {session ? (
              <div className="flex items-center gap-3">
                <span className="text-sm text-gray-600">
                  {session.user?.name ?? session.user?.email ?? 'Utente'}
                </span>
                <button
                  onClick={() => signOut({ callbackUrl: '/' })}
                  className="text-sm text-gray-500 hover:text-red-600 transition"
                >
                  Esci
                </button>
              </div>
            ) : (
              <button
                onClick={() => signIn('keycloak')}
                className="bg-blue-600 text-white px-4 py-2 rounded-lg text-sm hover:bg-blue-700 transition"
              >
                Accedi
              </button>
            )}
          </div>
        </div>
      </div>
    </nav>
  );
}
