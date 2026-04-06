type TodoItem = {
	ciccia: string;
};

interface TodoCardProps {
	item: TodoItem;
	index: number;
}

export default function TodoCard({ item, index }: TodoCardProps) {
	return (
		<div>
			<h3>Task {index + 1}</h3>
			<p>{item.ciccia}</p>
		</div>
	);
}