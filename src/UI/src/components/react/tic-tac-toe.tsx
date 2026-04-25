'use client';

import { useState } from 'react';

import styles from './tic-tac-toe.module.scss';

type ValueType = 'X' | 'O' | null;
type Squares = ValueType[];

interface SquareProps {
	value: ValueType;
	onSquareClick: () => void;
}

interface BoardProps {
	xIsNext: boolean;
	squares: Squares;
	onPlay(nextSquares: Squares): void;
}

function Square({ value, onSquareClick }: SquareProps) {
	return (
		<button type="button" className={styles.square} onClick={onSquareClick}>
			{value}
		</button>
	);
}

function Board({ xIsNext, squares, onPlay }: BoardProps) {
	function handleClick(index: number) {
		if (calculateWinner(squares) || squares[index]) {
			return;
		}

		const nextSquares = squares.slice();
		nextSquares[index] = xIsNext ? 'X' : 'O';
		onPlay(nextSquares);
	}

	const winner = calculateWinner(squares);
	const status = winner ? `Winner: ${winner}` : `Next player: ${xIsNext ? 'X' : 'O'}`;

	return (
		<>
			<div className={styles.status}>{status}</div>
			<div className={styles.boardRow}>
				<Square value={squares[0]} onSquareClick={() => handleClick(0)} />
				<Square value={squares[1]} onSquareClick={() => handleClick(1)} />
				<Square value={squares[2]} onSquareClick={() => handleClick(2)} />
			</div>
			<div className={styles.boardRow}>
				<Square value={squares[3]} onSquareClick={() => handleClick(3)} />
				<Square value={squares[4]} onSquareClick={() => handleClick(4)} />
				<Square value={squares[5]} onSquareClick={() => handleClick(5)} />
			</div>
			<div className={styles.boardRow}>
				<Square value={squares[6]} onSquareClick={() => handleClick(6)} />
				<Square value={squares[7]} onSquareClick={() => handleClick(7)} />
				<Square value={squares[8]} onSquareClick={() => handleClick(8)} />
			</div>
		</>
	);
}

export default function TicTacToe() {
	const [history, setHistory] = useState<Squares[]>([Array<ValueType>(9).fill(null)]);
	const [currentMove, setCurrentMove] = useState(0);
	const xIsNext = currentMove % 2 === 0;
	const currentSquares = history[currentMove] ?? Array<ValueType>(9).fill(null);

	function handlePlay(nextSquares: Squares) {
		const nextHistory = [...history.slice(0, currentMove + 1), nextSquares];
		setHistory(nextHistory);
		setCurrentMove(nextHistory.length - 1);
	}

	function jumpTo(nextMove: number) {
		setCurrentMove(nextMove);
	}

	const moves = history.map((_, move) => {
		const description = move > 0 ? `Go to move #${move}` : 'Go to game start';

		return (
			<li key={move}>
				<button type="button" onClick={() => jumpTo(move)}>
					{description}
				</button>
			</li>
		);
	});

	return (
		<div className={styles.game}>
			<div className={styles.gameBoard}>
				<Board xIsNext={xIsNext} squares={currentSquares} onPlay={handlePlay} />
			</div>
			<div className={styles.gameInfo}>
				<ol>{moves}</ol>
			</div>
		</div>
	);
}

function calculateWinner(squares: Squares): Exclude<ValueType, null> | null {
	const lines = [
		[0, 1, 2],
		[3, 4, 5],
		[6, 7, 8],
		[0, 3, 6],
		[1, 4, 7],
		[2, 5, 8],
		[0, 4, 8],
		[2, 4, 6],
	] as const;

	for (const [a, b, c] of lines) {
		if (squares[a] && squares[a] === squares[b] && squares[a] === squares[c]) {
			return squares[a];
		}
	}

	return null;
}
