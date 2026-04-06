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
		// A semantic section groups the whole list and can be announced by assistive technologies.
		<section aria-label={label}>
			<ul>
				{items.length === 0 ? (
					// Render a semantic empty state when there are no items to display.
					<li>
						<article>
							{emptyMessage}
						</article>
					</li>
				) : (
					// Map transforms the array into one <li> for each item.
					items.map((item, index) => (
						// React uses key to track each element between renders.
						<li key={itemKeyFunction(item, index)}>
							{/* article represents the single card content for the current item. */}
							<article>{itemRenderer(item, index)}</article>
						</li>
					))
				)}
			</ul>
		</section>
	);
}