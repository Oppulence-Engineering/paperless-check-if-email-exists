import * as React from "react"
import { Switch as SwitchPrimitive } from "radix-ui"

import { cn } from "#lib/utils"

function Switch({
  className,
  size = "default",
  ...props
}: React.ComponentProps<typeof SwitchPrimitive.Root> & {
  size?: "sm" | "default"
}) {
  return (
    <SwitchPrimitive.Root
      data-slot="switch"
      data-size={size}
      className={cn(
        "peer group/switch relative inline-flex shrink-0 cursor-pointer items-center rounded-full transition-colors duration-150 ease-out outline-none focus-visible:ring-2 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 data-[size=default]:h-5 data-[size=default]:w-8 data-[size=sm]:h-3.5 data-[size=sm]:w-6 data-[state=checked]:bg-primary data-[state=unchecked]:bg-foreground/15",
        className
      )}
      {...props}
    >
      <SwitchPrimitive.Thumb
        data-slot="switch-thumb"
        className={cn(
          "pointer-events-none block rounded-full bg-white shadow transition-transform duration-150 ease-out motion-reduce:transition-none group-data-[size=default]/switch:size-3.5 group-data-[size=default]/switch:data-[state=unchecked]:translate-x-1 group-data-[size=default]/switch:data-[state=checked]:translate-x-[15px] group-data-[size=sm]/switch:size-2.5 group-data-[size=sm]/switch:data-[state=unchecked]:translate-x-0.5 group-data-[size=sm]/switch:data-[state=checked]:translate-x-3"
        )}
      />
    </SwitchPrimitive.Root>
  )
}

export { Switch }
