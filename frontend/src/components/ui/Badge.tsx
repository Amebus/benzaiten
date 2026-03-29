interface BadgeProps {
  children: React.ReactNode;
  color?: string;
  variant?: 'solid' | 'outline';
  className?: string;
}

export default function Badge({ children, color, variant = 'solid', className = '' }: BadgeProps) {
  if (variant === 'outline') {
    return (
      <span
        className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border ${className}`}
        style={color ? { borderColor: color, color } : undefined}
      >
        {children}
      </span>
    );
  }

  return (
    <span
      className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium text-white ${className}`}
      style={color ? { backgroundColor: color } : { backgroundColor: '#6B7280' }}
    >
      {children}
    </span>
  );
}
