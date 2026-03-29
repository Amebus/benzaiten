// Client-side API calls go through the BFF proxy
const API_BASE = '/api/proxy';

export class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
    this.name = 'ApiError';
  }
}

async function request<T>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const response = await fetch(`${API_BASE}/${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: 'Unknown error' }));
    throw new ApiError(response.status, error.error ?? 'Request failed');
  }

  return response.json();
}

// Works API
export const worksApi = {
  list: (params?: { work_type?: string; search?: string; page?: number }) => {
    const query = new URLSearchParams();
    if (params?.work_type) query.set('work_type', params.work_type);
    if (params?.search) query.set('search', params.search);
    if (params?.page) query.set('page', String(params.page));
    return request<unknown[]>(`works?${query}`);
  },
  get: (id: string) => request<unknown>(`works/${id}`),
  create: (data: unknown) => request<unknown>('works', {
    method: 'POST',
    body: JSON.stringify(data),
  }),
  update: (id: string, data: unknown) => request<unknown>(`works/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  }),
  delete: (id: string) => request<void>(`works/${id}`, { method: 'DELETE' }),
  addTag: (workId: string, tagId: string) => request<void>(`works/${workId}/tags`, {
    method: 'POST',
    body: JSON.stringify({ tag_id: tagId }),
  }),
  removeTag: (workId: string, tagId: string) => request<void>(`works/${workId}/tags/${tagId}`, {
    method: 'DELETE',
  }),
  addPerson: (workId: string, personId: string, role: string) =>
    request<void>(`works/${workId}/people`, {
      method: 'POST',
      body: JSON.stringify({ person_id: personId, role }),
    }),
};

// Tags API
export const tagsApi = {
  list: () => request<unknown[]>('tags'),
  get: (id: string) => request<unknown>(`tags/${id}`),
  create: (data: unknown) => request<unknown>('tags', {
    method: 'POST',
    body: JSON.stringify(data),
  }),
  update: (id: string, data: unknown) => request<unknown>(`tags/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  }),
  delete: (id: string) => request<void>(`tags/${id}`, { method: 'DELETE' }),
};

// Library API
export const libraryApi = {
  list: () => request<unknown[]>('library'),
  get: (id: string) => request<unknown>(`library/${id}`),
  create: (data: unknown) => request<unknown>('library', {
    method: 'POST',
    body: JSON.stringify(data),
  }),
  update: (id: string, data: unknown) => request<unknown>(`library/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  }),
  delete: (id: string) => request<void>(`library/${id}`, { method: 'DELETE' }),
};
