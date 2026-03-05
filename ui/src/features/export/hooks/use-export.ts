import { exportSpecification } from "../lib/export-spec";
import { exportTestSuite } from "../lib/export-tests";
import { exportTraceability } from "../lib/export-traceability";
import { exportAllXlsx, exportAllMarkdown } from "../lib/export-all";
import type { Specification } from "@/shared/types/specification";
import type { TestSuite } from "@/shared/types/test-suite";
import type { TraceabilityMatrix } from "@/shared/types/traceability";

function downloadBlob(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

export function useExport() {
  return {
    exportSpec: (spec: Specification, format: "csv" | "xlsx") => {
      const blob = exportSpecification(spec, format);
      downloadBlob(blob, `specification.${format}`);
    },
    exportTests: (suite: TestSuite, format: "csv" | "xlsx") => {
      const blob = exportTestSuite(suite, format);
      downloadBlob(blob, `test-suite.${format}`);
    },
    exportTraceability: (
      matrix: TraceabilityMatrix,
      format: "csv" | "xlsx",
    ) => {
      const blob = exportTraceability(matrix, format);
      downloadBlob(blob, `traceability.${format}`);
    },
    exportAllData: (
      spec: Specification,
      suite: TestSuite,
      traceability: TraceabilityMatrix,
    ) => {
      const date = new Date().toISOString().slice(0, 10);
      const xlsxBlob = exportAllXlsx(spec, suite, traceability);
      downloadBlob(xlsxBlob, `spec-forge-export-${date}.xlsx`);
      const mdBlob = exportAllMarkdown(spec, suite, traceability);
      downloadBlob(mdBlob, `spec-forge-export-${date}.md`);
    },
  };
}
