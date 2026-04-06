import type { ComponentType, ReactNode } from 'react';

export type ListItemRenderer<Item> = (item: Item, index: number) => ReactNode;

export type ListKeyResolver<Item> = (item: Item, index: number) => string | number;

export interface IListItemComponentProps<Item> {
	item: Item;
	index: number;
}

export interface IListProps<Item = any> {
	items: Item[];
	children?: ListItemRenderer<Item>;
	renderItem?: ListItemRenderer<Item>;
	itemComponent?: ComponentType<IListItemComponentProps<Item>>;
	itemKey: ListKeyResolver<Item> | string;
	label?: string;
	emptyMessage?: ReactNode;
}
