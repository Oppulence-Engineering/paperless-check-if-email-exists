"use client";

import * as React from "react";
import { format, isValid, parse } from "date-fns";
import { CalendarClockIcon, XIcon } from "#lib/icons";

import { Button } from "#components/button";
import { Calendar } from "#components/calendar";
import { Popover, PopoverContent, PopoverTrigger } from "#components/popover";
import { cn } from "#lib/utils";

/** The value format of a native date input, so form readers do not change. */
const VALUE_FORMAT = "yyyy-MM-dd";

type DatePickerProps = {
  /** Submits the date as yyyy-MM-dd under this name, like a native date input. */
  name?: string;
  defaultValue?: string;
  className?: string;
  disabled?: boolean;
  id?: string;
  placeholder?: string;
  "aria-describedby"?: string;
};

function parseDate(value?: string) {
  if (!value) return undefined;
  const parsed = parse(value, VALUE_FORMAT, new Date());
  return isValid(parsed) ? parsed : undefined;
}

function DatePicker({
  name,
  defaultValue = "",
  className,
  disabled,
  id,
  placeholder = "Choose a date",
  "aria-describedby": ariaDescribedBy,
}: DatePickerProps) {
  const [value, setValue] = React.useState(defaultValue);
  const [open, setOpen] = React.useState(false);
  const inputRef = React.useRef<HTMLInputElement>(null);
  const selected = parseDate(value);

  // Like a native field, go back to the default value when the form resets.
  React.useEffect(() => {
    const form = inputRef.current?.form;
    if (!form) return;
    const reset = () => setValue(defaultValue);
    form.addEventListener("reset", reset);
    return () => form.removeEventListener("reset", reset);
  }, [defaultValue]);

  return (
    <div className={cn("flex min-w-0 items-center gap-2", className)} data-slot="date-picker">
      <input ref={inputRef} type="hidden" name={name} value={value} />
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button
            id={id}
            type="button"
            variant="outline"
            disabled={disabled}
            aria-describedby={ariaDescribedBy}
            className={cn(
              "h-full min-w-0 flex-1 justify-start overflow-hidden rounded-[inherit] px-3 text-left font-normal",
              !selected && "text-muted-foreground",
            )}
          >
            <CalendarClockIcon />
            <span className="truncate">{selected ? format(selected, "MMM d, yyyy") : placeholder}</span>
          </Button>
        </PopoverTrigger>
        <PopoverContent align="start" className="w-auto p-0">
          <Calendar
            mode="single"
            selected={selected}
            defaultMonth={selected}
            onSelect={(date) => {
              setValue(date ? format(date, VALUE_FORMAT) : "");
              setOpen(false);
            }}
          />
        </PopoverContent>
      </Popover>
      {selected ? (
        <Button
          type="button"
          variant="ghost"
          size="icon-sm"
          onClick={() => setValue("")}
          disabled={disabled}
          aria-label="Clear date"
        >
          <XIcon />
        </Button>
      ) : null}
    </div>
  );
}

export { DatePicker, type DatePickerProps };
