export type WorkType = 'MANGA' | 'ANIME' | 'MOVIE' | 'MUSIC';
export type MediaStatus = 'ONGOING' | 'COMPLETED' | 'DROPPED' | 'ANNOUNCED';

export interface Work {
  id: string;
  work_type: WorkType;
  title: string;
  original_title?: string;
  synopsis?: string;
  year?: number;
  metadata?: Record<string, unknown>;
  created_at: string;
  updated_at: string;
  tags?: Tag[];
  people?: WorkPerson[];
  images?: Image[];
  release_statuses?: ReleaseStatus[];
}

export interface Tag {
  id: string;
  name: string;
  slug: string;
  description?: string;
  color?: string;
  created_at: string;
}

export interface Person {
  id: string;
  name: string;
  original_name?: string;
  country_code?: string;
  birth_date?: string;
  created_at: string;
}

export interface WorkPerson {
  work_id: string;
  person_id: string;
  person: Person;
  role: string;
}

export interface Image {
  id: string;
  work_id: string;
  s3_key: string;
  kind: 'COVER' | 'BANNER' | 'SCREENSHOT';
  display_order: number;
  width?: number;
  height?: number;
  created_at: string;
}

export interface ReleaseStatus {
  id: string;
  work_id: string;
  country_code: string;
  status: MediaStatus;
  started_at?: string;
  completed_at?: string;
}

export interface LibraryItem {
  id: string;
  user_id: string;
  work_id: string;
  work?: Work;
  owned_volumes?: string[];
  current_episode?: number;
  total_episodes?: number;
  purchase_price?: number;
  variant_notes?: string;
  personal_rating?: number;
  notes?: string;
  created_at: string;
  updated_at: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
}
