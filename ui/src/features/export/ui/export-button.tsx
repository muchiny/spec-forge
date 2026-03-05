import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Download, ChevronDown, FileSpreadsheet, FileText } from "lucide-react";
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
        data-testid="export-button"
        onClick={() => setOpen(!open)}
        className="bg-surface-0 hover:bg-surface-1 text-text flex items-center gap-2 rounded-lg px-4 py-2.5 text-xs font-medium transition-all duration-200"
      >
        <Download className="h-3.5 w-3.5" />
        {t("export.button")}
        <ChevronDown
          className={cn(
            "h-3 w-3 transition-transform duration-200",
            open && "rotate-180",
          )}
        />
      </button>
      {open && (
        <div
          data-testid="export-dropdown"
          className="bg-mantle border-surface-1 animate-in zoom-in-95 fade-in duration-150 absolute right-0 z-10 mt-2 min-w-[140px] rounded-xl border p-1 shadow-xl"
        >
          <button
            data-testid="export-csv"
            onClick={() => {
              onExport("csv");
              setOpen(false);
            }}
            className="text-text hover:bg-surface-0 flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-xs transition-colors"
          >
            <FileText className="text-subtext-0 h-3.5 w-3.5" />
            {t("export.csv")}
          </button>
          <button
            data-testid="export-xlsx"
            onClick={() => {
              onExport("xlsx");
              setOpen(false);
            }}
            className="text-text hover:bg-surface-0 flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-xs transition-colors"
          >
            <FileSpreadsheet className="text-green h-3.5 w-3.5" />
            {t("export.xlsx")}
          </button>
        </div>
      )}
    </div>
  );
}
