create table if not exists public.tags_tb
(
    id_tag      uuid                     not null
        constraint pk_tags_tb
            primary key,
    name        varchar(100)             not null
        constraint unq_tags_tb__name
            unique,
    slug        varchar(100)             not null
        constraint unq_tags_tb__slug
            unique,
    description text,
    color       varchar(7),
    created_at  timestamp with time zone not null
);

alter table public.tags_tb
    owner to "postgresDev01";

create table if not exists public.authors_tb
(
    id_author     uuid                     not null
        constraint pk_authors_tb
            primary key,
    name          varchar(200)             not null,
    original_name varchar(200),
    country_code  varchar(2),
    birth_date    date,
    metadata      jsonb,
    created_at    timestamp with time zone not null
);

alter table public.authors_tb
    owner to "postgresDev01";

create index if not exists idx_authors_tb__country_code
    on public.authors_tb using hash (country_code);

create table if not exists public.users_tb
(
    id_user       uuid         not null
        constraint pk_users_tb
            primary key,
    email         varchar(100) not null,
    username      varchar(100) not null
        constraint unq_users_tb__username
            unique,
    name          varchar(50),
    surname       varchar(50),
    birthday      date         not null,
    language_code varchar(4)   not null,
    culture_code  varchar(10)  not null
);

alter table public.users_tb
    owner to "postgresDev01";

create table if not exists public.work_types_tb
(
    id_work_type uuid        not null
        constraint pk_work_types_tb
            primary key,
    code         varchar(20) not null
        constraint unq_work_types_tb__code
            unique
);

alter table public.work_types_tb
    owner to "postgresDev01";

create table if not exists public.works_tb
(
    id_work        uuid                     not null
        constraint pk_works_tb
            primary key,
    id_work_type   uuid                     not null
        constraint fk_works_tb__id_work_type_to_work_types_tb
            references public.work_types_tb,
    title          varchar(500)             not null,
    original_title varchar(500),
    synopsis       text,
    year           integer,
    metadata       jsonb,
    created_at     timestamp with time zone not null,
    updated_at     timestamp with time zone not null
);

alter table public.works_tb
    owner to "postgresDev01";

create index if not exists idx_works_tb__gin__works_title_en
    on public.works_tb using gin (to_tsvector('english'::regconfig, title::text));

create index if not exists idx_works_tb__gin__works_title_it
    on public.works_tb using gin (to_tsvector('italian'::regconfig, title::text));

create index if not exists idx_works_tb__id_work_type
    on public.works_tb using hash (id_work_type);

create table if not exists public.works_tags_tb
(
    id_work_tag uuid not null
        constraint pk_works_tags_tb
            primary key,
    id_work     uuid not null
        constraint fk_works_tags_tb__id_work_to_work_tb
            references public.works_tb,
    id_tag      uuid not null
        constraint fk_works_tags_tb__id_tag_to_tags_tb
            references public.tags_tb,
    constraint unq_works_tags_tb__id_work__id_tag
        unique (id_work, id_tag)
);

alter table public.works_tags_tb
    owner to "postgresDev01";

create index if not exists idx_works_tags__id_work
    on public.works_tags_tb using hash (id_work);

create index if not exists idx_works_tags__id_tag
    on public.works_tags_tb using hash (id_tag);

create table if not exists public.works_authors_tb
(
    id_work_author uuid        not null
        constraint pk_works_authors_tb
            primary key,
    id_work        uuid        not null
        constraint fk_works_authors_tb__id_work_to_works_tb
            references public.works_tb,
    id_author      uuid        not null
        constraint fk_works_authors_tb__id_author_to_author_tb
            references public.authors_tb,
    role           varchar(50) not null
);

alter table public.works_authors_tb
    owner to "postgresDev01";

create index if not exists idx_works_authors__id_work
    on public.works_authors_tb using hash (id_work);

create index if not exists idx_works_authors__id_author
    on public.works_authors_tb using hash (id_author);

create table if not exists public.work_release_statuses_tb
(
    id_work_release_status uuid        not null
        constraint pk_work_release_statuses_tb
            primary key,
    id_work                uuid        not null
        constraint fk_work_release_statuses_tb__id_work_to_works_tb
            references public.works_tb,
    country_code           varchar(4)  not null,
    status                 varchar(50) not null,
    started_at             date,
    completed_at           date,
    constraint unq_work_release_statuses_tb__id_work__country_code
        unique (id_work, country_code)
);

alter table public.work_release_statuses_tb
    owner to "postgresDev01";

create index if not exists idx_release_status_work_tb__id_work
    on public.work_release_statuses_tb using hash (id_work);

create table if not exists public.user_library_items_tb
(
    id_user_library_item uuid                     not null
        constraint pk_user_library_items_tb
            primary key,
    id_user              uuid                     not null
        constraint fk_user_library_items_tb__id_user_to_users_tb
            references public.users_tb,
    id_work              uuid                     not null
        constraint fk_user_library_items_tb__id_work_to_works_tb
            references public.works_tb,
    owned_volumes        text[],
    current_episode      integer,
    total_episodes       integer,
    purchase_price       numeric(10, 2),
    variant_notes        text,
    personal_rating      integer,
    notes                text,
    created_at           timestamp with time zone not null,
    updated_at           timestamp with time zone not null,
    constraint unq_user_library_items_tb__id_user__id_work
        unique (id_user, id_work)
);

alter table public.user_library_items_tb
    owner to "postgresDev01";

create index if not exists idx_user_library_items_tb__id_work
    on public.user_library_items_tb using hash (id_work);

create index if not exists idx_user_library_items_tb__id_user
    on public.user_library_items_tb using hash (id_user);

create table if not exists public.work_images_tb
(
    id            uuid                     not null
        constraint pk_work_images_tb
            primary key,
    id_work       uuid                     not null
        constraint fk_work_images_tb__id_work_to_works_tb
            references public.works_tb,
    name          varchar(150)             not null,
    storage_key   varchar(500)             not null,
    kind          varchar(50)              not null,
    display_order integer,
    width         integer,
    height        integer,
    created_at    timestamp with time zone not null
);

alter table public.work_images_tb
    owner to "postgresDev01";

create index if not exists idx_work_images_tb__id_work
    on public.work_images_tb using hash (id_work);

create table if not exists public.publishers_tb
(
    id_publisher   uuid         not null
        constraint pk_publishers_tb
            primary key,
    publisher_name varchar(100) not null,
    country_code   varchar(4)   not null
);

alter table public.publishers_tb
    owner to "postgresDev01";

create table if not exists public.publisher_editions_tb
(
    id_publisher_edition uuid        not null
        constraint pk_publisher_editions_tb
            primary key,
    id_publisher         uuid        not null
        constraint fk_publisher_editions_tb__id_publisher_to_publishers_tb
            references public.publishers_tb,
    edition_name         varchar(50) not null
);

alter table public.publisher_editions_tb
    owner to "postgresDev01";

create index if not exists idx_publisher_editions_tb__id_publisher
    on public.publisher_editions_tb using hash (id_publisher);

create table if not exists public.works_publisher_editions_tb
(
    id_work_publisher_edition uuid    not null
        constraint pk_works_publisher_editions_tb
            primary key,
    id_work                   uuid    not null
        constraint fk_works_publisher_editions_tb__id_work_to_works_tb
            references public.works_tb,
    id_publisher_edition      uuid    not null
        constraint fk_works_publisher_editions_tb__id_publisher_edition_to_publish
            references public.publisher_editions_tb,
    year_start                integer not null,
    year_end                  integer
);

alter table public.works_publisher_editions_tb
    owner to "postgresDev01";

create index if not exists idx_works_publisher_editions_tb__id_publicher_edition
    on public.works_publisher_editions_tb using hash (id_publisher_edition);

create index if not exists idx_works_publisher_editions_tb__id_work
    on public.works_publisher_editions_tb using hash (id_work);

create table if not exists public.countries_tb
(
    country_code varchar(4) not null
        constraint pk_countries_tb
            primary key,
    country_name varchar(50)
);

alter table public.countries_tb
    owner to "postgresDev01";

create index if not exists idx_countries_tb__gin__country_code_en
    on public.countries_tb using gin (to_tsvector('english'::regconfig, country_code::text));

create index if not exists idx_countries_tb__gin__country_code_it
    on public.countries_tb using gin (to_tsvector('italian'::regconfig, country_code::text));

