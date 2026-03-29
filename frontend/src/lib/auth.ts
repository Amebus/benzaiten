import { getServerSession } from 'next-auth/next';
import { authOptions } from '@/lib/auth-options';
import { Session } from 'next-auth';

export async function getSession(): Promise<Session | null> {
  return getServerSession(authOptions);
}

export async function requireAuth(): Promise<Session> {
  const session = await getSession();
  if (!session) {
    throw new Error('Authentication required');
  }
  return session;
}
