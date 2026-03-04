import { open } from "@tauri-apps/plugin-dialog";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";

export function useFilePicker() {
  const setFiles = usePipelineStore((s) => s.setFiles);
  const selectedFiles = usePipelineStore((s) => s.selectedFiles);

  const pickFiles = async () => {
    const result = await open({
      multiple: true,
      filters: [
        {
          name: "User Stories",
          extensions: ["md", "markdown", "yaml", "yml", "pdf", "docx"],
        },
      ],
    });

    if (result) {
      const paths = Array.isArray(result) ? result : [result];
      setFiles([...selectedFiles, ...paths]);
    }
  };

  const pickDirectory = async () => {
    const result = await open({ directory: true });
    if (result) {
      const path = Array.isArray(result) ? result[0] : result;
      if (path) setFiles([...selectedFiles, path]);
    }
  };

  return { pickFiles, pickDirectory };
}
