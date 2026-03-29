'use client';

import { signIn, useSession } from 'next-auth/react';
import { useRouter } from 'next/navigation';
import { useEffect } from 'react';

export default function LoginPage() {
  const { data: session } = useSession();
  const router = useRouter();

  useEffect(() => {
    if (session) {
      router.push('/catalog');
    }
  }, [session, router]);

  return (
    <div className="flex min-h-[60vh] items-center justify-center">
      <div className="bg-white p-8 rounded-xl shadow-lg max-w-md w-full text-center">
        <h1 className="text-2xl font-bold mb-2">弁財天 Benzaiten</h1>
        <p className="text-gray-600 mb-8">Accedi con il tuo account Keycloak</p>
        <button
          onClick={() => signIn('keycloak', { callbackUrl: '/catalog' })}
          className="w-full bg-blue-600 text-white py-3 rounded-lg hover:bg-blue-700 transition font-semibold"
        >
          Accedi con Keycloak
        </button>
      </div>
    </div>
  );
}
