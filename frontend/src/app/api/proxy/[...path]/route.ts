import { NextRequest, NextResponse } from 'next/server';
import { getServerSession } from 'next-auth/next';
import { authOptions } from '@/lib/auth-options';

const BACKEND_API_URL = process.env.BACKEND_API_URL ?? 'http://localhost:8080';

async function proxyRequest(
  req: NextRequest,
  params: { path: string[] },
  method: string
): Promise<NextResponse> {
  const session = await getServerSession(authOptions);

  const path = params.path.join('/');
  const searchParams = req.nextUrl.searchParams.toString();
  const url = `${BACKEND_API_URL}/api/${path}${searchParams ? `?${searchParams}` : ''}`;

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  };

  if (session?.accessToken) {
    headers['Authorization'] = `Bearer ${session.accessToken}`;
  }

  const fetchOptions: RequestInit = {
    method,
    headers,
  };

  if (method !== 'GET' && method !== 'HEAD') {
    const contentType = req.headers.get('content-type') ?? '';
    if (contentType.includes('multipart/form-data')) {
      const formData = await req.formData();
      delete headers['Content-Type'];
      fetchOptions.body = formData as unknown as BodyInit;
    } else {
      const body = await req.text();
      if (body) {
        fetchOptions.body = body;
      }
    }
  }

  try {
    const response = await fetch(url, fetchOptions);
    const data = await response.json().catch(() => ({}));
    return NextResponse.json(data, { status: response.status });
  } catch (error) {
    console.error('Proxy error:', error);
    return NextResponse.json(
      { error: 'Backend service unavailable' },
      { status: 503 }
    );
  }
}

type RouteParams = { params: { path: string[] } };

export async function GET(req: NextRequest, { params }: RouteParams) {
  return proxyRequest(req, params, 'GET');
}

export async function POST(req: NextRequest, { params }: RouteParams) {
  return proxyRequest(req, params, 'POST');
}

export async function PUT(req: NextRequest, { params }: RouteParams) {
  return proxyRequest(req, params, 'PUT');
}

export async function DELETE(req: NextRequest, { params }: RouteParams) {
  return proxyRequest(req, params, 'DELETE');
}

export async function PATCH(req: NextRequest, { params }: RouteParams) {
  return proxyRequest(req, params, 'PATCH');
}
