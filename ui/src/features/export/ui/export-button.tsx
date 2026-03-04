import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Download, ChevronDown } from "lucide-react";
import { cn } from "@/shared/lib/utils";

interface ExportButtonProps {
  onExport: (format: "csv" | "xlsx") => void;
}

export function ExportButton({ onExport }: ExportButtonProps) {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);

  return (
    <div className="relative">
      <button
        onClick={() => setOpen(!open)}
        className="bg-surface-0 hover:bg-surface-1 text-text flex items-center gap-2 rounded-lg px-3 py-2 text-xs transition-colors"
      >
        <Download className="h-3.5 w-3.5" />
        {t("export.button")}
        <ChevronDown className={cn("h-3 w-3 transition-transform", open && "rotate-180")} />
      </button>
      {open && (
        <div className="bg-mantle border-surface-1 absolute right-0 z-10 mt-1 rounded-lg border py-1 shadow-lg">
          <button
            onClick={() => { onExport("csv"); setOpen(false); }}
            className="text-text hover:bg-surface-0 w-full px-4 py-1.5 text-left text-xs"
          >
            {t("export.csv")}
          </button>
          <button
            onClick={() => { onExport("xlsx"); setOpen(false); }}
            className="text-text hover:bg-surface-0 w-full px-4 py-1.5 text-left text-xs"
          >
            {t("export.xlsx")}
          </button>
        </div>
      )}
    </div>
  );
}
