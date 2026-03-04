import { useTranslation } from "react-i18next";
import { FolderOpen, FileUp, X } from "lucide-react";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { useFilePicker } from "../hooks/use-file-picker";

export function FilePickerPanel() {
  const { t } = useTranslation();
  const { pickFiles, pickDirectory } = useFilePicker();
  const selectedFiles = usePipelineStore((s) => s.selectedFiles);
  const removeFile = usePipelineStore((s) => s.removeFile);

  return (
    <div className="bg-mantle rounded-xl border p-4">
      <h3 className="text-text mb-3 text-sm font-semibold">{t("pipeline.inputFiles")}</h3>

      <div className="mb-3 flex gap-2">
        <button
          onClick={pickFiles}
          className="bg-surface-0 hover:bg-surface-1 text-text flex items-center gap-2 rounded-lg px-3 py-2 text-xs transition-colors"
        >
          <FileUp className="h-3.5 w-3.5" />
          {t("pipeline.addFiles")}
        </button>
        <button
          onClick={pickDirectory}
          className="bg-surface-0 hover:bg-surface-1 text-text flex items-center gap-2 rounded-lg px-3 py-2 text-xs transition-colors"
        >
          <FolderOpen className="h-3.5 w-3.5" />
          {t("pipeline.addFolder")}
        </button>
      </div>

      {selectedFiles.length === 0 ? (
        <p className="text-subtext-0 text-xs italic">{t("pipeline.noFiles")}</p>
      ) : (
        <ul className="space-y-1">
          {selectedFiles.map((file) => (
            <li
              key={file}
              className="bg-surface-0 flex items-center justify-between rounded-lg px-3 py-1.5 text-xs"
            >
              <span className="text-text truncate">{file.split("/").pop()}</span>
              <button
                onClick={() => removeFile(file)}
                className="text-subtext-0 hover:text-red ml-2 shrink-0"
              >
                <X className="h-3 w-3" />
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
