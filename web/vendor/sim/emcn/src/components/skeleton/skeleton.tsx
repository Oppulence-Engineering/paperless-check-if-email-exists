import { cn } from '../../lib/cn'

/**
 * Placeholder loading skeleton with a subtle pulse animation.
 * @param props - Standard div attributes including className for sizing.
 */
function Skeleton({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        'animate-pulse rounded-none bg-[var(--surface-active)] motion-reduce:animate-none',
        className
      )}
      {...props}
    />
  )
}

export { Skeleton }
