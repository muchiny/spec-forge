import { useTranslation } from "react-i18next";
import { FolderOpen, FileUp, X, File } from "lucide-react";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { useFilePicker } from "../hooks/use-file-picker";

export function FilePickerPanel() {
  const { t } = useTranslation();
  const { pickFiles, pickDirectory } = useFilePicker();
  const selectedFiles = usePipelineStore((s) => s.selectedFiles);
  const removeFile = usePipelineStore((s) => s.removeFile);

  return (
    <div
      data-testid="file-picker-panel"
      className="bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-blue/20"
    >
      <h3 className="text-text mb-3 text-sm font-semibold">
        {t("pipeline.inputFiles")}
      </h3>

      <div className="mb-4 flex gap-2">
        <button
          data-testid="file-picker-add-files"
          onClick={pickFiles}
          className="bg-blue/15 text-blue hover:bg-blue/25 flex items-center gap-2 rounded-lg px-4 py-2.5 text-xs font-medium transition-colors"
        >
          <FileUp className="h-3.5 w-3.5" />
          {t("pipeline.addFiles")}
        </button>
        <button
          data-testid="file-picker-add-folder"
          onClick={pickDirectory}
          className="bg-green/15 text-green hover:bg-green/25 flex items-center gap-2 rounded-lg px-4 py-2.5 text-xs font-medium transition-colors"
        >
          <FolderOpen className="h-3.5 w-3.5" />
          {t("pipeline.addFolder")}
        </button>
      </div>

      {selectedFiles.length === 0 ? (
        <div
          data-testid="file-picker-empty"
          className="border-surface-1 flex flex-col items-center rounded-lg border border-dashed py-8 text-center"
        >
          <File className="text-surface-2 mb-2 h-8 w-8" />
          <p className="text-subtext-0 text-xs">{t("pipeline.noFiles")}</p>
        </div>
      ) : (
        <ul data-testid="file-picker-list" className="space-y-1.5">
          {selectedFiles.map((file, i) => (
            <li
              key={file}
              data-testid={`file-picker-item-${i}`}
              className="bg-surface-0 group flex items-center justify-between rounded-lg px-3 py-2 text-xs transition-all duration-200 hover:bg-surface-1"
            >
              <div className="flex items-center gap-2 truncate">
                <File className="text-blue h-3.5 w-3.5 shrink-0" />
                <span className="text-text truncate">
                  {file.split("/").pop()}
                </span>
              </div>
              <button
                data-testid={`file-picker-remove-${i}`}
                onClick={() => removeFile(file)}
                className="text-subtext-0 hover:bg-red/15 hover:text-red ml-2 shrink-0 rounded p-1 transition-colors"
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
