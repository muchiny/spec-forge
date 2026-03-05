import { useState, useRef, useEffect, useCallback } from "react";
import { ChevronDown } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useLocaleStore } from "@/shared/stores/use-locale-store";
import {
  supportedLocales,
  localeLabels,
  type Locale,
} from "@/shared/config/i18n";
import { cn } from "@/shared/lib/utils";

import gbFlag from "@/shared/assets/flags/gb.svg";
import frFlag from "@/shared/assets/flags/fr.svg";

const flagSrc: Record<Locale, string> = {
  en: gbFlag,
  fr: frFlag,
};

function Flag({ locale, className }: { locale: Locale; className?: string }) {
  return (
    <span
      className={cn(
        "relative inline-flex shrink-0 overflow-hidden rounded-[3px]",
        className,
      )}
    >
      <img
        src={flagSrc[locale]}
        alt=""
        className="h-full w-full object-cover"
        draggable={false}
      />
      <span className="pointer-events-none absolute inset-0 rounded-[3px] shadow-[inset_0_0_4px_rgba(0,0,0,0.15)] ring-1 ring-black/10 ring-inset" />
    </span>
  );
}

export function LanguageSwitcher() {
  const { t } = useTranslation();
  const { locale, setLocale } = useLocaleStore();
  const [open, setOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(-1);
  const ref = useRef<HTMLDivElement>(null);

  const close = useCallback(() => {
    setOpen(false);
    setHighlightedIndex(-1);
  }, []);

  useEffect(() => {
    if (!open) return;
    function handleClickOutside(e: MouseEvent) {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        close();
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [open, close]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    switch (e.key) {
      case "ArrowDown": {
        e.preventDefault();
        if (!open) {
          setOpen(true);
          setHighlightedIndex(0);
        } else {
          setHighlightedIndex((i) =>
            Math.min(i + 1, supportedLocales.length - 1),
          );
        }
        break;
      }
      case "ArrowUp": {
        e.preventDefault();
        if (!open) {
          setOpen(true);
          setHighlightedIndex(supportedLocales.length - 1);
        } else {
          setHighlightedIndex((i) => Math.max(i - 1, 0));
        }
        break;
      }
      case "Enter":
      case " ": {
        e.preventDefault();
        if (open && highlightedIndex >= 0) {
          const loc = supportedLocales[highlightedIndex];
          if (loc) {
            setLocale(loc);
            close();
          }
        } else if (!open) {
          setOpen(true);
          setHighlightedIndex(0);
        }
        break;
      }
      case "Escape":
      case "Tab": {
        if (open) {
          if (e.key === "Escape") e.preventDefault();
          close();
        }
        break;
      }
    }
  };

  return (
    <div className="relative" ref={ref}>
      <button
        type="button"
        data-testid="lang-switcher-button"
        onClick={() => {
          if (open) close();
          else {
            setOpen(true);
            setHighlightedIndex(0);
          }
        }}
        onKeyDown={handleKeyDown}
        aria-expanded={open}
        aria-haspopup="listbox"
        aria-label={t("header.switchLanguage")}
        className={cn(
          "text-subtext-0 hover:bg-surface-0 hover:text-text flex items-center gap-1.5 rounded-lg px-2 py-1.5 transition-colors",
          open && "bg-surface-0 text-text",
        )}
      >
        <Flag locale={locale} className="h-4 w-6" />
        <ChevronDown
          className={cn(
            "h-3 w-3 shrink-0 transition-transform duration-200",
            open && "rotate-180",
          )}
        />
      </button>
      {open && (
        <div
          role="listbox"
          data-testid="lang-switcher-dropdown"
          aria-label={t("header.switchLanguage")}
          className="bg-surface-0 border-surface-1 animate-in zoom-in-95 fade-in duration-150 absolute top-full right-0 z-50 mt-1 min-w-[160px] overflow-hidden rounded-xl border py-1 shadow-xl"
        >
          {supportedLocales.map((loc, index) => {
            const isActive = loc === locale;
            const isHighlighted = highlightedIndex === index;
            return (
              <button
                key={loc}
                type="button"
                role="option"
                data-testid={`lang-switcher-option-${loc}`}
                aria-selected={isActive}
                onClick={() => {
                  setLocale(loc);
                  close();
                }}
                onMouseEnter={() => setHighlightedIndex(index)}
                className={cn(
                  "flex w-full items-center gap-2.5 px-3 py-2 text-sm transition-colors",
                  isActive ? "text-blue font-medium" : "text-subtext-1",
                  isHighlighted && "bg-surface-1 text-text",
                  !isHighlighted &&
                    !isActive &&
                    "hover:bg-surface-1 hover:text-text",
                )}
              >
                <Flag locale={loc} className="h-4 w-6" />
                <span>{localeLabels[loc]}</span>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}
