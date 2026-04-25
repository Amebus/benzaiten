import type { ReactNode } from 'react';

import type { IListProps } from '../types';
import { isFunction, isString } from '@/basic/commons';

// Fallback renderer used when the parent component does not provide custom card content.
function defaultRenderItem<Item>(item: Item): ReactNode {
	return <pre>{JSON.stringify(item, null, 2)}</pre>;
}

// This is a generic React component: Item represents the type of each element in the list.
export default function List<Item = any>({
	items,
	children,
	renderItem,
	itemComponent: ItemComponent,
	itemKey,
	label = 'Items',
	emptyMessage = 'No data available.',
}: IListProps<Item>) {
	// Only use callable renderers; Astro slots can provide non-function children.
	const renderItemFunction = isFunction(renderItem) ? renderItem : undefined;
	const childrenFunction = isFunction(children) ? children : undefined;
	const itemComponentRenderer = ItemComponent ? (item: Item, index: number) => <ItemComponent item={item} index={index} /> : undefined;
	const itemRenderer = renderItemFunction ?? childrenFunction ?? itemComponentRenderer ?? defaultRenderItem;

	let itemKeyFunction = itemKey;
	if (isString(itemKeyFunction))
		itemKeyFunction = ((item) => (item as any)[itemKey as string]) 

	return (
		<section aria-label={label} className="list-block">
			<ul>
				{items.length === 0 ? (
					<li>
						<article>
							{emptyMessage}
						</article>
					</li>
				) : (
					items.map((item, index) => (
						<li key={itemKeyFunction(item, index)}>
							<article>{itemRenderer(item, index)}</article>
						</li>
					))
				)}
			</ul>
		</section>
	);
}