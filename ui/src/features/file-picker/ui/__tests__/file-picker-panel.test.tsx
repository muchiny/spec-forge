import { describe, it, expect, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import { renderWithProviders } from "@/test/test-utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { FilePickerPanel } from "../file-picker-panel";

describe("FilePickerPanel", () => {
  beforeEach(() => {
    usePipelineStore.setState({ selectedFiles: [] });
  });

  it("renders the panel", () => {
    renderWithProviders(<FilePickerPanel />);
    expect(screen.getByTestId("file-picker-panel")).toBeInTheDocument();
  });

  it("shows empty state when no files selected", () => {
    renderWithProviders(<FilePickerPanel />);
    expect(screen.getByTestId("file-picker-empty")).toBeInTheDocument();
  });

  it("shows add files and add folder buttons", () => {
    renderWithProviders(<FilePickerPanel />);
    expect(screen.getByTestId("file-picker-add-files")).toBeInTheDocument();
    expect(screen.getByTestId("file-picker-add-folder")).toBeInTheDocument();
  });

  it("shows file list when files are selected", () => {
    usePipelineStore.setState({
      selectedFiles: ["/path/to/story.md", "/path/to/another.yaml"],
    });
    renderWithProviders(<FilePickerPanel />);

    expect(screen.getByTestId("file-picker-list")).toBeInTheDocument();
    expect(screen.getByTestId("file-picker-item-0")).toBeInTheDocument();
    expect(screen.getByTestId("file-picker-item-1")).toBeInTheDocument();
    expect(screen.getByText("story.md")).toBeInTheDocument();
    expect(screen.getByText("another.yaml")).toBeInTheDocument();
  });

  it("shows remove button for each file", () => {
    usePipelineStore.setState({ selectedFiles: ["/a.md"] });
    renderWithProviders(<FilePickerPanel />);
    expect(screen.getByTestId("file-picker-remove-0")).toBeInTheDocument();
  });
});
